use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use crate::server::core::character::CharacterSession;
use crate::{read_lock};

pub struct Session {
    pub char_server_socket: Option<Arc<RwLock<TcpStream>>>,
    pub map_server_socket: Option<Arc<RwLock<TcpStream>>>,
    pub account_id: u32,
    // random value, known as login_id1 in hercules
    pub auth_code: i32,
    // random value, known as login_id2 in hercules
    pub user_level: u32,
    pub character: Option<Arc<Mutex<CharacterSession>>>
}

pub trait SessionsIter {
    fn find_by_stream(&self, tcpStream: &TcpStream) -> Option<u32>;
}

impl SessionsIter for HashMap<u32, Arc<RwLock<Session>>> {
    fn find_by_stream(&self, tcpStream: &TcpStream) -> Option<u32> {
        let map_entry_option = self.iter().find(|(_, session)| {
            let session = session.read().unwrap();
            if session.char_server_socket.as_ref().is_none() {
                return false
            }
            let char_server_socket = read_lock!(session.char_server_socket.as_ref().unwrap());
            let is_char_stream = char_server_socket.peer_addr().unwrap() == tcpStream.peer_addr().unwrap();
            let mut is_map_stream = false;
            if session.map_server_socket.as_ref().is_some() {
                let map_server_socket = read_lock!(session.map_server_socket.as_ref().unwrap());
                is_map_stream = map_server_socket.peer_addr().unwrap() == tcpStream.peer_addr().unwrap();
            }
            is_char_stream || is_map_stream
        });
        if map_entry_option.is_none() {
            return None;
        }
        Some(map_entry_option.unwrap().0.clone())
    }
}

impl Session {
    pub fn set_char_server_socket(&mut self, tcpStream: Arc<RwLock<TcpStream>>) {
        self.char_server_socket = Some(tcpStream);
    }
    pub fn set_map_server_socket(&mut self, tcpStream: Arc<RwLock<TcpStream>>) {
        self.map_server_socket = Some(tcpStream);
    }
    pub fn set_character(&mut self, character: Arc<Mutex<CharacterSession>>) {
        self.character = Some(character);
    }
    pub fn unset_character(&mut self) {
        self.character = None;
    }
}