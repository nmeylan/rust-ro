use crate::server::server::{PacketHandler, ServerContext, Server};
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use std::thread::{spawn, sleep};
use std::time::Duration;
use std::io::Write;
use std::thread;

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
            }
        };
        let server_context_ref = server_context.clone();
        thread::Builder::new().name("map server tick".to_string()).spawn(move || {
            while(true) {
                println!("tick map server");
                let server_context_guard = server_context_ref.lock().unwrap();
                println!("current sessions {}", server_context_guard.sessions.len());
                for tcp_stream in &server_context_guard.sessions {
                    let mut tcp_stream_guard = tcp_stream.lock().unwrap();
                    let buffer : [u8; 25] = [0x8D, 0x00, 0x19, 0x00, 0x80, 0x84, 0x1E, 0x00, 0x77, 0x61, 0x6C, 0x6B, 0x69, 0x72, 0x79, 0x20, 0x3A, 0x20, 0x71, 0x77, 0x65, 0x72, 0x74, 0x7A, 0x00];
                    println!("Send {:02X?} to {}", buffer, tcp_stream_guard.peer_addr().unwrap());
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
    fn handle_packet(&self, packet: &mut [u8]) -> Result<String, String> {
        println!("Map");
        Result::Ok("res".to_string())
    }
    fn handle_connection(&mut self, tcp_stream: Arc<Mutex<TcpStream>>) {
        println!("New connection in char server");
        let mut server_context_guard = self.server_context.lock().unwrap();
        server_context_guard.sessions.push(tcp_stream);
    }
}