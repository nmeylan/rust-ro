use crate::packets::packets::{PacketCzRequestMove2, Packet, PacketZcNotifyPlayermove};
use crate::server::core::{Server, FeatureState};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use std::time::SystemTime;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Position {
    x: u8,
    y: u8,
    dir: u8
}

impl Position {
    pub fn from_move_packet(packet: &PacketCzRequestMove2) -> Position {
        // example: for a movement to 158, 158 cell we receive following bytes for packet.dest_raw: 27, 89, E0
        let x: u8 = ((packet.dest_raw[0] & 0xff) << 2) | (packet.dest_raw[1] >> 6); // example: 158
        let y: u8 = ((packet.dest_raw[1] & 0x3f) << 4) | (packet.dest_raw[2] >> 4); // example: 158
        let dir: u8 = packet.dest_raw[2] & 0xff; // not use for the moment
        Position { x, y ,dir }
    }

    pub fn to_move_data(self, destination: Position) -> [u8; 6] {
        let mut move_data: [u8; 6] = [0; 6];
        move_data[0] = self.x >> 2;
        move_data[1] = (self.x << 6) | ((self.y >> 4) & 0x3f);
        move_data[2] = (self.y << 4) | ((destination.x >> 6) & 0x0f);
        move_data[3] = (destination.x << 2) | ((u32::from(destination.y) >> 8) & 0x03) as u8;
        move_data[4] = destination.y;
        move_data[5] = 136; // hardcoded value in hercules (8 << 4) | (8 & 0x0f)
        move_data
    }
}


pub fn handle_char_move(server: &Server, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, session_id: u32) -> FeatureState {
    let move_packet = packet.as_any().downcast_ref::<PacketCzRequestMove2>().unwrap();
    let mut server_context_guard = server.server_context.lock().unwrap();
    let mut session = server_context_guard.sessions.get_mut(&session_id).unwrap();
    let destination = Position::from_move_packet(move_packet);
    let character_session = session.character.as_mut().unwrap();
    let current_position = Position { x: character_session.current_x as u8, y: character_session.current_y as u8, dir: 0 };
    println!("{:?}", current_position);
    println!("{:?}", destination);
    let mut packet_zc_notify_playermove = PacketZcNotifyPlayermove::new();
    packet_zc_notify_playermove.set_move_data(current_position.to_move_data(destination.clone()));
    packet_zc_notify_playermove.set_move_start_time(SystemTime::now().elapsed().unwrap().as_secs() as u32);
    packet_zc_notify_playermove.fill_raw();

    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
    tcp_stream_guard.write(&packet_zc_notify_playermove.raw());
    tcp_stream_guard.flush();
    character_session.set_current_x(destination.x as i16);
    character_session.set_current_y(destination.y as i16);
    return FeatureState::Implemented(Box::new(packet_zc_notify_playermove));
}