use crate::server::server::{PacketHandler, Server, ServerContext, Session};
use std::net::{SocketAddr, Ipv4Addr, TcpStream};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread::{sleep};
use std::time::Duration;
use std::thread;
use crate::packets::packets::{Packet, PacketChEnter, PacketChSendMapInfo};

#[derive(Clone)]
pub struct CharServer {
    server_context: Arc<Mutex<ServerContext>>
}

impl CharServer {
    pub(crate) fn new(server_context: Arc<Mutex<ServerContext>>) -> Server<CharServer> {
        let server = Server {
            name: "Char".to_string(),
            local_port: 6123,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6121),
            packet_handler: CharServer {
                server_context: server_context.clone()
            }
        };
        let server_context_ref = server_context.clone();
        thread::Builder::new().name("char server tick".to_string()).spawn(move || {
            loop {
                // let server_context_guard = server_context_ref.lock().unwrap();
                // println!("current sessions {}", server_context_guard.sessions.len());
                // for tcp_stream in &server_context_guard.sessions {
                //     let mut tcp_stream_guard = tcp_stream.lock().unwrap();
                //     let buffer : [u8; 25] = [0x8D, 0x00, 0x19, 0x00, 0x80, 0x84, 0x1E, 0x00, 0x77, 0x61, 0x6C, 0x6B, 0x69, 0x72, 0x79, 0x20, 0x3A, 0x20, 0x71, 0x77, 0x65, 0x72, 0x74, 0x7A, 0x00];
                //     println!("Send {:02X?} to {}", buffer, tcp_stream_guard.peer_addr().unwrap());
                //     tcp_stream_guard.write(&buffer);
                //     tcp_stream_guard.flush();
                // }
                // drop(server_context_guard);
                sleep(Duration::new(2, 0));
            }
        });
        return server;
    }
}
impl PacketHandler for CharServer {

    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        if packet.as_any().downcast_ref::<PacketChSendMapInfo>().is_some() {
            let packet_send_map_info = packet.as_any_mut().downcast_mut::<PacketChSendMapInfo>().unwrap();
            packet_send_map_info.set_map_server_port(6124);
            packet_send_map_info.fill_raw();
        }
        if packet.as_any().downcast_ref::<PacketChEnter>().is_some() { // PACKET_CH_ENTER
            let packet_ch_enter = packet.as_any().downcast_ref::<PacketChEnter>().unwrap();
            let account_id = packet_ch_enter.aid;
            println!("New connection in char server: account {}", account_id);
            let mut server_context_guard = self.server_context.lock().unwrap();
            server_context_guard.sessions.insert(account_id, Session {
                char_server_socket: Some(tcp_stream),
                map_server_socket: None,
                account_id
            });
        }
        Result::Ok("res".to_string())
    }

}