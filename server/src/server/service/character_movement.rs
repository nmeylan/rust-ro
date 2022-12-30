use crate::server::events::game_event::{CharacterChangeMap, CharacterRemoveFromMap, GameEvent};
use crate::server::core::map::{Map, RANDOM_CELL};
use crate::server::core::position::Position;
use crate::server::Server;


pub fn change_map_packet(destination_map: &str, x: u16, y: u16, char_id: u32, server: &Server) {
    server.add_to_next_tick(GameEvent::CharacterClearFov(char_id));
    let character_ref = server.get_character_unsafe(char_id);
    server.add_to_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap{char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance()}), 0);
    drop(character_ref);

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
        char_id,
        new_map_name: destination_map.to_owned(),
        new_instance_id: map_instance.id,
        new_position: Some(Position { x, y, dir: 3 }),
    }), 2);
}