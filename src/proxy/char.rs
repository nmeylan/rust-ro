use crate::proxy::proxy::{PacketHandler, Proxy, ServerContext, Session};
use std::net::{SocketAddr, Ipv4Addr, TcpStream};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use crate::packets::packets::{Packet, PacketChEnter, PacketChSendMapInfo};
use crate::server::core::Server;

#[derive(Clone)]
pub struct CharProxy {
    server_context: Arc<Mutex<ServerContext>>
}

impl CharProxy {
    pub(crate) fn new(server_context: Arc<Mutex<ServerContext>>, server: Arc<Server>) -> Proxy<CharProxy> {
        let server = Proxy {
            name: "Char".to_string(),
            local_port: 6123,
            server,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6121),
            specific_proxy: CharProxy {
                server_context: server_context.clone()
            }
        };
        return server;
    }
}
impl PacketHandler for CharProxy {

    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        if packet.as_any().downcast_ref::<PacketChSendMapInfo>().is_some() {
            let packet_send_map_info = packet.as_any_mut().downcast_mut::<PacketChSendMapInfo>().unwrap();
            packet_send_map_info.set_map_server_port(6124);
            packet_send_map_info.fill_raw();
        }
        if packet.as_any().downcast_ref::<PacketChEnter>().is_some() { // PACKET_CH_ENTER
            let packet_ch_enter = packet.as_any().downcast_ref::<PacketChEnter>().unwrap();
            let account_id = packet_ch_enter.aid;
            println!("New connection in char proxy: account {}", account_id);
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