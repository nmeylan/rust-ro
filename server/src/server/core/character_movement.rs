use std::fmt::Formatter;
use std::sync::Arc;
use std::thread::sleep;
use sqlx::Error;

use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tokio::time::Duration;

use packets::packets::{Packet, PacketCzRequestMove, PacketCzRequestMove2, PacketZcNpcackMapmove};
use crate::server::core::character::{Character, MovementTask};
use crate::server::core::event::{CharacterChangeMap, CharacterUpdatePosition, Event};

use crate::server::core::map::{Map, MAP_EXT, MapItem, RANDOM_CELL};
use crate::server::core::path::PathNode;
use crate::server::core::position::Position;
use crate::server::core::session::Session;
use crate::server::server::Server;
use crate::util::string::StringUtil;

// TODO find a formula
fn extra_delay(speed: u16) -> i16 {
    return 20;
    if speed < 100 {
        -10
    } else if speed > 100 {
        60
    } else {
        20
    }
}

// If movement not smooth:
// teleport in front -> server movement faster than client movement
// teleport back -> server movement slower than client movement
pub fn move_character_task(runtime: &Runtime, path: Vec<PathNode>, session: Arc<Session>, server: Arc<Server>, current_movement_task_id: MovementTask) -> JoinHandle<()> {
    todo!("move_character_task");
    // let handle = runtime.spawn(async move {
    //     let mut has_been_canceled = false;
    //     {
    //         for path_node in path {
    //             let delay: u64;
    //             {
    //                 let char_id = session.char_id();
    //                 let character = server.get_character_unsafe(char_id);
    //                 let mut movement_tasks_guard = character.movement_tasks.lock().unwrap();
    //                 if !movement_tasks_guard.contains(&current_movement_task_id) {
    //                     has_been_canceled = true;
    //                     break;
    //                 }
    //
    //                 if character.x() != path_node.x && character.y() != path_node.y { // diagonal movement
    //                     delay = (character.status.speed as f64 / 0.6) as u64;
    //                 } else {
    //                     delay = character.status.speed as u64;
    //                 }
    //                 // info!("walk delay {}", delay);
    //                 debug!("[{:?} - {}] [{} paralell tasks] movement update_position", std::thread::current().id(), current_movement_task_id, movement_tasks_guard.len()) ;
    //                 character.update_position(path_node.x, path_node.y);
    //                 {
    //                     let current_map_guard = read_lock!(character.current_map);
    //                     let map_ref = current_map_guard.as_ref().unwrap();
    //                     if map_ref.is_warp_cell(path_node.x, path_node.y) {
    //                         let warp = map_ref.get_warp_at(path_node.x, path_node.y).unwrap();
    //                         drop(current_map_guard);
    //                         change_map(&warp.dest_map_name, warp.to_x, warp.to_y, session.clone(), server.clone(), None);
    //                         debug!("[{:?} - {}] Warp break", std::thread::current().id(), current_movement_task_id);
    //                         movement_tasks_guard.clear();
    //                         break;
    //                     }
    //                 }
    //
    //                 character.load_units_in_fov(&session);
    //             }
    //             sleep(Duration::from_millis(delay));
    //         }
    //     }
    //     if !has_been_canceled {
    //         {
    //             let session_clone = session.clone();
    //             let character = server.get_character_unsafe(session.char_id());
    //             character.remove_movement_task_id(current_movement_task_id);
    //         }
    //         save_character_position(server.clone(), session.clone()).await;
    //     }
    // });
    // handle
}

pub fn change_map_packet(destination_map: &String, x: u16, y: u16, session: Arc<Session>, server: Arc<Server>) {
    let char_id = session.char_id();
    let character = server.get_character_unsafe(char_id);
    server.add_to_next_tick(Event::CharacterClearFov(char_id));
    server.add_to_next_tick(Event::CharacterRemoveFromMap(char_id));

    let map_name: String = Map::name_without_ext(destination_map.to_string());
    debug!("Char enter on map {}", map_name);
    let map_ref = server.maps.get(&map_name).unwrap();
    let map_instance = map_ref.player_join_map(server.clone());
    if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
        let walkable_cell = Map::find_random_walkable_cell(&map_instance.cells, map_instance.x_size);
        server.add_to_next_tick(Event::CharacterUpdatePosition(CharacterUpdatePosition{
            char_id: session.char_id.unwrap(),
            x: walkable_cell.0,
            y: walkable_cell.1,
        }));
    } else {
        server.add_to_next_tick(Event::CharacterUpdatePosition(CharacterUpdatePosition{
            char_id: session.char_id.unwrap(), x, y,
        }));
    }

    server.add_to_next_tick(Event::CharacterChangeMap(CharacterChangeMap{
        char_id: session.char_id.unwrap(),
        new_map_name: destination_map.clone(),
        new_instance_id: map_instance.id,
        new_position: Some(Position {x, y, dir: 3}),
        old_map_name: None,
        old_position: None
    }));

    server.insert_map_item(session.account_id, character.to_map_item());

    // if let Some(runtime) = runtime {
    //     runtime.spawn(async move {
    //         save_character_position(server, session.clone()).await
    //     });
    // }
}
//
// pub async fn save_character_position(server: Arc<Server>, session: Arc<Session>) -> Result<(), Error> {
//     let char_id = session.char_id();
//     let character = server.get_character_unsafe(char_id);
//     server.repository.character_save_position(session.account_id, char_id, Map::name_without_ext(character.get_current_map_name()), character.x(), character.y()).await
// }