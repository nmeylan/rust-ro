use std::net::TcpStream;
use std::sync::{Arc, RwLock};
pub struct Response {
    socket: Arc<RwLock<TcpStream>>,
    packet: Vec<u8>,
}

impl Response {
    pub fn new(socket: Arc<RwLock<TcpStream>>, packet: Vec<u8>) -> Self {
        Self {
            socket,
            packet
        }
    }

    pub fn socket(&self) -> Arc<RwLock<TcpStream>> {
        self.socket.clone()
    }

    pub fn serialized_packet(&self) -> &Vec<u8> {
        &self.packet
    }
}