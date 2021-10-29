use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use packets::packets::Packet;
use crate::server::server::Server;

pub fn handle_attack(server: Arc<Server>, packet: &mut dyn Packet, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, session_id: u32) {

}