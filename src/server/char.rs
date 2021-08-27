use crate::server::server::PacketHandler;
use byteorder::WriteBytesExt;

#[derive(Clone)]
pub struct CharServer;

impl PacketHandler for CharServer {
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        if packet[0] == 0xc5 && packet[1] == 0x0a {
            // char_send_map_info
            // map server IP is in bytes 22..26
            // map server port is in bytes 26..28
            println!("{:x?}", &packet[22..28]);
            let mut wtr = Vec::new();
            wtr.write_u8(0xec).unwrap();
            packet[26] = wtr[0];
        }
        Result::Ok("res".to_string())
    }
}