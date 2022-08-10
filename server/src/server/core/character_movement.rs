use std::fmt::{Formatter};
use std::sync::Arc;
use std::thread::sleep;

use tokio::runtime::Runtime;
use tokio::task::JoinHandle;
use tokio::time::Duration;

use packets::packets::{Packet, PacketCzRequestMove, PacketCzRequestMove2, PacketZcNpcackMapmove};
use crate::server::core::character::MovementTask;

use crate::server::core::map::{Map, MAP_EXT, MapItem, RANDOM_CELL};
use crate::server::core::path::PathNode;
use crate::server::core::session::Session;
use crate::server::server::Server;
use crate::util::string::StringUtil;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub(crate) dir: u16,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.dir)
    }
}

impl Position {
    pub fn from_move_packet(packet: &PacketCzRequestMove) -> Position {
        // example: for a movement to 158, 158 cell we receive following bytes for packet.dest_raw: 27, 89, E0
        let x = (((packet.dest_raw[0] & 0xff) as u16) << 2) | (packet.dest_raw[1] >> 6) as u16; // example: 158
        let y = (((packet.dest_raw[1] & 0x3f) as u16) << 4) | (packet.dest_raw[2] >> 4) as u16; // example: 158
        let dir: u16 = (packet.dest_raw[2] & 0x0f) as u16; // not use for the moment
        Position { x, y, dir }
    }
    pub fn from_move2_packet(packet: &PacketCzRequestMove2) -> Position {
        // example: for a movement to 158, 158 cell we receive following bytes for packet.dest_raw: 27, 89, E0
        let x = (((packet.dest_raw[0] & 0xff) as u16) << 2) | (packet.dest_raw[1] >> 6) as u16; // example: 158
        let y = (((packet.dest_raw[1] & 0x3f) as u16) << 4) | (packet.dest_raw[2] >> 4) as u16; // example: 158
        let dir: u16 = (packet.dest_raw[2] & 0x0f) as u16; // not use for the moment
        Position { x, y, dir }
    }

    pub fn to_move_data(&self, destination: &Position) -> [u8; 6] {
        let mut move_data: [u8; 6] = [0; 6];
        move_data[0] = (self.x >> 2) as u8;
        move_data[1] = ((self.x << 6) | ((self.y >> 4) & 0x3f)) as u8;
        move_data[2] = ((self.y << 4) | ((destination.x >> 6) & 0x0f)) as u8;
        move_data[3] = ((destination.x << 2) | (destination.y >> 8) & 0x03) as u8;
        move_data[4] = destination.y as u8;
        move_data[5] = 136; // hardcoded value in hercules (8 << 4) | (8 & 0x0f)
        move_data
    }

    pub fn to_pos(&self) -> [u8; 3] {
        let mut move_data: [u8; 3] = [0; 3];
        move_data[0] = (self.x >> 2) as u8;
        move_data[1] = ((self.x << 6) | ((self.y >> 4) & 0x3f)) as u8;
        move_data[2] = ((self.y << 4) | (self.dir & 0xf)) as u8;
        move_data
    }
}

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
    let handle = runtime.spawn(async move {
        let mut has_been_canceled = false;
        {
            for path_node in path {
                let delay: u64;
                {
                    let character = session.get_character();
                    let mut movement_tasks_guard = character.movement_tasks.lock().unwrap();
                    if !movement_tasks_guard.contains(&current_movement_task_id) {
                        has_been_canceled = true;
                        break;
                    }

                    if character.x() != path_node.x && character.y() != path_node.y { // diagonal movement
                        delay = (character.status.speed as f64 / 0.6) as u64;
                    } else {
                        delay = character.status.speed as u64;
                    }
                    // info!("walk delay {}", delay);
                    info!("[{:?} - {}] [{} paralell tasks] movement update_position", std::thread::current().id(), current_movement_task_id, movement_tasks_guard.len()) ;
                    character.update_position(path_node.x, path_node.y);
                    {
                        let current_map_guard = read_lock!(character.current_map);
                        let map_ref = current_map_guard.as_ref().unwrap();
                        if map_ref.is_warp_cell(path_node.x, path_node.y) {
                            let warp = map_ref.get_warp_at(path_node.x, path_node.y).unwrap();
                            drop(current_map_guard);
                            change_map(&warp.dest_map_name, warp.to_x, warp.to_y, session.clone(), server.clone());
                            info!("[{:?} - {}] Warp break", std::thread::current().id(), current_movement_task_id);
                            movement_tasks_guard.clear();
                            break;
                        }
                    }

                    character.load_units_in_fov(&session);
                }
                sleep(Duration::from_millis(delay));
            }
        }
        if !has_been_canceled {
            {
                let session_clone = session.clone();
                let character = session_clone.get_character();
                character.remove_movement_task_id(current_movement_task_id);
            }
            save_character_position(server.clone(), session.clone()).await;
        }
    });
    handle
}

pub fn change_map(destination_map: &String, x: u16, y: u16, session: Arc<Session>, server: Arc<Server>) {
    let packet_zc_npcack_mapmove = change_map_packet(destination_map, x, y, session.clone(), server.clone());
    session.send_to_map_socket(packet_zc_npcack_mapmove.raw());
    let character_session = session.get_character();
    character_session.clear_map_view();
    // TODO fix code below
    // let runtime = Runtime::new().unwrap();
    // runtime.spawn(async move {
    //     save_character_position(server, session.clone()).await
    // });
}

pub fn change_map_packet(destination_map: &String, x: u16, y: u16, session: Arc<Session>, server: Arc<Server>) -> PacketZcNpcackMapmove {
    let character_session = session.get_character();
    character_session.remove_from_existing_map();
    let map_name: String = Map::name_without_ext(destination_map.to_string());
    debug!("Char enter on map {}", map_name);
    let map_ref = server.maps.get(&map_name).unwrap();
    let map = map_ref.clone();
    let map_instance = map.player_join_map(character_session, server.clone());
    if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
        let walkable_cell = Map::find_random_walkable_cell(&map_instance.cells, map_instance.x_size);
        character_session.update_position(walkable_cell.0, walkable_cell.1);
    } else {
        character_session.update_position(x, y);
    }

    character_session.join_and_set_map(map_instance);
    server.insert_map_item(session.account_id, character_session.clone());

    let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();

    let mut new_current_map: [char; 16] = [0 as char; 16];
    let map_name = format!("{}{}", destination_map, MAP_EXT);
    map_name.fill_char_array(new_current_map.as_mut());
    packet_zc_npcack_mapmove.set_map_name(new_current_map);
    packet_zc_npcack_mapmove.set_x_pos(character_session.x() as i16);
    packet_zc_npcack_mapmove.set_y_pos(character_session.y() as i16);
    packet_zc_npcack_mapmove.fill_raw();
    packet_zc_npcack_mapmove.display();
    packet_zc_npcack_mapmove
}

pub async fn save_character_position(server: Arc<Server>, session: Arc<Session>) {
    info!("save_character_position");
    let character = session.get_character();
    let res = sqlx::query("UPDATE `char` SET last_map = ?, last_x = ?, last_y = ? WHERE account_id = ? AND char_id = ?") // TODO add bcrypt on user_pass column, but not supported by hercules
        .bind(Map::name_without_ext(character.get_current_map_name()))
        .bind(character.x())
        .bind(character.y())
        .bind(session.account_id)
        .bind(character.char_id)
        .execute(&server.repository.pool);
    let res = res.await;
    info!("Update char last position {:?}", res);
}