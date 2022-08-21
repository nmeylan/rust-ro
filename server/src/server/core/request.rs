use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use tokio::runtime::Runtime;

use packets::packets::Packet;
use crate::server::core::session::Session;

pub struct RequestContext<'server: 'request, 'request> {
    runtime: &'server Runtime,
    session_id: Option<u32>,
    packet_ver: u32,
    packet: &'request dyn Packet,
    socket: Arc<RwLock<TcpStream>>,
    session: Option<Arc<Session>>,
}

impl<'server: 'request, 'request> RequestContext<'server, 'request> {
    pub fn new(runtime: &'server Runtime, session_id: Option<u32>, packet_ver: u32, socket: Arc<RwLock<TcpStream>>, packet: &'request dyn Packet) -> Self {
        Self {
            runtime,
            session_id,
            packet_ver,
            packet,
            socket,
            session: None
        }
    }

    pub fn packet(&self) -> &'request dyn Packet {
        self.packet
    }

    pub fn set_session(&mut self, session: Arc<Session>) {
        self.session = Some(session);
    }

    pub fn runtime(&self) -> &'server Runtime {
        self.runtime
    }

    pub fn packet_ver(&self) -> u32 {
        self.packet_ver
    }

    pub fn socket(&self) -> Arc<RwLock<TcpStream>> {
        self.socket.clone()
    }

    pub fn session(&self) -> Arc<Session> {
        self.session.as_ref().expect("Expected session to not be null in RequestContext. Ensure session exists before calling this method").clone()
    }
}