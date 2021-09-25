use crate::proxy::proxy::{PacketHandler, Proxy};
use std::net::{SocketAddr, Ipv4Addr, TcpStream};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use crate::packets::packets::{Packet, PacketChEnter, PacketChSendMapInfo, PacketHcRefuseEnter};
use crate::server::core::{Server, ServerContext, Session};

#[derive(Clone)]
pub struct CharProxy {
}

impl CharProxy {
    pub(crate) fn new(server: Arc<Server>) -> Proxy<CharProxy> {
        let server = Proxy {
            name: "Char".to_string(),
            local_port: 6123,
            server,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6121),
            specific_proxy: CharProxy {}
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
        Result::Ok("res".to_string())
    }

}