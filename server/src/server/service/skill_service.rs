use std::mem;
use std::sync::mpsc::SyncSender;
use std::sync::Once;
use models::enums::EnumWithNumberValue;
use models::enums::skill::{SkillType, UseSkillFailure, UseSkillFailureClientSideType};
use models::item::NormalInventoryItem;
use packets::packets::{PacketZcAckTouseskill, PacketZcActionFailure, PacketZcNotifySkill2, PacketZcUseskillAck2};
use skills::OffensiveSkill;
use models::enums::skill_enums::SkillEnum;
use models::status::{StatusSnapshot};
use models::status_bonus::StatusBonusSource::Skill;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::map_item::MapItemSnapshot;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::packets::packets::Packet;
use crate::server::model::action::{Damage, SkillCasted, SkillUsed};
use crate::server::service::battle_service::BattleService;
use crate::server::service::status_service::StatusService;

static mut SERVICE_INSTANCE: Option<SkillService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct SkillService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    configuration_service: &'static GlobalConfigService,
    battle_service: BattleService,
    status_service: &'static StatusService,
    force_no_delay: bool,
}


impl SkillService {
    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, battle_service: BattleService, status_service: &'static StatusService, configuration_service: &'static GlobalConfigService) -> SkillService {
        SkillService { client_notification_sender, persistence_event_sender, configuration_service, battle_service, status_service, force_no_delay: false }
    }
    pub fn instance() -> &'static SkillService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn force_no_delay(mut self) -> Self {
        self.force_no_delay = true;
        self
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, battle_service: BattleService, status_service: &'static StatusService, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(SkillService::new(client_notification_sender, persistence_event_sender, battle_service, status_service, configuration_service));
        });
    }

    pub fn start_use_skill(&self, character: &mut Character, target: Option<MapItemSnapshot>, source_status: &StatusSnapshot, target_status: Option<&StatusSnapshot>, skill_id: u32, skill_level: u8, tick: u128) -> SkillCasted {
        if target.is_none() || target_status.is_none() {
            return SkillCasted::invalid();
        }
        let target_snapshot = target.unwrap();
        let skill = SkillEnum::from_id(skill_id);
        let mut skill = skills::skill_enums::to_object(skill, skill_level).unwrap();

        let validate_sp = skill.validate_sp(source_status);
        if validate_sp.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::SpInsufficient);
            return SkillCasted::invalid();
        }
        let validate_hp = skill.validate_hp(source_status);
        if validate_hp.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::HpInsufficient);
            return SkillCasted::invalid();
        }
        let maybe_ammo = character.status.ammo.map(|ammo|
            (ammo.ammo_type, character.get_item_from_inventory(ammo.inventory_index).map(|ammo_in_inventory| ammo_in_inventory.amount as u32).unwrap_or(0)));
        let validate_ammo = skill.validate_ammo(maybe_ammo);
        if validate_ammo.is_err() {
            let mut packet_zc_action_failure = PacketZcActionFailure::new(self.configuration_service.packetver());
            packet_zc_action_failure.fill_raw();
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_action_failure.raw_mut())))).unwrap();
            return SkillCasted::invalid();
        }

        let validate_weapon = skill.validate_weapon(source_status);
        if validate_weapon.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::ThisWeapon);
            return SkillCasted::invalid();
        }
        let validate_zeny = skill.validate_zeny(source_status);
        if validate_zeny.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::Money);
            return SkillCasted::invalid();
        }

        let validate_items = skill.validate_item(&character.inventory_normal().iter().map(|(_, i)| i.to_normal_item()).collect::<Vec<NormalInventoryItem>>());
        if validate_items.is_err() {
            self.send_skill_fail_packet(character, validate_items.err().unwrap());
            return SkillCasted::invalid();
        }

        // TODO use char stats
        skill.update_cast_time(skill.base_cast_time());
        skill.update_after_cast_act_delay(skill.base_after_cast_act_delay());
        skill.update_after_cast_walk_delay(skill.base_after_cast_walk_delay());
        let mut packet_zc_useskill_ack2 = PacketZcUseskillAck2::new(self.configuration_service.packetver());
        packet_zc_useskill_ack2.set_target_id(target_snapshot.map_item().id());
        packet_zc_useskill_ack2.set_skid(skill_id as u16);
        packet_zc_useskill_ack2.set_property(12);  // element
        packet_zc_useskill_ack2.set_delay_time(skill.cast_time()); // cast time
        packet_zc_useskill_ack2.set_aid(character.char_id);
        packet_zc_useskill_ack2.fill_raw();
        self.client_notification_sender.send(Notification::Area(
            AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_useskill_ack2.raw_mut()))
        )).unwrap();


        let no_delay = self.force_no_delay || skill.cast_time() == 0;
        character.set_skill_in_use(target.map(|target| target.map_item().id()), tick, skill);
        let mut skill_usage_response = SkillCasted::valid();
        if no_delay {
            skill_usage_response = SkillCasted::no_delay();
        }

        skill_usage_response
    }

    pub fn do_use_skill(&self, character: &mut Character, target: Option<MapItemSnapshot>, source_status: &StatusSnapshot, target_status: Option<&StatusSnapshot>, tick: u128) -> Option<SkillUsed> {
        if target.is_none() || target_status.is_none() {
            return None;
        }

        if !self.force_no_delay && tick < character.skill_in_use().start_skill_tick + character.skill_in_use().skill.cast_time() as u128 {
            return None;
        }
        let skill = &character.skill_in_use().skill;
        let skill_type = skill.skill_type();
        let mut damage: i32 = 0;
        let mut packets: Vec<u8> = vec![];
        let mut attack_motion: u128 = 0;
        let mut target_id = character.char_id;
        let mut bonuses = Default::default();
        match skill.skill_type() {
            SkillType::Offensive => {
                let skill = skill.as_offensive_skill().unwrap();
                damage = self.calculate_damage(source_status, target_status.as_ref().unwrap(), skill);
                let mut packet_zc_notify_skill2 = PacketZcNotifySkill2::new(self.configuration_service.packetver());
                packet_zc_notify_skill2.set_skid(skill.id() as u16);
                target_id = target.as_ref().unwrap().map_item().id();
                packet_zc_notify_skill2.set_target_id(target_id);
                packet_zc_notify_skill2.set_damage(damage);
                packet_zc_notify_skill2.set_start_time(0);

                attack_motion = self.status_service.attack_motion(source_status) as u128;
                packet_zc_notify_skill2.set_attack_mt(attack_motion as i32);
                packet_zc_notify_skill2.set_attacked_mt(attack_motion as i32);
                packet_zc_notify_skill2.set_level(skill.level() as i16);

                packet_zc_notify_skill2.set_count(skill.hit_count().abs() as i16);
                packet_zc_notify_skill2.set_aid(character.char_id);
                packet_zc_notify_skill2.set_action(6); // TODO
                packet_zc_notify_skill2.fill_raw();
                packets = mem::take(packet_zc_notify_skill2.raw_mut());

            }
            SkillType::Interactive => {}
            SkillType::Performance => {}
            SkillType::Support => {
                let skill = skill.as_supportive_skill().unwrap();
                bonuses = skill.bonuses_to_target(tick);
            }
            SkillType::Passive => {}
        }
        character.update_skill_used_at_tick(tick);
        if packets.len() > 0 {
            self.client_notification_sender.send(Notification::Area(
                AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, packets)
            )).unwrap();
        }

        Some(
            SkillUsed {
                skill_type,
                source_id: character.char_id,
                target_id,
                damage_to_target: damage,
                damage_to_self: 0,
                effects: vec![],
                bonuses,
                attacked_at: tick + attack_motion,
            }
        )
    }

    pub fn after_skill_used(&self, character: &mut Character, tick: u128) {
        let used_at = character.skill_in_use().used_at_tick.unwrap();
        if tick < used_at + character.skill_in_use().skill.after_cast_act_delay() as u128 {
            return;
        }
        character.clear_skill_in_use();
    }

    fn send_skill_fail_packet(&self, character: &mut Character, cause: UseSkillFailure) {
        let mut packet_zc_ack_touseskill = PacketZcAckTouseskill::new(self.configuration_service.packetver());
        packet_zc_ack_touseskill.set_cause(cause.value() as u8);
        packet_zc_ack_touseskill.set_num(UseSkillFailureClientSideType::SkillFailed.value() as u32);
        packet_zc_ack_touseskill.set_result(false);
        packet_zc_ack_touseskill.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_ack_touseskill.raw_mut())))).unwrap();
    }

    pub fn calculate_damage(&self, source_status: &StatusSnapshot, target_status: &StatusSnapshot, skill: &dyn OffensiveSkill) -> i32 {
        self.battle_service.calculate_damage(source_status, target_status, Some(skill))
    }

}
