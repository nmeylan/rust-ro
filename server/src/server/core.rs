// This is the core of this server. As all feature won't be implemented in one shot, the idea is to proxy unimplemented feature to hercules server.
// For the moment I won't implement TCPListener in this file, but in the "future" proxies will be removed and only this file will have a TCPListener.

use crate::packets::packets::{Packet, PacketUnknown, PacketZcNotifyTime, PacketZcNotifyChat, PacketCaLogin, PacketAcAcceptLogin2, PacketAcRefuseLoginR2, PacketAcRefuseLoginR3, PacketChEnter, PacketHcRefuseEnter, PacketChMakeChar2, PacketChDeleteChar2, PacketHcDeleteChar3Reserved, PacketChDeleteChar4Reserved, PacketCzEnter2, PacketChSelectChar, PacketCzRestart, PacketCzReqDisconnect, PacketCzReqDisconnect2, PacketCzRequestMove2, PacketCzNotifyActorinit, PacketCzBlockingPlayCancel, PacketZcLoadConfirm};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use crate::repository::lib::Repository;
use sqlx::{Database, MySql};
use crate::server::login::authenticate;
use std::collections::HashMap;
use std::net::TcpStream;
use tokio::runtime::Runtime;
use crate::server::login::handle_login;
use crate::server::char::{handle_char_enter, handle_make_char, handle_delete_reserved_char, handle_select_char, handle_enter_game, handle_restart, handle_disconnect, handle_char_loaded_client_side, handle_blocking_play_cancel};
use std::net::Shutdown::Both;
use crate::server::movement::{handle_char_move, Position};
use std::ops::{DerefMut, Deref};
use std::rc::Rc;
use crate::server::map::Map;
use tokio::task::JoinHandle;
use tokio::sync::mpsc::Sender;
use accessor::Setters;
use crate::server::scripts::warps::Warp;

pub struct Server {
    pub sessions: Arc<RwLock<HashMap<u32, RwLock<Session>>>>,
    pub repository: Arc<Repository<MySql>>,
    pub maps: Arc<RwLock<HashMap<String, Map>>>,
    pub warps: Arc<HashMap<String, Vec<Warp>>>,
}

pub enum FeatureState {
    Implemented(Box<dyn Packet>),
    Unimplemented,
}

impl Server {
    pub fn remove_session(&self, session_id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&session_id);
    }
}

pub trait SessionsIter {
    fn find_by_stream(&self, tcpStream: &TcpStream) -> Option<u32>;
}

impl SessionsIter for HashMap<u32, RwLock<Session>> {
    fn find_by_stream(&self, tcpStream: &TcpStream) -> Option<u32> {
        let map_entry_option = self.iter().find(|(_, session)| {
            let session = session.read().unwrap();
            if session.char_server_socket.as_ref().is_none() {
                return false
            }
            let char_server_socket = session.char_server_socket.as_ref().unwrap().lock().unwrap();
            let is_char_stream = char_server_socket.peer_addr().unwrap() == tcpStream.peer_addr().unwrap();
            let mut is_map_stream = false;
            if session.map_server_socket.as_ref().is_some() {
                let map_server_socket = session.map_server_socket.as_ref().unwrap().lock().unwrap();
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

pub struct Session {
    pub char_server_socket: Option<Arc<Mutex<TcpStream>>>,
    pub map_server_socket: Option<Arc<Mutex<TcpStream>>>,
    pub account_id: u32,
    // random value, known as login_id1 in hercules
    pub auth_code: i32,
    // random value, known as login_id2 in hercules
    pub user_level: u32,
    pub character: Option<Arc<Mutex<CharacterSession>>>
}

#[derive(Setters, Debug)]
pub struct CharacterSession {
    #[set]
    pub name: [char; 24],
    #[set]
    pub speed: u16,
    #[set]
    pub char_id: u32,
    #[set]
    pub current_map: [char; 16],
    #[set]
    pub current_position: Position,
    pub movement_task_id: Option<u128>
}

impl Session {
    pub fn set_char_server_socket(&mut self, tcpStream: Arc<Mutex<TcpStream>>) {
        self.char_server_socket = Some(tcpStream);
    }
    pub fn set_map_server_socket(&mut self, tcpStream: Arc<Mutex<TcpStream>>) {
        self.map_server_socket = Some(tcpStream);
    }
    pub fn set_character(&mut self, character: Arc<Mutex<CharacterSession>>) {
        self.character = Some(character);
    }
    pub fn unset_character(&mut self) {
        self.character = None;
    }
}

impl CharacterSession {
    pub fn set_current_x(&mut self, current_x: u16) {
        self.current_position.x = current_x;
    }
    pub fn set_current_y(&mut self, current_y: u16) {
        self.current_position.y = current_y;
    }
    pub fn get_current_map_name(&self) -> String {
        self.current_map.iter().filter(|c| **c != '\0').collect()
    }
    pub fn set_movement_task_id(&mut self, id: u128) {
        self.movement_task_id = Some(id);
    }
}

impl Server {
    pub fn new(
        repository: Arc<Repository<MySql>>,
        maps: Arc<RwLock<HashMap<String, Map>>>,
        warps: Arc<HashMap<String, Vec<Warp>>>
    ) -> Server {
        let server = Server {
            sessions: Arc::new(RwLock::new(HashMap::<u32, RwLock<Session>>::new())),
            repository,
            maps,
            warps
        };
        server.start_tick();
        server
    }

    pub fn start_tick(&self) {
        thread::Builder::new().name("main tick thread".to_string()).spawn(move || {
            loop {
                sleep(Duration::new(2, 0));
            }
        });
    }

    pub fn dispatch(&self, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> FeatureState {
        if packet.as_any().downcast_ref::<PacketUnknown>().is_some() {
            println!("Unknown packet {} of length {}: {:02X?}", packet.id(), packet.raw().len(), packet.raw());
        }
        // Login
        if packet.as_any().downcast_ref::<PacketCaLogin>().is_some() {
            return handle_login(self, packet, runtime, tcp_stream);
        }
        // Char selection
        if packet.as_any().downcast_ref::<PacketChEnter>().is_some() {
            return handle_char_enter(self, packet, runtime, tcp_stream);
        }
        // Char creation
        if packet.as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_make_char(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Delete char reservation
        if packet.as_any().downcast_ref::<PacketChDeleteChar4Reserved>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_delete_reserved_char(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Select char
        if packet.as_any().downcast_ref::<PacketChSelectChar>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_select_char(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Enter game
        if packet.as_any().downcast_ref::<PacketCzEnter2>().is_some() {
            return handle_enter_game(self, packet, runtime, tcp_stream);
        }
        // Game menu "Character select"
        if packet.as_any().downcast_ref::<PacketCzRestart>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_restart(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Game menu "Exit to windows"
        if packet.as_any().downcast_ref::<PacketCzReqDisconnect2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_disconnect(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Player click on map cell
        if packet.as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_char_move(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Client notify player has been loaded
        if packet.as_any().downcast_ref::<PacketCzNotifyActorinit>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_char_loaded_client_side(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Client send PACKET_CZ_BLOCKING_PLAY_CANCEL after char has loaded
        if packet.as_any().downcast_ref::<PacketCzBlockingPlayCancel>().is_some() {
            packet.debug();
            let mut packet_zc_load_confirm = PacketZcLoadConfirm::new();
            packet_zc_load_confirm.fill_raw();
            let mut tcp_stream_guard = tcp_stream.lock().unwrap();
            tcp_stream_guard.write(&packet_zc_load_confirm.raw());
            tcp_stream_guard.flush();
            return FeatureState::Implemented(Box::new(packet_zc_load_confirm));
        }
        // Char creation
        FeatureState::Unimplemented
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<Mutex<TcpStream>>) -> Option<u32> {
        let session_guard = self.sessions.read().unwrap();
        let stream_guard = tcp_stream.lock().unwrap();
        let session_option = session_guard.find_by_stream(&stream_guard);
        if session_option.is_none() {
            // TODO uncomment below. keep it comment while we need to proxy data to hercules, so until forever
            // stream_guard.shutdown(Both);
            return None
        }
        Some(session_option.unwrap())
    }
}