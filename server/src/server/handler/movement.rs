use packets::packets::{PacketCzRequestMove2, Packet, PacketZcNotifyPlayermove};
use tokio::runtime::Runtime;
use std::sync::{Arc};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use parking_lot::RwLock;
use crate::server::core::map::MapItem;
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
    let character = session_guard.character.as_ref().unwrap();
    let current_map_guard = read_lock!(character.current_map);
    let map = current_map_guard.as_ref().unwrap().clone();
    let path = path_search_client_side_algorithm(map, character.x(), character.y(), destination.x, destination.y);
    // TODO
    // * Control if cell is walkable
    // * Control player state (dead? stun?, frozen?)
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    character.set_movement_task_id(id);
    movement::move_character_task(runtime, path.clone(), session.clone(), server.clone(), id.clone());
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    let current_position = Position {x: character.x(), y: character.y(), dir: 0};
    packet_zc_notify_playermove.set_move_data(current_position.to_move_data(destination.clone()));
    packet_zc_notify_playermove.set_move_start_time(now as u32);
    packet_zc_notify_playermove.fill_raw();
    socket_send!(tcp_stream, &packet_zc_notify_playermove.raw());
    // debug_in_game_chat(&session, format!("path: {:?}", path.iter().map(|node| (node.x, node.y)).collect::<Vec<(u16, u16)>>()));
    // debug_in_game_chat(&session, format!("current_position: {:?}, destination {:?}", current_position, destination));
}