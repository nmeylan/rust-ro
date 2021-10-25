use crate::proxy::proxy::{PacketHandler, Proxy};
use std::net::{SocketAddr, TcpStream};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use packets::packets::{Packet, PacketChSendMapInfo};
use crate::server::configuration::ProxyConfig;

#[derive(Clone)]
pub struct CharProxy {
}

impl CharProxy {
    pub(crate) fn new(config: &ProxyConfig) -> Proxy<CharProxy> {
        let server = Proxy {
            name: "Char".to_string(),
            local_port: config.local_char_server_port,
            target: SocketAddr::new(IpAddr::from_str(&config.remote_char_server_ip).unwrap(), config.remote_char_server_port),
            specific_proxy: CharProxy {}
        };
        return server;
    }
}
impl PacketHandler for CharProxy {

    fn handle_packet(&self, _tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        if packet.as_any().downcast_ref::<PacketChSendMapInfo>().is_some() {
            let packet_send_map_info = packet.as_any_mut().downcast_mut::<PacketChSendMapInfo>().unwrap();
            packet_send_map_info.set_map_server_port(6124);
            packet_send_map_info.fill_raw();
        }
        Result::Ok("res".to_string())
    }

}