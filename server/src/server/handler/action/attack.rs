use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use packets::packets::{Packet, PacketCzRequestAct2, PacketZcNotifyAct3};
use crate::server::enums::action::ActionType;
use crate::server::server::Server;
use std::io::Write;

pub fn handle_attack(server: Arc<Server>, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session_id: u32) {
    let packet_cz_request_act2 = cast!(packet, PacketCzRequestAct2);
    let sessions_guard = read_lock!(server.sessions);
    let session = read_session!(sessions_guard, &session_id);
    let mut character = session.character.as_ref().unwrap().lock().unwrap();
    let map_ref = character.current_map.as_ref().unwrap().clone();
    let map_guard = read_lock!(map_ref);
    let mob_found = map_guard.mobs.get(&packet_cz_request_act2.target_gid);
    if mob_found.is_some() {
        let mob = mob_found.unwrap();
        let mob_guard = read_lock!(mob);
        info!("Hit {}!", mob_guard.name);
    }
    let mut packet_zc_notify_act3 = PacketZcNotifyAct3::new();
    packet_zc_notify_act3.set_target_gid(packet_cz_request_act2.target_gid);
    packet_zc_notify_act3.set_action(ActionType::ATTACK_MULTIPLE.value());
    packet_zc_notify_act3.set_gid(session_id);
    packet_zc_notify_act3.set_attack_mt(498);
    packet_zc_notify_act3.set_attacked_mt(1);
    packet_zc_notify_act3.set_damage(2);
    packet_zc_notify_act3.set_count(1);
    packet_zc_notify_act3.fill_raw();
    socket_send!(tcp_stream, packet_zc_notify_act3.raw());
}