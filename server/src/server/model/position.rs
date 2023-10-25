use packets::packets::{PacketCzRequestMove, PacketCzRequestMove2};
use models::position::Position;


pub trait PositionPacket {
    fn from_move_packet(packet: &PacketCzRequestMove) -> Position;
    fn from_move2_packet(packet: &PacketCzRequestMove2) -> Position;
}

impl PositionPacket for Position {
    fn from_move_packet(packet: &PacketCzRequestMove) -> Position {
        // example: for a movement to 158, 158 cell we receive following bytes for packet.dest_raw: 27, 89, E0
        let x = ((packet.dest_raw[0] as u16) << 2) | (packet.dest_raw[1] >> 6) as u16; // example: 158
        let y = (((packet.dest_raw[1] & 0x3f) as u16) << 4) | (packet.dest_raw[2] >> 4) as u16; // example: 158
        let dir: u16 = (packet.dest_raw[2] & 0x0f) as u16; // not use for the moment
        Position { x, y, dir }
    }
    fn from_move2_packet(packet: &PacketCzRequestMove2) -> Position {
        // example: for a movement to 158, 158 cell we receive following bytes for packet.dest_raw: 27, 89, E0
        let x = ((packet.dest_raw[0] as u16) << 2) | (packet.dest_raw[1] >> 6) as u16; // example: 158
        let y = (((packet.dest_raw[1] & 0x3f) as u16) << 4) | (packet.dest_raw[2] >> 4) as u16; // example: 158
        let dir: u16 = (packet.dest_raw[2] & 0x0f) as u16; // not use for the moment
        Position { x, y, dir }
    }
}
