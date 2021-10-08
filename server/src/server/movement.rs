use crate::packets::packets::{PacketCzRequestMove2, Packet, PacketZcNotifyPlayermove};
use crate::server::core::{Server, FeatureState, CharacterSession};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;
use crate::util::debug::debug_in_game_chat;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{DerefMut};
use crate::server::map::Map;
use crate::server::path::{path_search_client_side_algorithm, PathNode, MOVE_DIAGONAL_COST, MOVE_COST};
use std::thread::sleep;
use tokio::time::Duration;
use futures::FutureExt;
use tokio::task::JoinHandle;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::error::TryRecvError;

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

pub fn handle_char_move(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let move_packet = packet.as_any().downcast_ref::<PacketCzRequestMove2>().unwrap();
    let sessions_guard = server.sessions.read().unwrap();
    let mut session = sessions_guard.get(&session_id).unwrap().try_read().unwrap();
    let destination = Position::from_move_packet(move_packet);
    let mut character_session_guard = session.character.as_ref().unwrap().lock().unwrap();
    let map_name: String = Map::name_without_ext(character_session_guard.get_current_map_name());
    let maps_guard = server.maps.read().unwrap();
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
    move_character(runtime, path.clone(), session.character.as_ref().unwrap().clone(), id.clone());
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    packet_zc_notify_playermove.set_move_data(current_position.to_move_data(destination.clone()));
    packet_zc_notify_playermove.set_move_start_time(now as u32);
    packet_zc_notify_playermove.fill_raw();
    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_zc_notify_playermove.raw());
    tcp_stream_guard.flush();
    return FeatureState::Implemented(Box::new(packet_zc_notify_playermove));
}

fn move_character(runtime: &Runtime, path: Vec<PathNode>, character: Arc<Mutex<CharacterSession>>, task_id: u128) -> JoinHandle<()> {
    let handle = runtime.spawn(async move {
        for (i, path_node) in path.iter().enumerate() {
            let mut delay: u64;
            {
                let mut character_session = character.lock().unwrap();
                if task_id != character_session.movement_task_id.unwrap(){
                    break;
                }
                if character_session.current_position.x != path_node.x && character_session.current_position.y != path_node.y { // diagonal movement
                    delay = (character_session.speed * (MOVE_DIAGONAL_COST / MOVE_COST)) as u64;
                } else {
                    delay = (character_session.speed - 25) as u64;
                }
                character_session.set_current_x(path_node.x);
                character_session.set_current_y(path_node.y);
            }
            sleep(Duration::from_millis(delay));
        }
    });
    handle
}
