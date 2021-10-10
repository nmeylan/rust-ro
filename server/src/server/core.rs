// This is the core of this server. As all feature won't be implemented in one shot, the idea is to proxy unimplemented feature to hercules server.
// For the moment I won't implement TCPListener in this file, but in the "future" proxies will be removed and only this file will have a TCPListener.

use crate::packets::packets::{Packet, PacketUnknown, PacketCaLogin, PacketChEnter, PacketChMakeChar2, PacketChDeleteChar4Reserved, PacketCzEnter2, PacketChSelectChar, PacketCzRestart,  PacketCzReqDisconnect2, PacketCzRequestMove2, PacketCzNotifyActorinit, PacketCzBlockingPlayCancel, PacketZcLoadConfirm};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};
use crate::repository::lib::Repository;
use sqlx::{MySql};
use std::collections::HashMap;
use tokio::runtime::Runtime;
use crate::server::login::handle_login;
use crate::server::char::{handle_char_enter, handle_make_char, handle_delete_reserved_char, handle_select_char, handle_enter_game, handle_restart, handle_disconnect, handle_char_loaded_client_side};
use crate::server::movement::{handle_char_move, Position};
use crate::server::map::Map;
use accessor::Setters;
use crate::server::scripts::warps::Warp;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener, Shutdown};
use log::{error};
use crate::packets::packets_parser::parse;
use crate::{read_lock, socket_send};

pub struct Server {
    pub sessions: Arc<RwLock<HashMap<u32, Arc<RwLock<Session>>>>>,
    pub repository: Arc<Repository<MySql>>,
    pub maps: Arc<RwLock<HashMap<String, Map>>>,
    pub warps: Arc<HashMap<String, Vec<Arc<Warp>>>>,
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
        warps: Arc<HashMap<String, Vec<Arc<Warp>>>>
    ) -> Server {
        let server = Server {
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<RwLock<Session>>>::new())),
            repository,
            maps,
            warps
        };
        server
    }

    pub fn start(self, port: u16) -> JoinHandle<()> {
        self.listen(port)
    }

    fn listen(self, port: u16) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        println!("Server listen on 0.0.0.0:{}", port);
        let server_shared_ref = Arc::new(self);
        let server_shared_ref = server_shared_ref.clone();
        spawn(move || {
            for tcp_stream in listener.incoming() {
                // Receive new connection, starting new thread
                let server_shared_ref = server_shared_ref.clone();
                spawn(move || {
                    let runtime = Runtime::new().unwrap();
                    let mut tcp_stream = tcp_stream.unwrap();
                    let tcp_stream_arc = Arc::new(RwLock::new(tcp_stream.try_clone().unwrap())); // todo remove this clone
                    let mut buffer = [0; 2048];
                    loop {
                        match tcp_stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                if bytes_read == 0 {
                                    tcp_stream.shutdown(Shutdown::Both);
                                    break;
                                }
                                let mut packet = parse(&mut buffer[..bytes_read]);
                                server_shared_ref.dispatch(&runtime, tcp_stream_arc.clone(), packet.as_mut());
                            }
                            Err(err) => error!("{}", err)
                        }
                    }
                });
            }
        })
    }


    pub fn dispatch(&self, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, packet: &mut dyn Packet) {
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
            let mut packet_zc_load_confirm = PacketZcLoadConfirm::new();
            packet_zc_load_confirm.fill_raw();
            socket_send!(tcp_stream, &packet_zc_load_confirm.raw());
        }
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<RwLock<TcpStream>>) -> Option<u32> {
        let session_guard = self.sessions.read().unwrap();
        let stream_guard = read_lock!(tcp_stream);
        let session_option = session_guard.find_by_stream(&stream_guard);
        if session_option.is_none() {
            // TODO uncomment below. keep it comment while we need to proxy data to hercules, so until forever
            // stream_guard.shutdown(Both);
            return None
        }
        Some(session_option.unwrap())
    }
}