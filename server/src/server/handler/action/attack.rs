use std::net::TcpStream;
use std::sync::{Arc};
use tokio::runtime::Runtime;
use packets::packets::{Packet, PacketCzRequestAct2, PacketZcNotifyAct3};
use crate::server::enums::action::ActionType;
use crate::server::server::Server;
use std::io::Write;
use parking_lot::RwLock;

pub fn handle_attack(server: Arc<Server>, packet: &mut dyn Packet, _runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session_id: u32) {
    let packet_cz_request_act2 = cast!(packet, PacketCzRequestAct2);
    let sessions_guard = read_lock!(server.sessions);
    let session = sessions_guard.get(&session_id).unwrap();
    let character = session.get_character();
    let current_map_guard = read_lock!(character.current_map);
    let map_ref = current_map_guard.as_ref().unwrap().clone();
    let mobs_guard = read_lock!(map_ref.mobs);
    let mob_found = mobs_guard.get(&packet_cz_request_act2.target_gid);
    if mob_found.is_some() {
        info!("Hit {}!", mob_found.unwrap().name);
    }
    let mut packet_zc_notify_act3 = PacketZcNotifyAct3::new();
    packet_zc_notify_act3.set_target_gid(packet_cz_request_act2.target_gid);
    packet_zc_notify_act3.set_action(ActionType::AttackMultiple.value());
    packet_zc_notify_act3.set_gid(session_id);
    packet_zc_notify_act3.set_attack_mt(498);
    packet_zc_notify_act3.set_attacked_mt(1);
    packet_zc_notify_act3.set_damage(2);
    packet_zc_notify_act3.set_count(1);
    packet_zc_notify_act3.fill_raw();
    socket_send!(tcp_stream, packet_zc_notify_act3.raw());
}