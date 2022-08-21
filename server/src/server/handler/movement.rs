use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use rand::Rng;
use tokio::runtime::Runtime;

use packets::packets::{Packet, PacketCzRequestMove, PacketCzRequestMove2, PacketZcNotifyPlayermove};

use crate::server::core::character_movement;
use crate::server::core::character_movement::Position;
use crate::server::core::map::MapItem;
use crate::server::core::path::path_search_client_side_algorithm;
use crate::server::core::request::RequestContext;
use crate::server::server::Server;
use crate::util::tick::get_tick;


pub fn handle_char_move(server: Arc<Server>, context: RequestContext) {
    let destination = if context.packet().as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
        let move_packet = cast!(context.packet(), PacketCzRequestMove2);
        Position::from_move2_packet(move_packet)
    } else {
        let move_packet = cast!(context.packet(), PacketCzRequestMove);
        Position::from_move_packet(move_packet)
    };
    debug!("Request move to {}", destination);
    let character = context.session().character();
    let current_map_guard = read_lock!(character.current_map);
    let map = current_map_guard.as_ref().unwrap().clone();
    drop(current_map_guard);
    let path = path_search_client_side_algorithm(map, character.x(), character.y(), destination.x, destination.y);
    // TODO
    // * Control if cell is walkable
    // * Control player state (dead? stun?, frozen?)
    let id = rand::thread_rng().gen::<u64>();
    character.add_movement_task_id(id);
    character_movement::move_character_task(&context.runtime(), path, context.session().clone(), server, id);
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    let current_position = Position { x: character.x(), y: character.y(), dir: 0 };
    packet_zc_notify_playermove.set_move_data(current_position.to_move_data(&destination));
    packet_zc_notify_playermove.set_move_start_time(get_tick());
    packet_zc_notify_playermove.fill_raw();
    socket_send!(context.socket(), packet_zc_notify_playermove.raw());
    // debug_in_game_chat(&session, format!("path: {:?}", path.iter().map(|node| (node.x, node.y)).collect::<Vec<(u16, u16)>>()));
    // debug_in_game_chat(&session, format!("current_position: {:?}, destination {:?}", current_position, destination));
}