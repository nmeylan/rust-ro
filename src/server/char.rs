use crate::server::server::{PacketHandler, Server};
use byteorder::WriteBytesExt;
use std::net::{SocketAddr, Ipv4Addr, TcpStream};
use std::net::IpAddr;
use std::io::Write;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::borrow::Borrow;

#[derive(Clone)]
pub struct CharServer {
    sessions: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>
}

impl CharServer {
    pub(crate) fn new() -> Server<CharServer> {
        let sessions = Arc::new(Mutex::new(Vec::new()));
        let sessions_ref = sessions.clone();
        let server = Server {
            name: "Char".to_string(),
            local_port: 6123,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6121),
            packet_handler: CharServer {
                sessions: sessions_ref
            }
        };
        let sessions_ref = sessions.clone();
        spawn(move || {
            while(true) {
                let sessions_guard = sessions_ref.lock().unwrap();
                println!("current sessions {}", sessions_guard.to_vec().len());
                for a in sessions_guard.to_vec() {
                    let mut tcp_stream_guard = a.lock().unwrap();
                    let buffer : [u8; 25] = [0x8D, 0x00, 0x19, 0x00, 0x80, 0x84, 0x1E, 0x00, 0x77, 0x61, 0x6C, 0x6B, 0x69, 0x72, 0x79, 0x20, 0x3A, 0x20, 0x71, 0x77, 0x65, 0x72, 0x74, 0x7A, 0x00];
                    println!("Send {:02X?} to {}", buffer, tcp_stream_guard.peer_addr().unwrap());
                    tcp_stream_guard.write(&buffer);
                    tcp_stream_guard.flush();
                }
                drop(sessions_guard);
                sleep(Duration::new(2, 0));
            }
        });
        return server;
    }
}
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

    fn handle_connection(&mut self, tcp_stream: Arc<Mutex<TcpStream>>) {
        println!("New connection in char server");
        let mut sessions_guard = self.sessions.lock().unwrap();
        sessions_guard.push(tcp_stream);
    }
}