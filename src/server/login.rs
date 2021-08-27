use crate::server::server::PacketHandler;
use byteorder::WriteBytesExt;

#[derive(Clone)]
pub struct LoginServer;

impl PacketHandler for LoginServer {

    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        if packet[0] == 0xc4 && packet[1] == 0x0a {
            // PACKET_AC_ACCEPT_LOGIN
            // char server IP is in bytes 64..68
            // char server port is in bytes 68..70
            println!("{:x?}", &packet[64..70]);
            let mut wtr = Vec::new();
            wtr.write_u8(0xeb).unwrap();
            packet[68] = wtr[0];
        }
        Result::Ok("res".to_string())
    }
}