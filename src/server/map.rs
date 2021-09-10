use crate::server::server::{PacketHandler, ServerContext, Server, Session};
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use std::thread::{sleep};
use std::time::Duration;
use std::io::{Write, Cursor};
use std::thread;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::packets::packets::{Packet, PacketCzEnter2};

#[derive(Clone)]
pub struct MapServer {
    server_context: Arc<Mutex<ServerContext>>,
}

impl MapServer {
    pub(crate) fn new(server_context: Arc<Mutex<ServerContext>>) -> Server<MapServer> {
        let server = Server {
            name: "map".to_string(),
            local_port: 6124,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6122),
            packet_handler: MapServer {
                server_context: server_context.clone()
            },
        };
        let server_context_ref = server_context.clone();
        thread::Builder::new().name("map server tick".to_string()).spawn(move || {
            loop {
                let server_context_guard = server_context_ref.lock().unwrap();
                for session in server_context_guard.sessions.values() {
                    if session.map_server_socket.as_ref().is_none() {
                        continue;
                    }
                    let mut tcp_stream_guard = session.map_server_socket.as_ref().unwrap().lock().unwrap();
                    let buffer: [u8; 25] = [0x8D, 0x00, 0x19, 0x00, 0x80, 0x84, 0x1E, 0x00, 0x77, 0x61, 0x6C, 0x6B, 0x69, 0x72, 0x79, 0x20, 0x3A, 0x20, 0x71, 0x77, 0x65, 0x72, 0x74, 0x7A, 0x00];
                    // println!("Send {:02X?} to {}", buffer, session.account_id);
                    tcp_stream_guard.write(&buffer);
                    tcp_stream_guard.flush();
                    drop(tcp_stream_guard);
                }
                drop(server_context_guard);
                sleep(Duration::new(2, 0));
            }
        });
        return server;
    }
}

impl PacketHandler for MapServer {
    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        if packet.as_any().downcast_ref::<PacketCzEnter2>().is_some() { // PACKET_CZ_ENTER2
            let mut packet_ch_enter = packet.as_any().downcast_ref::<PacketCzEnter2>().unwrap();
            let account_id = packet_ch_enter.aid;
            println!("New connection in map server: account {} {}", account_id, tcp_stream.lock().unwrap().peer_addr().unwrap());
            let mut server_context_guard = self.server_context.lock().unwrap();
            let existing_session = server_context_guard.sessions.get(&account_id).unwrap();
            let char_server_socket_ref = existing_session.char_server_socket.as_ref().unwrap().clone();
            server_context_guard.sessions.insert(account_id, Session {
                char_server_socket: Some(char_server_socket_ref),
                map_server_socket: Some(tcp_stream),
                account_id,
            });
        } else {
            let server_context_guard = self.server_context.lock().unwrap();
            let incoming_stream_guard = tcp_stream.lock().unwrap();
            let session = server_context_guard.sessions.iter().find(|(_, value)| {
                let map_server_socket = value.map_server_socket.as_ref().unwrap().lock().unwrap();
                map_server_socket.peer_addr().unwrap() == incoming_stream_guard.peer_addr().unwrap()
            });
            if session.is_some() {
                println!("found session {}", session.unwrap().0);
            }
        }

        Result::Ok("res".to_string())
    }
}