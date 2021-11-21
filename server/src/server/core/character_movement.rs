use packets::packets::{PacketCzRequestMove2, Packet, PacketZcNpcackMapmove};
use tokio::runtime::Runtime;
use std::sync::{Arc};
use std::sync::atomic::Ordering::{Relaxed};
use std::thread::sleep;

use tokio::time::Duration;
use tokio::task::JoinHandle;
use crate::server::core::character::Character;
use crate::server::core::map::{Map, MAP_EXT, MapItem};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::path::{MOVE_COST, MOVE_DIAGONAL_COST, PathNode};
use crate::server::core::session::Session;
use crate::server::server::Server;
use crate::util::string::StringUtil;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
    pub(crate) dir: u16
}

impl Position {
    pub fn from_move_packet(packet: &PacketCzRequestMove2) -> Position {
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
pub fn move_character_task(runtime: &Runtime, path: Vec<PathNode>, session: Arc<Session>, server: Arc<Server>, task_id: u64) -> JoinHandle<()> {
    let server = server.clone();
    let handle = runtime.spawn(async move {
        let mut has_been_canceled = false;
        {
            for path_node in path {
                let delay: u64;
                {
                    let character = session.get_character();
                    let movement_task_id = character.movement_task_id.load(Relaxed);
                    if movement_task_id != 0 && task_id != movement_task_id {
                        has_been_canceled = true;
                        break;
                    }

                    if character.x() != path_node.x && character.y() != path_node.y { // diagonal movement
                        delay = (character.status.speed * (MOVE_DIAGONAL_COST / MOVE_COST) + 10) as u64;
                    } else {
                        delay = ((character.status.speed / 2) as i16 + extra_delay(character.status.speed)) as u64;
                    }

                    {
                        let current_map_guard = read_lock!(character.current_map);
                        let map_ref = current_map_guard.as_ref().unwrap().clone();
                        if map_ref.is_warp_cell(path_node.x, path_node.y) {
                            change_map(map_ref, &path_node, session.clone(), &character);
                            break;
                        }
                    }
                    character.update_position(path_node.x, path_node.y);

                    character.load_units_in_fov(&session);
                }
                sleep(Duration::from_millis(delay));
            }
        }
        if !has_been_canceled {
            {
                let session_clone = session.clone();
                let character = session_clone.get_character();
                character.set_movement_task_id(0);
            }
            save_character_position(server.clone(), session.clone()).await;
        }
    });
    handle
}


fn change_map(map: Arc<MapInstance>, path_node: &PathNode, session: Arc<Session>, character_session: &Character) {
    let warp = map.get_warp_at(path_node.x, path_node.y).unwrap();
    let mut new_current_map: [char; 16] = [0 as char; 16];
    let map_name = format!("{}{}", warp.dest_map_name, MAP_EXT);
    map_name.fill_char_array(new_current_map.as_mut());
    character_session.remove_from_existing_map();
    character_session.set_current_map_name(new_current_map.clone());
    character_session.update_x_y(warp.to_x, warp.to_y);

    let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();
    packet_zc_npcack_mapmove.set_map_name(new_current_map);
    packet_zc_npcack_mapmove.set_x_pos(character_session.x() as i16);
    packet_zc_npcack_mapmove.set_y_pos(character_session.y() as i16);
    packet_zc_npcack_mapmove.fill_raw();

    session.send_to_map_socket(packet_zc_npcack_mapmove.raw());
}


async fn save_character_position(server: Arc<Server>, session: Arc<Session>) {
    let character = session.get_character();
    let res = sqlx::query("UPDATE `char` SET last_map = ?, last_x = ?, last_y = ? WHERE account_id = ? AND char_id = ?") // TODO add bcrypt on user_pass column, but not supported by hercules
        .bind(Map::name_without_ext(character.get_current_map_name()))
        .bind(character.x())
        .bind(character.y())
        .bind(session.account_id)
        .bind(character.char_id)
        .execute(&server.repository.pool);
    let res = res.await;
    debug!("Update char last position {:?}", res);
}