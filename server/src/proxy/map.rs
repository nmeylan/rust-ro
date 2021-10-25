use crate::proxy::proxy::{PacketHandler, Proxy};
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use packets::packets::{Packet};
use crate::server::configuration::ProxyConfig;

#[derive(Clone)]
pub struct MapProxy {
}

impl MapProxy {
    pub(crate) fn new(config: &ProxyConfig) -> Proxy<MapProxy> {
        let server = Proxy {
            name: "map".to_string(),
            local_port: config.local_map_server_port,
            target: SocketAddr::new(IpAddr::from_str(&config.remote_map_server_ip).unwrap(), config.remote_map_server_port),
            specific_proxy: MapProxy {},
        };
        return server;
    }
}

impl PacketHandler for MapProxy {
    fn handle_packet(&self, _tcp_stream: Arc<Mutex<TcpStream>>, _packet: &mut dyn Packet) -> Result<String, String> {
        Result::Ok("res".to_string())
    }
}