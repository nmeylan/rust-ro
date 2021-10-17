use crate::packets::packets::{PacketCzRequestMove2, Packet, PacketZcNotifyPlayermove};
use tokio::runtime::Runtime;
use std::sync::{Arc, RwLock};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use crate::server::core::map::Map;
use crate::server::core::movement;
use crate::server::core::movement::Position;
use crate::server::core::path::path_search_client_side_algorithm;
use crate::server::server::Server;

pub fn handle_char_move(server: Arc<Server>, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session_id: u32) {
    let move_packet = cast!(packet, PacketCzRequestMove2);
    let sessions_guard = read_lock!(server.sessions);
    let session = sessions_guard.get(&session_id).unwrap();
    let session_guard = read_lock!(session);
    let destination = Position::from_move_packet(move_packet);
    let mut character_session_guard = session_guard.character.as_ref().unwrap().lock().unwrap();
    let map_name: String = Map::name_without_ext(character_session_guard.get_current_map_name());
    let maps_guard = read_lock!(server.maps);
    let map = maps_guard.get(&map_name[..]).unwrap();
    let current_position = character_session_guard.current_position.clone();

    let path = path_search_client_side_algorithm(map, &current_position, &destination);
    // TODO
    // * Control if cell is walkable
    // * Control player state (dead? stun?, frozen?)
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    character_session_guard.set_movement_task_id(id);
    std::mem::drop(character_session_guard);
    movement::move_character(runtime, path.clone(), session.clone(), map_name, server.clone(), id.clone());
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    packet_zc_notify_playermove.set_move_data(current_position.to_move_data(destination.clone()));
    packet_zc_notify_playermove.set_move_start_time(now as u32);
    packet_zc_notify_playermove.fill_raw();
    socket_send!(tcp_stream, &packet_zc_notify_playermove.raw());
    // debug_in_game_chat(&session, format!("path: {:?}", path.iter().map(|node| (node.x, node.y)).collect::<Vec<(u16, u16)>>()));
    // debug_in_game_chat(&session, format!("current_position: {:?}, destination {:?}", current_position, destination));
}