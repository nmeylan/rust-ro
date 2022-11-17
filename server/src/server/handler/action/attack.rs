



use packets::packets::{Packet, PacketCzRequestAct2, PacketZcNotifyAct3};
use crate::server::Server;
use crate::server::core::request::Request;

use crate::server::enums::action::ActionType;

pub fn handle_attack(_server: &Server, context: Request) {
    let packet_cz_request_act2 = cast!(context.packet(), PacketCzRequestAct2);
    let session = context.session();
    let _char_id = session.char_id();
    // let character = server.get_character_unsafe(char_id);
    // let map_ref = character.current_map.as_ref().unwrap().clone();
    // let mobs_guard = read_lock!(map_ref.mobs);
    // let mob_found = mobs_guard.get(&packet_cz_request_act2.target_gid);
    // if mob_found.is_some() {
    //     info!("Hit {}!", mob_found.unwrap().name);
    // }
    let mut packet_zc_notify_act3 = PacketZcNotifyAct3::new();
    packet_zc_notify_act3.set_target_gid(packet_cz_request_act2.target_gid);
    packet_zc_notify_act3.set_action(ActionType::AttackMultiple.value());
    packet_zc_notify_act3.set_gid(session.account_id);
    packet_zc_notify_act3.set_attack_mt(498);
    packet_zc_notify_act3.set_attacked_mt(1);
    packet_zc_notify_act3.set_damage(2);
    packet_zc_notify_act3.set_count(1);
    packet_zc_notify_act3.fill_raw();
    socket_send!(context, packet_zc_notify_act3);
}