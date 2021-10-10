use crate::packets::packets::{PacketCzRequestMove2, Packet, PacketZcNotifyPlayermove, PacketZcNpcackMapmove};
use crate::server::core::{Server, CharacterSession, Session};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex, RwLock};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use crate::server::map::{Map, MAP_EXT};
use crate::server::path::{path_search_client_side_algorithm, PathNode, MOVE_DIAGONAL_COST, MOVE_COST};
use std::thread::sleep;
use tokio::time::Duration;

use tokio::task::JoinHandle;
use crate::{read_lock, read_session, cast, socket_send};
use std::collections::HashMap;
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

    pub fn to_move_data(&self, destination: Position) -> [u8; 6] {
        let mut move_data: [u8; 6] = [0; 6];
        move_data[0] = (self.x >> 2) as u8;
        move_data[1] = ((self.x << 6) | ((self.y >> 4) & 0x3f)) as u8;
        move_data[2] = ((self.y << 4) | ((destination.x >> 6) & 0x0f)) as u8;
        move_data[3] = ((destination.x << 2) | (destination.y >> 8) & 0x03) as u8;
        move_data[4] = destination.y as u8;
        move_data[5] = 136; // hardcoded value in hercules (8 << 4) | (8 & 0x0f)
        move_data
    }

    pub fn is_equals(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub fn handle_char_move(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session_id: u32) {
    let move_packet = cast!(packet, PacketCzRequestMove2);
    let sessions_guard = read_lock!(server.sessions);
    let session = sessions_guard.get(&session_id).unwrap();
    let session_guard = read_lock!(session);
    let destination = Position::from_move_packet(move_packet);
    let mut character_session_guard = session_guard.character.as_ref().unwrap().lock().unwrap();
    let map_name: String = Map::name_without_ext(character_session_guard.get_current_map_name());
    let maps_guard = read_lock!(server.maps);
    let map = maps_guard.get(&map_name[..]).unwrap();
    let current_position = character_session_guard.current_position.clone();

    let path = path_search_client_side_algorithm(map, &current_position, &destination);
    // TODO
    // * Control if cell is walkable
    // * Control player state (dead? stun?, frozen?)
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    character_session_guard.set_movement_task_id(id);
    std::mem::drop(character_session_guard);
    move_character(runtime, path.clone(), session.clone(), map_name,server.maps.clone(), id.clone());
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    packet_zc_notify_playermove.set_move_data(current_position.to_move_data(destination.clone()));
    packet_zc_notify_playermove.set_move_start_time(now as u32);
    packet_zc_notify_playermove.fill_raw();
    socket_send!(tcp_stream, &packet_zc_notify_playermove.raw());
    // debug_in_game_chat(&session, format!("path: {:?}", path.iter().map(|node| (node.x, node.y)).collect::<Vec<(u16, u16)>>()));
    // debug_in_game_chat(&session, format!("current_position: {:?}, destination {:?}", current_position, destination));
}

fn move_character(runtime: &Runtime, path: Vec<PathNode>, session: Arc<RwLock<Session>>, map_name: String, maps: Arc<RwLock<HashMap<String, Map>>>, task_id: u128) -> JoinHandle<()> {
    let handle = runtime.spawn(async move {
        let maps = read_lock!(maps);
        let map = maps.get(&map_name).unwrap();
        for path_node in path {
            let delay: u64;
            {
                let session = read_lock!(session);
                let mut character_session = session.character.as_ref().unwrap().lock().unwrap();
                if task_id != character_session.movement_task_id.unwrap(){
                    break;
                }
                if character_session.current_position.x != path_node.x && character_session.current_position.y != path_node.y { // diagonal movement
                    delay = (character_session.speed * (MOVE_DIAGONAL_COST / MOVE_COST)) as u64;
                } else {
                    delay = (character_session.speed - 25) as u64;
                }
                if map.is_warp_cell(path_node.x, path_node.y) {
                    let warp = map.warps.get(&map.get_cell_index_of(path_node.x, path_node.y)).unwrap();
                    let mut new_current_map: [char; 16] = [0 as char; 16];
                    let map_name = format!("{}{}", warp.dest_map_name, MAP_EXT);
                    map_name.fill_char_array(new_current_map.as_mut());
                    character_session.current_map = new_current_map.clone();
                    character_session.set_current_x(warp.to_x);
                    character_session.set_current_y(warp.to_y);

                    let mut packet_zc_npcack_mapmove = PacketZcNpcackMapmove::new();
                    packet_zc_npcack_mapmove.set_map_name(new_current_map);
                    packet_zc_npcack_mapmove.set_x_pos(character_session.current_position.x as i16);
                    packet_zc_npcack_mapmove.set_y_pos(character_session.current_position.y as i16);
                    packet_zc_npcack_mapmove.fill_raw();
                    let tcp_stream = session.map_server_socket.as_ref().unwrap();
                    socket_send!(tcp_stream, packet_zc_npcack_mapmove.raw());

                    break;
                }
                character_session.set_current_x(path_node.x);
                character_session.set_current_y(path_node.y);
            }
            sleep(Duration::from_millis(delay));
        }
    });
    handle
}


fn save_character_position()