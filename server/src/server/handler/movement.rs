

use std::ops::Deref;
use std::thread::sleep;

use std::time::{Duration, SystemTime, UNIX_EPOCH};




use packets::packets::{PacketCzRequestMove, PacketCzRequestMove2};


use crate::server::core::movement::Movement;
use crate::server::events::game_event::CharacterMovement;
use crate::server::events::game_event::GameEvent::{CharacterMove};
use crate::server::core::position::Position;
use crate::server::core::path::path_search_client_side_algorithm;
use crate::server::core::request::Request;
use crate::server::Server;



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
    let map_instance = server.get_map_instance_from_character(character.deref()).unwrap_or_else(|| panic!("Expected to find map instance for character but didn't succeed"));
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
    if character.is_attacking() {
        let attack = character.attack();
        if attack.last_attack_tick as i128 + attack.last_attack_motion as i128 - 40 > start_at as i128 {
            let delay = ((attack.last_attack_tick + attack.last_attack_motion as u128) - 40 - start_at) as u64;
            info!("Character is attacking, move will be delayed by {}ms", delay);
            sleep(Duration::from_millis(delay));
        }
    }
    let start_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let path = Movement::from_path(path, start_at);
    // if let Some(previous_movement) = maybe_previous_movement {
    //     path.push(previous_movement);
    // }
    server.add_to_next_movement_tick(CharacterMove(CharacterMovement {
        char_id: character.char_id,
        destination,
        path,
        start_at,
        current_position,
        cancel_attack: true
    }));
    // debug_in_game_chat(&session, format!("path: {:?}", path.iter().map(|node| (node.x, node.y)).collect::<Vec<(u16, u16)>>()));
    // debug_in_game_chat(&session, format!("current_position: {:?}, destination {:?}", current_position, destination));
}