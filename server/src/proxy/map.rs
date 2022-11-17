use std::net::{SocketAddr, TcpStream};
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use packets::packets::Packet;
use crate::proxy::{PacketHandler, Proxy};

use crate::server::core::configuration::ProxyConfig;

#[derive(Clone)]
pub struct MapProxy {}

impl MapProxy {
    pub(crate) fn new(config: &ProxyConfig) -> Proxy<MapProxy> {
        Proxy {
            name: "map".to_string(),
            local_port: config.local_map_server_port,
            target: SocketAddr::new(IpAddr::from_str(&config.remote_map_server_ip).unwrap(), config.remote_map_server_port),
            specific_proxy: MapProxy {},
        }
    }
}

impl PacketHandler for MapProxy {
    fn handle_packet(&self, _tcp_stream: Arc<Mutex<TcpStream>>, _packet: &mut dyn Packet) {
        // noop
    }
}