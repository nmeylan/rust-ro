use std::net::{SocketAddr, TcpStream};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use packets::packets::{Packet, PacketChSendMapInfo, PacketHcNotifyZonesvr, ZserverAddr};
use crate::proxy::{PacketHandler, Proxy};
use crate::server::core::configuration::ProxyConfig;

#[derive(Clone)]
pub struct CharProxy {
}

impl CharProxy {
    pub(crate) fn new(config: &ProxyConfig) -> Proxy<CharProxy> {
        Proxy {
            name: "Char".to_string(),
            local_port: config.local_char_server_port,
            target: SocketAddr::new(IpAddr::from_str(&config.remote_char_server_ip).unwrap(), config.remote_char_server_port),
            specific_proxy: CharProxy {}
        }
    }
}
impl PacketHandler for CharProxy {

    fn handle_packet(&self, _tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) {
        if packet.as_any().downcast_ref::<PacketChSendMapInfo>().is_some() {
            let packet_send_map_info = packet.as_any_mut().downcast_mut::<PacketChSendMapInfo>().unwrap();
            packet_send_map_info.set_map_server_port(6124);
            packet_send_map_info.fill_raw();
        }
        if packet.as_any().downcast_ref::<PacketHcNotifyZonesvr>().is_some() {
            let packet_send_map_info = packet.as_any_mut().downcast_mut::<PacketHcNotifyZonesvr>().unwrap();
            let  mut addr = ZserverAddr::new();
            addr.set_ip(16777343);
            addr.set_port(6124);
            packet_send_map_info.set_addr(addr);
            packet_send_map_info.fill_raw();
        }
    }

}