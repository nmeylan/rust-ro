use crate::proxy::proxy::{PacketHandler, ServerContext, Proxy, Session};
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use crate::packets::packets::{Packet, PacketCzEnter2};
use crate::server::core::Server;

#[derive(Clone)]
pub struct MapProxy {
    server_context: Arc<Mutex<ServerContext>>,
}

impl MapProxy {
    pub(crate) fn new(server_context: Arc<Mutex<ServerContext>>, server: Arc<Server>) -> Proxy<MapProxy> {
        let server = Proxy {
            name: "map".to_string(),
            local_port: 6124,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6122),
            server,
            specific_proxy: MapProxy {
                server_context: server_context.clone()
            },
        };
        return server;
    }
}

impl PacketHandler for MapProxy {
    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        if packet.as_any().downcast_ref::<PacketCzEnter2>().is_some() { // PACKET_CZ_ENTER2
            let packet_ch_enter = packet.as_any().downcast_ref::<PacketCzEnter2>().unwrap();
            let account_id = packet_ch_enter.aid;
            println!("New connection in map proxy: account {} {}", account_id, tcp_stream.lock().unwrap().peer_addr().unwrap());
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