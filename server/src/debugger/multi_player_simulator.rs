use crate::server::model::session::Session;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::Server;
use packets::packets::{Packet, PacketChSelectChar, PacketCzEnter2};
use rand::{thread_rng, RngCore};
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

pub struct MultiPlayerSimulator {
    sessions: Vec<Arc<Session>>,
    server: Arc<Server>,
}
impl MultiPlayerSimulator {
    pub fn new(server: Arc<Server>) -> Self {
        Self { sessions: vec![], server }
    }
    pub fn simulate(&mut self, char_id: u32) {
        let client_socket = TcpStream::connect(format!("127.0.0.1:{}", GlobalConfigService::instance().config().server.port)).unwrap();
        info!("Start simulation for char {}, created socket: {}", char_id, client_socket.local_addr().unwrap());
        let account_id = thread_rng().next_u32();
        let session = Session::create_with_map_socket_and_char_id(account_id, char_id, GlobalConfigService::instance().packetver(),
                                                                  Arc::new(RwLock::new(client_socket)));
        let session = Arc::new(session);
        self.sessions.push(session.clone());
        self.server.state().add_session(account_id, session.clone());

        let mut packet = PacketChSelectChar::new(GlobalConfigService::instance().packetver());
        packet.fill_raw();
        session.map_server_socket.as_ref().unwrap().write().unwrap().write_all(packet.raw().as_slice());
        sleep(Duration::from_millis(500));
        let mut packet= PacketCzEnter2::new(GlobalConfigService::instance().packetver());
        packet.aid = account_id;
        packet.auth_code = session.auth_code;
        packet.fill_raw();
        session.map_server_socket.as_ref().unwrap().write().unwrap().write_all(packet.raw().as_slice());
    }
}