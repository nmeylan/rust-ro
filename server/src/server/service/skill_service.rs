use std::mem;
use std::sync::mpsc::SyncSender;
use std::sync::Once;
use packets::packets::{PacketZcNotifySkill2, PacketZcUseskillAck2};
use skills::skill_enums::SkillEnum;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
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

    pub fn start_use_skill(&self, character: &mut Character, target: Option<MapItemSnapshot>, skill_id: u32, skill_level: u8) {
        let item_snapshot = target.unwrap();
        let skill = SkillEnum::from_id(skill_id);
        let skill = skill.to_object(skill_level).unwrap();
// PacketZcUseskillAck2
        // PacketZcNotifySkill2
        let mut packet_zc_useskill_ack2 = PacketZcUseskillAck2::new(self.configuration_service.packetver());
        packet_zc_useskill_ack2.set_target_id(item_snapshot.map_item().id());
        packet_zc_useskill_ack2.set_skid(skill_id as u16);
        packet_zc_useskill_ack2.set_property(12);  // element
        packet_zc_useskill_ack2.set_delay_time(skill.cast_delay()); // cast time
        packet_zc_useskill_ack2.set_aid(character.char_id);
        packet_zc_useskill_ack2.fill_raw();
        self.client_notification_sender.send(Notification::Area(
            (AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_useskill_ack2.raw_mut())))
        )).unwrap();

    }

    pub fn do_use_skill(&self, character: &mut Character, target: Option<MapItemSnapshot>, skill_id: u32, skill_level: u8) {
        let item_snapshot = target.unwrap();
        let mut packet_zc_notify_skill2 = PacketZcNotifySkill2::new(self.configuration_service.packetver());
        packet_zc_notify_skill2.set_skid(skill_id as u16);
        packet_zc_notify_skill2.set_attack_mt(305);
        packet_zc_notify_skill2.set_target_id(item_snapshot.map_item().id());
        packet_zc_notify_skill2.set_damage(30);
        packet_zc_notify_skill2.set_start_time(0);
        packet_zc_notify_skill2.set_attacked_mt(480);
        packet_zc_notify_skill2.set_level(10);
        packet_zc_notify_skill2.set_count(10);
        packet_zc_notify_skill2.set_aid(character.char_id);
        packet_zc_notify_skill2.set_action(6); //
        packet_zc_notify_skill2.fill_raw();

        self.client_notification_sender.send(Notification::Area(
            (AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_notify_skill2.raw_mut())))
        )).unwrap();
    }
}
