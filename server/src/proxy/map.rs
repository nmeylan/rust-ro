use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use configuration::configuration::ProxyConfig;
use packets::packets::Packet;

use crate::proxy::{PacketHandler, Proxy};

#[derive(Clone)]
pub struct MapProxy {}

impl MapProxy {
    pub(crate) fn new(config: &ProxyConfig) -> Proxy<MapProxy> {
        Proxy {
            name: "map".to_string(),
            local_port: config.local_map_server_port,
            target: SocketAddr::new(
                IpAddr::from_str(&config.remote_map_server_ip).unwrap(),
                config.remote_map_server_port,
            ),
            specific_proxy: MapProxy {},
            is_alive: Arc::new(AtomicBool::new(true)),
        }
    }
}

impl PacketHandler for MapProxy {
    fn handle_packet(&self, _tcp_stream: Arc<Mutex<TcpStream>>, _packet: &mut dyn Packet) {
        // noop
    }
}
