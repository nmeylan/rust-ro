use crate::proxy::proxy::PacketHandler;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use crate::packets::packets::{Packet, PacketAcAcceptLogin2};

#[derive(Clone)]
pub struct LoginProxy;

impl PacketHandler for LoginProxy {
    fn handle_packet(&self, _: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> Result<String, String> {
        if packet.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
            let packet_accept_login2 = packet.as_any_mut().downcast_mut::<PacketAcAcceptLogin2>().unwrap();
            let server_char = packet_accept_login2.server_list.get_mut(0).unwrap();
            server_char.set_port(6123);
            packet_accept_login2.fill_raw();
        }
        Result::Ok("res".to_string())
    }
}