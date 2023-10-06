use std::mem;
use crate::{Notification, Server};
use crate::server::model::request::Request;

use packets::packets::{Packet, PacketCzUseSkill, PacketZcNotifySkill2, PacketZcUseskillAck2};
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType};

pub fn handle_use_skill(server: &Server, context: Request) {
    let packet_cz_use_skill = cast!(context.packet(), PacketCzUseSkill);
// PacketZcUseskillAck2
    // PacketZcNotifySkill2
    let mut packet_zc_useskill_ack2 = PacketZcUseskillAck2::new(context.packet_ver());
    packet_zc_useskill_ack2.set_target_id(packet_cz_use_skill.target_id);
    packet_zc_useskill_ack2.set_skid(packet_cz_use_skill.skid);
    packet_zc_useskill_ack2.set_property(12);
    packet_zc_useskill_ack2.set_aid(context.session().account_id);
    packet_zc_useskill_ack2.fill_raw();
    let character = server.state().get_character(context.session().char_id.unwrap()).unwrap();
    server.client_notification_sender().send(Notification::Area(
        (AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_useskill_ack2.raw_mut())))
    )).unwrap();

    let mut packet_zc_notify_skill2 = PacketZcNotifySkill2::new(context.packet_ver());
    packet_zc_notify_skill2.set_skid(packet_cz_use_skill.skid);
    packet_zc_notify_skill2.set_attack_mt(305);
    packet_zc_notify_skill2.set_target_id(packet_cz_use_skill.target_id);
    packet_zc_notify_skill2.set_damage(30);
    packet_zc_notify_skill2.set_attacked_mt(480);
    packet_zc_notify_skill2.set_level(10);
    packet_zc_notify_skill2.set_count(1);
    packet_zc_notify_skill2.set_aid(context.session().account_id);
    packet_zc_notify_skill2.set_action(6);
    packet_zc_notify_skill2.fill_raw();

    let character = server.state().get_character(context.session().char_id.unwrap()).unwrap();
    server.client_notification_sender().send(Notification::Area(
        (AreaNotification::new(character.current_map_name().clone(), character.current_map_instance(), AreaNotificationRangeType::Fov { x: character.x, y: character.y, exclude_id: None }, mem::take(packet_zc_notify_skill2.raw_mut())))
    )).unwrap();
}