use crate::packets::packets::{PacketCzRequestMove2, Packet, PacketZcNotifyPlayermove};
use crate::server::core::{Server, FeatureState};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::time::SystemTime;
use std::io::Write;
use crate::util::debug::debug_in_game_chat;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::DerefMut;
use crate::server::map::Map;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: i16,
    pub y: i16,
    pub(crate) dir: i16
}

impl Position {
    pub fn from_move_packet(packet: &PacketCzRequestMove2) -> Position {
        // example: for a movement to 158, 158 cell we receive following bytes for packet.dest_raw: 27, 89, E0
        let x = (((packet.dest_raw[0] & 0xff) as i16) << 2) | (packet.dest_raw[1] >> 6) as i16; // example: 158
        let y = (((packet.dest_raw[1] & 0x3f) as i16) << 4) | (packet.dest_raw[2] >> 4) as i16; // example: 158
        let dir: i16 = (packet.dest_raw[2] & 0x0f) as i16; // not use for the moment
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
    let mut server_context_guard = server.server_context.lock().unwrap();
    let mut session = server_context_guard.sessions.get_mut(&session_id).unwrap();
    let destination = Position::from_move_packet(move_packet);
    let character_session = session.character.as_mut().unwrap();

    let map_name : String = Map::name_without_ext(character_session.get_current_map_name());
    let maps_guard = server.maps.lock().unwrap();
    let map = maps_guard.get(&map_name[..]).unwrap();
    let is_walkable = map.is_cell_walkable(destination.x, destination.y);
    // TODO
    // * Control if cell is walkable
    // * Control player state (dead? stun?, frozen?)
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    packet_zc_notify_playermove.set_move_data(character_session.current_position.to_move_data(destination.clone()));
    packet_zc_notify_playermove.set_move_start_time(SystemTime::now().elapsed().unwrap().as_secs() as u32);
    packet_zc_notify_playermove.fill_raw();

    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_zc_notify_playermove.raw());
    tcp_stream_guard.flush();
    character_session.set_current_x(destination.x);
    character_session.set_current_y(destination.y);
    debug_in_game_chat(session, format!("destination: {:?}, is_walkable: {:?}", destination, is_walkable));
    debug_in_game_chat(session, format!("{:?}{:?}, is_walkable: {:?}", destination.x, destination.y + 1, map.is_cell_walkable(destination.x, destination.y + 1)));
    debug_in_game_chat(session, format!("{:?}{:?}, is_walkable: {:?}", destination.x + 1, destination.y, map.is_cell_walkable(destination.x + 1, destination.y)));
    debug_in_game_chat(session, format!("{:?}{:?}, is_walkable: {:?}", destination.x, destination.y - 1, map.is_cell_walkable(destination.x, destination.y - 1)));
    debug_in_game_chat(session, format!("{:?}{:?}, is_walkable: {:?}", destination.x - 1, destination.y , map.is_cell_walkable(destination.x - 1, destination.y)));
    debug_in_game_chat(session, format!("move_data {:X?}", packet_zc_notify_playermove.move_data_raw));
    return FeatureState::Implemented(Box::new(packet_zc_notify_playermove));
}