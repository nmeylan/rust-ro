use std::mem;
use std::sync::mpsc::SyncSender;
use std::sync::Once;
use enums::EnumWithNumberValue;
use enums::skill::{UseSkillFailure, UseSkillFailureClientSideType};
use packets::packets::{PacketZcAckTouseskill, PacketZcActionFailure, PacketZcNotifySkill2, PacketZcUseskillAck2};
use skills::skill_enums::SkillEnum;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, CharNotification, Notification};
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::map_item::MapItemSnapshot;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;
use crate::packets::packets::Packet;

static mut SERVICE_INSTANCE: Option<SkillService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct SkillService {
    client_notification_sender: SyncSender<Notification>,
    persistence_event_sender: SyncSender<PersistenceEvent>,
    configuration_service: &'static GlobalConfigService,
}

impl SkillService {
    pub fn new(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, configuration_service: &'static GlobalConfigService) -> SkillService {
        SkillService { client_notification_sender, persistence_event_sender, configuration_service }
    }
    pub fn instance() -> &'static SkillService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>, configuration_service: &'static GlobalConfigService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(SkillService::new(client_notification_sender, persistence_event_sender, configuration_service));
        });
    }

    pub fn start_use_skill(&self, character: &mut Character, target: Option<MapItemSnapshot>, skill_id: u32, skill_level: u8, tick: u128) {
        let item_snapshot = target.unwrap();
        let skill = SkillEnum::from_id(skill_id);
        let mut skill = skill.to_object(skill_level).unwrap();

        let validate_sp = skill.validate_sp(character.status.sp);
        if validate_sp.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::SpInsufficient);
            return;
        }
        let validate_hp = skill.validate_hp(character.status.hp);
        if validate_hp.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::HpInsufficient);
            return;
        }
        let maybe_ammo = character.status.ammo.map(|ammo|
            (ammo.ammo_type, character.get_item_from_inventory(ammo.inventory_index).map(|ammo_in_inventory| ammo_in_inventory.amount as u32).unwrap_or(0)));
        let validate_ammo = skill.validate_ammo(maybe_ammo);
        if validate_ammo.is_err() {
            let mut packet_zc_action_failure = PacketZcActionFailure::new(self.configuration_service.packetver());
            packet_zc_action_failure.fill_raw();
            self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_action_failure.raw_mut())))).unwrap();
            return;
        }

        let validate_weapon = skill.validate_weapon(character.status.right_hand_weapon());
        if validate_weapon.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::ThisWeapon);
            return;
        }
        let validate_zeny = skill.validate_zeny(character.status.zeny);
        if validate_zeny.is_err() {
            self.send_skill_fail_packet(character, UseSkillFailure::Money);
            return;
        }

        // TODO use char stats
        skill.update_cast_time(skill.base_cast_time());
        skill.update_after_cast_act_delay(skill.base_after_cast_act_delay());
        skill.update_after_cast_walk_delay(skill.base_after_cast_walk_delay());
        let mut packet_zc_useskill_ack2 = PacketZcUseskillAck2::new(self.configuration_service.packetver());
        packet_zc_useskill_ack2.set_target_id(item_snapshot.map_item().id());
        packet_zc_useskill_ack2.set_skid(skill_id as u16);
        packet_zc_useskill_ack2.set_property(12);  // element
        packet_zc_useskill_ack2.set_delay_time(skill.cast_time()); // cast time
        packet_zc_useskill_ack2.set_aid(character.char_id);
        packet_zc_useskill_ack2.fill_raw();
        self.client_notification_sender.send(Notification::Area(
            AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_useskill_ack2.raw_mut()))
        )).unwrap();


        let no_delay = skill.cast_time() == 0;
        character.set_skill_in_use(target.map(|target| target.map_item().id()), tick, skill, no_delay);
        if no_delay {
            self.do_use_skill(character, target, tick);
        }
    }

    fn send_skill_fail_packet(&self, character: &mut Character, cause: UseSkillFailure) {
        let mut packet_zc_ack_touseskill = PacketZcAckTouseskill::new(self.configuration_service.packetver());
        packet_zc_ack_touseskill.set_cause(cause.value() as u8);
        packet_zc_ack_touseskill.set_num(UseSkillFailureClientSideType::SkillFailed.value() as u32);
        packet_zc_ack_touseskill.set_result(false);
        packet_zc_ack_touseskill.fill_raw();
        self.client_notification_sender.send(Notification::Char(CharNotification::new(character.char_id, mem::take(packet_zc_ack_touseskill.raw_mut())))).unwrap();
    }

    pub fn do_use_skill(&self, character: &mut Character, target: Option<MapItemSnapshot>, tick: u128) {
        if tick < character.skill_in_use().start_skill_tick + character.skill_in_use().skill.cast_time() as u128 {
            return;
        }
        let skill = &character.skill_in_use().skill;
        let target_snapshot = target.unwrap();
        let mut packet_zc_notify_skill2 = PacketZcNotifySkill2::new(self.configuration_service.packetver());
        packet_zc_notify_skill2.set_skid(skill.id() as u16);
        packet_zc_notify_skill2.set_attack_mt(305); // TODO
        packet_zc_notify_skill2.set_target_id(target_snapshot.map_item().id());
        packet_zc_notify_skill2.set_damage(30); // TODO
        packet_zc_notify_skill2.set_start_time(0);
        packet_zc_notify_skill2.set_attacked_mt(480); // TODO
        packet_zc_notify_skill2.set_level(skill.level() as i16);
        packet_zc_notify_skill2.set_count(skill.hit_count().abs() as i16);
        packet_zc_notify_skill2.set_aid(character.char_id);
        packet_zc_notify_skill2.set_action(6); // TODO
        packet_zc_notify_skill2.fill_raw();

        character.update_skill_used_at_tick(tick);
        self.client_notification_sender.send(Notification::Area(
            AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_notify_skill2.raw_mut()))
        )).unwrap();
    }

    pub fn after_skill_used(&self, character: &mut Character, tick: u128) {
        let used_at = character.skill_in_use().used_at_tick.unwrap();
        if tick < used_at + character.skill_in_use().skill.after_cast_act_delay() as u128 {
            return;
        }
        character.clear_skill_in_use();
    }
}
