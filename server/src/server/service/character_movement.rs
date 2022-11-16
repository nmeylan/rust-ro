use std::fmt::Formatter;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::Error;
use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant};

use packets::packets::{Packet, PacketCzRequestMove, PacketCzRequestMove2, PacketZcNpcackMapmove};

use crate::server::state::character::{Character, MovementTask};
use crate::server::events::game_event::{CharacterChangeMap, GameEvent};
use crate::server::core::map::{Map, MAP_EXT, RANDOM_CELL};
use crate::server::core::path::PathNode;
use crate::server::core::position::Position;
use crate::server::core::session::Session;
use crate::server::server::Server;
use crate::util::string::StringUtil;


pub fn change_map_packet(destination_map: &String, x: u16, y: u16, session: Arc<Session>, server: &Server) {
    let char_id = session.char_id();
    server.add_to_next_tick(GameEvent::CharacterClearFov(char_id));
    server.add_to_next_tick(GameEvent::CharacterRemoveFromMap(char_id));

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

    server.add_to_tick(GameEvent::CharacterChangeMap(CharacterChangeMap {
        char_id: session.char_id.unwrap(),
        new_map_name: destination_map.clone(),
        new_instance_id: map_instance.id,
        new_position: Some(Position { x, y, dir: 3 }),
        old_map_name: None,
        old_position: None,
    }), 2);
}