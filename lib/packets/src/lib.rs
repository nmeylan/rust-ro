#![allow(clippy::all)]
#![allow(unused_braces)]
#![allow(unused_parens)]

pub mod packets;
pub mod packets_print;
pub mod packets_impl;
pub mod packets_parser;
mod print;

#[cfg(test)]
mod tests {
    use crate::packets::{Packet, PacketCzRenameMer, PacketCzRequestMove};
    use crate::packets_parser::parse_json;

    // #[test]
    fn json_serde_when_packet_contains_position() {
        // Given
        let mut request_move = PacketCzRequestMove::new(20120307);
        let x: u16 = 100;
        let y: u16 = 140;
        let dir: u16 = 6;
        let mut move_data: [u8; 3] = [0; 3];
        move_data[0] = (x >> 2) as u8;
        move_data[1] = ((x << 6) | ((y >> 4) & 0x3f)) as u8;
        move_data[2] = ((y << 4) | (dir & 0xf)) as u8;
        request_move.set_dest(move_data);
        // When
        let json = request_move.to_json(20120307);
        let packet = parse_json(json.as_str(), 20120307).unwrap();
        let request_move_from_json = packet.as_any().downcast_ref::<PacketCzRequestMove>().unwrap();
        // Then
        assert_eq!(request_move_from_json.dest, request_move.dest);
    }
    // #[test]
    fn json_serde_when_packet_contains_string() {
        // Given
        let mut rename_mer = PacketCzRenameMer::new(20120307);
        let mut name: [char; 24] = [0 as char; 24];
        name[0] = 'A';
        name[1] = 'R';
        name[2] = 'C';
        name[3] = 'H';
        rename_mer.set_name(name);
        // When
        let json = rename_mer.to_json(20120307);
        let packet = parse_json(json.as_str(), 20120307).unwrap();
        let rename_mer_from_json = packet.as_any().downcast_ref::<PacketCzRenameMer>().unwrap();
        // Then
        assert_eq!(rename_mer_from_json.name, rename_mer.name);
    }
}