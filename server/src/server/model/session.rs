use std::collections::HashMap;
use std::fs::File;
use std::net::{Shutdown, TcpStream};
use std::sync::{Arc, Mutex, RwLock};

use crate::server::state::character::Character;
use packets::packets::Packet;
use serde::Serialize;
use serde_json::value::RawValue;
use std::io::Write;
use tokio::sync::mpsc::Sender;

pub struct Session {
    pub char_server_socket: Option<Arc<RwLock<TcpStream>>>,
    pub map_server_socket: Option<Arc<RwLock<TcpStream>>>,
    pub account_id: u32,
    // random value, known as login_id1 in hercules
    pub auth_code: i32,
    // random value, known as login_id2 in hercules
    pub user_level: u32,
    pub char_id: Option<u32>,
    pub packetver: u32,
    pub is_simulated: bool,
    pub script_handler_channel_sender: Mutex<Option<Sender<Vec<u8>>>> // TODO keep track on creation. Abort script thread after X minutes + abort on new script interaction
}

#[derive(Serialize)]
pub struct SessionRecord {
    pub session_id: u32,
    pub char_id: Option<u32>,
    position: Position,
    packetver: u32,
    pub entries: Mutex<Vec<SessionRecordEntry>>
}
#[derive(Serialize)]
struct Position {
    x: u16,
    y: u16,
    dir: u16,
}

const PACKET_TO_RECORDS: &'static [&'static str; 1] = &[
    "PacketCzRequestMove"
];
impl SessionRecord {
    pub fn new(character: &Character, packetver: u32) -> Self {
        Self {
            session_id: character.account_id,
            char_id: Some(character.char_id),
            position: Position {x: character.x, y: 0, dir: 0 },
            entries: Mutex::new(vec![]),
            packetver,
        }
    }

    pub fn record(&self, tick: u128, packet_id: String, packet_name: String, packet: &dyn Packet) {
        if PACKET_TO_RECORDS.contains(&packet_name.as_str()) {
            self.entries.lock().unwrap().push(SessionRecordEntry { time: tick, packet_id, packet_name, data: RawValue::from_string(packet.to_json(self.packetver)).unwrap() });
        } else {
            debug!("Will not record packet with name {}", packet_name);
        }
    }

    pub fn finish(&self) {
        if self.entries.lock().unwrap().is_empty() {
            return
        }
        let mut file = File::create(format!("target/session_{}.json", self.session_id)).unwrap();
        file.write_all(&serde_json::to_string_pretty(self).unwrap().as_bytes()).unwrap();
    }
}

#[derive(Serialize)]
struct SessionRecordEntry {
    time: u128,
    packet_id: String,
    packet_name: String,
    data: Box<RawValue>
}

pub trait SessionsIter {
    fn find_by_stream(&self, tcp_stream: &TcpStream) -> Option<u32>;
}

impl SessionsIter for HashMap<u32, Arc<Session>> {
    fn find_by_stream(&self, tcp_stream: &TcpStream) -> Option<u32> {
        let map_entry_option = self.iter().find(|(_, session)| {
            if session.map_server_socket.is_some() {
                let map_server_socket = read_lock!(session.map_server_socket.as_ref().unwrap());
                let is_map_stream = if session.is_simulated {
                    map_server_socket.peer_addr().is_ok() && map_server_socket.local_addr().unwrap() == tcp_stream.peer_addr().unwrap()
                } else {
                    map_server_socket.peer_addr().is_ok() && map_server_socket.peer_addr().unwrap() == tcp_stream.peer_addr().unwrap()
                };
                if is_map_stream {
                    return true;
                }
            }
            if session.char_server_socket.is_none() {
                return false
            }
            let char_server_socket = read_lock!(session.char_server_socket.as_ref().unwrap());
            if char_server_socket.peer_addr().is_err() {
                return false;
            }
            debug!("char_server_socket.peer_addr {:?}", char_server_socket.peer_addr());
            char_server_socket.peer_addr().unwrap() == tcp_stream.peer_addr().unwrap()
        });
        map_entry_option?;
        Some(*map_entry_option.unwrap().0)
    }
}

impl Session {

    pub fn create_empty(account_id: u32, auth_code: i32, user_level: u32, packetver: u32) -> Session {
        Session {
            char_server_socket: None,
            map_server_socket: None,
            account_id,
            auth_code,
            user_level,
            char_id: None,
            packetver,
            is_simulated: false,
            script_handler_channel_sender: Mutex::new(None)
        }
    }

    pub fn recreate_with_char_socket(&self, char_socket: Arc<RwLock<TcpStream>>) -> Session {
        Session {
            char_server_socket: Some(char_socket),
            map_server_socket: self.map_server_socket.clone(),
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: self.char_id,
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_handler_channel_sender: Mutex::new(None)
        }
    }

    pub fn recreate_with_map_socket(&self, map_socket: Arc<RwLock<TcpStream>>) -> Session {
        Session {
            char_server_socket: self.char_server_socket.clone(),
            map_server_socket: Some(map_socket),
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: self.char_id,
            packetver: self.packetver,
            is_simulated: false,
            script_handler_channel_sender: Mutex::new(None)
        }
    }

    pub fn create_with_map_socket_and_char_id(account_id: u32, char_id: u32, packetver: u32, map_socket: Arc<RwLock<TcpStream>>) -> Session {
        Session {
            char_server_socket: None,
            map_server_socket: Some(map_socket),
            account_id,
            auth_code: 0,
            user_level: 99,
            char_id: Some(char_id),
            packetver,
            is_simulated: true,
            script_handler_channel_sender: Mutex::new(None)
        }
    }

    pub fn recreate_with_character(&self, char_id: u32) -> Session {
        Session {
            char_server_socket: self.char_server_socket.clone(),
            map_server_socket: self.map_server_socket.clone(),
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: Some(char_id),
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_handler_channel_sender: Mutex::new(None)
        }
    }

    pub fn recreate_without_character(&self) -> Session {
        Session {
            char_server_socket: self.char_server_socket.clone(),
            map_server_socket: None,
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: None,
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_handler_channel_sender: Mutex::new(None)
        }
    }

    pub fn snapshot(&self) -> Session {
        Session {
            char_server_socket: None,
            map_server_socket: None,
            account_id: self.account_id,
            auth_code: self.auth_code,
            user_level: self.user_level,
            char_id: self.char_id,
            packetver: self.packetver,
            is_simulated: self.is_simulated,
            script_handler_channel_sender: Mutex::new(None),
        }
    }

    pub fn set_script_handler_channel_sender(&self, script_handler_channel_sender: Sender<Vec<u8>>) {
        *self.script_handler_channel_sender.lock().unwrap() = Some(script_handler_channel_sender);
    }

    pub fn char_id(&self) -> u32 {
        self.char_id.expect("Expect char_id to be set in the session, but was not")
    }

    pub fn disconnect(&self) {
        if let Some(socket) = self.char_server_socket.as_ref() {
            let write_guard = socket.write().unwrap();
            if let Ok(_) = write_guard.shutdown(Shutdown::Both) {
                debug!("Disconnected from char server");
            }
        }
        if let Some(socket) = self.map_server_socket.as_ref() {
            let write_guard = socket.write().unwrap();
            if let Ok(_) = write_guard.shutdown(Shutdown::Both) {
                debug!("Disconnected from map server");
            }
        }
    }
}