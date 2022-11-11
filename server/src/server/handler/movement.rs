use std::io::Write;
use std::net::TcpStream;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;
use tokio::runtime::Runtime;

use packets::packets::{Packet, PacketCzRequestMove, PacketCzRequestMove2, PacketZcNotifyPlayermove};

use crate::server::core::character_movement;
use crate::server::core::character_movement::Movement;
use crate::server::core::event::CharacterMovement;
use crate::server::core::event::Event::{CharacterClearMove, CharacterMove};
use crate::server::core::position::Position;
use crate::server::core::map::MapItem;
use crate::server::core::path::path_search_client_side_algorithm;
use crate::server::core::request::Request;
use crate::server::server::Server;
use crate::util::tick::{get_current_time, get_tick};


pub fn handle_char_move(server: &Server, context: Request) {
    let destination = if context.packet().as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
        let move_packet = cast!(context.packet(), PacketCzRequestMove2);
        Position::from_move2_packet(move_packet)
    } else {
        let move_packet = cast!(context.packet(), PacketCzRequestMove);
        Position::from_move_packet(move_packet)
    };
    debug!("Request move to {}", destination);
    let character = server.get_character_from_context_unsafe(&context);
    let map_instance = server.get_map_instance_from_character(character.deref()).unwrap();
    // server.add_to_next_movement_tick(CharacterClearMove(character.char_id));
    let mut current_position = Position { x: character.x(), y: character.y(), dir: 0 };
    if character.is_moving() {
        if let Some(previous_movement) = character.peek_movement() {
            // if get_current_time() > previous_movement.move_at() && ((get_current_time() - previous_movement.move_at()) < (character.status.speed/2) as u128) {
                current_position = *previous_movement.position()
            // }
        }
    }
    // let maybe_previous_movement = character.peek_movement().cloned();

    let path = path_search_client_side_algorithm(map_instance, current_position.x(), current_position.y(), destination.x, destination.y);
    let start_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut path = Movement::from_path(path, start_at, &current_position);
    // if let Some(previous_movement) = maybe_previous_movement {
    //     path.push(previous_movement);
    // }
    server.add_to_next_movement_tick(CharacterMove(CharacterMovement {
        char_id: character.char_id,
        destination,
        path,
        start_at,
        current_position,
    }));
    // debug_in_game_chat(&session, format!("path: {:?}", path.iter().map(|node| (node.x, node.y)).collect::<Vec<(u16, u16)>>()));
    // debug_in_game_chat(&session, format!("current_position: {:?}, destination {:?}", current_position, destination));
}