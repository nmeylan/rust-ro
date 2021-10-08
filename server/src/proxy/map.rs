use crate::proxy::proxy::{PacketHandler, Proxy};
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use std::net::{TcpStream, Ipv4Addr, SocketAddr};
use crate::packets::packets::{Packet};
use crate::server::core::{Server};

#[derive(Clone)]
pub struct MapProxy {
}

impl MapProxy {
    pub(crate) fn new(server: Arc<Server>) -> Proxy<MapProxy> {
        let server = Proxy {
            name: "map".to_string(),
            local_port: 6124,
            target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6122),
            server,
            specific_proxy: MapProxy {},
        };
        return server;
    }
}

impl PacketHandler for MapProxy {
    fn handle_packet(&self, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        Result::Ok("res".to_string())
    }
}