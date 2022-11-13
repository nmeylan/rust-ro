use std::fmt::Formatter;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::Error;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant};

use packets::packets::{Packet, PacketCzRequestMove, PacketCzRequestMove2, PacketZcNpcackMapmove};

use crate::server::core::character::{Character, MovementTask};
use crate::server::core::event::{CharacterChangeMap, Event};
use crate::server::core::map::{Map, MAP_EXT, RANDOM_CELL};
use crate::server::core::path::PathNode;
use crate::server::core::position::Position;
use crate::server::core::session::Session;
use crate::server::server::Server;
use crate::util::string::StringUtil;

#[derive(Clone, Copy)]
pub struct Movement {
    position: Position,
    is_diagonal: bool,
    move_at: u128,
}

impl Movement {
    pub fn move_at(&self) -> u128 {
        self.move_at
    }
    pub fn set_move_at(&mut self, tick: u128) {
        self.move_at = tick
    }
    pub fn is_diagonal(&self) -> bool {
        self.is_diagonal
    }
    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn from_path(path: Vec<PathNode>, start_at: u128, start_position: &Position) -> Vec<Movement> {
        let mut movements = vec![];
        let mut current_position = start_position.clone();
        for path_node in path.iter() {
            let position = Position { x: path_node.x, y: path_node.y, dir: 0 };
            movements.push(Movement {
                position,
                is_diagonal: path_node.is_diagonal,
                move_at: start_at, // Will be re-set in game loop to take current player speed
            });
            current_position = position;
        }
        movements.reverse();
        movements
    }

    pub fn delay(speed: u16, is_diagonal: bool) -> u128 {
        if is_diagonal {
            (speed as f64 / 0.6) as u128
        } else {
            speed as u128
        }
    }
}


pub fn change_map_packet(destination_map: &String, x: u16, y: u16, session: Arc<Session>, server: &Server) {
    let char_id = session.char_id();
    server.add_to_next_tick(Event::CharacterClearFov(char_id));
    server.add_to_next_tick(Event::CharacterRemoveFromMap(char_id));

    let map_name: String = Map::name_without_ext(destination_map);
    debug!("Char enter on map {}", map_name);
    let map_ref = server.maps.get(&map_name).unwrap();
    let map_instance = map_ref.player_join_map(server);
    let (x, y) = if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
        let walkable_cell = Map::find_random_walkable_cell(&map_instance.cells, map_instance.x_size);
        (walkable_cell.0, walkable_cell.1)
    } else {
        (x, y)
    };

    server.add_to_tick(Event::CharacterChangeMap(CharacterChangeMap {
        char_id: session.char_id.unwrap(),
        new_map_name: destination_map.clone(),
        new_instance_id: map_instance.id,
        new_position: Some(Position { x, y, dir: 3 }),
        old_map_name: None,
        old_position: None,
    }), 2);
}