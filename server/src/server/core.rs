// This is the core of this server. As all feature won't be implemented in one shot, the idea is to proxy unimplemented feature to hercules server.
// For the moment I won't implement TCPListener in this file, but in the "future" proxies will be removed and only this file will have a TCPListener.

use crate::packets::packets::{Packet, PacketUnknown, PacketZcNotifyTime, PacketZcNotifyChat, PacketCaLogin, PacketAcAcceptLogin2, PacketAcRefuseLoginR2, PacketAcRefuseLoginR3, PacketChEnter, PacketHcRefuseEnter, PacketChMakeChar2, PacketChDeleteChar2, PacketHcDeleteChar3Reserved, PacketChDeleteChar4Reserved};
use std::sync::{Arc, Mutex};
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
use crate::server::char::{handle_char_enter, handle_make_char, handle_delete_reserved_char};
use std::net::Shutdown::Both;

pub struct Server {
    pub server_context: Arc<Mutex<ServerContext>>,
    pub repository: Arc<Repository<MySql>>,
}

pub enum FeatureState {
    Implemented(Box<dyn Packet>),
    Unimplemented,
}

pub struct ServerContext {
    pub sessions: HashMap<u32, Session>,
}

pub trait SessionsIter {
    fn find_by_stream(&self, tcpStream: &TcpStream) -> Option<u32>;
}

impl SessionsIter for HashMap<u32, Session> {
    fn find_by_stream(&self, tcpStream: &TcpStream) -> Option<u32> {
        let map_entry_option = self.iter().find(|(_, value)| {
            if value.char_server_socket.as_ref().is_none() {
                return false
            }
            let char_server_socket = value.char_server_socket.as_ref().unwrap().lock().unwrap();
            let is_char_stream = char_server_socket.peer_addr().unwrap() == tcpStream.peer_addr().unwrap();
            let mut is_map_stream = false;
            if value.map_server_socket.as_ref().is_some() {
                let map_server_socket = value.map_server_socket.as_ref().unwrap().lock().unwrap();
                is_map_stream = map_server_socket.peer_addr().unwrap() == tcpStream.peer_addr().unwrap();
            }
            is_char_stream || is_map_stream
        });
        if map_entry_option.is_none(){
            return None;
        }
        Some(map_entry_option.unwrap().0.clone())
    }
}

pub struct Session {
    pub char_server_socket: Option<Arc<Mutex<TcpStream>>>,
    pub map_server_socket: Option<Arc<Mutex<TcpStream>>>,
    pub account_id: u32,
    pub login_id1: i32,
    pub login_id2: u32,
}

impl Session {
    pub fn set_char_server_socket(&mut self, tcpStream: Arc<Mutex<TcpStream>>) {
        self.char_server_socket = Some(tcpStream);
    }
    pub fn set_map_server_socket(&mut self, tcpStream: Arc<Mutex<TcpStream>>) {
        self.map_server_socket = Some(tcpStream);
    }
}

impl Server {
    pub fn new(server_context: Arc<Mutex<ServerContext>>, repository: Arc<Repository<MySql>>) -> Server {
        let server = Server {
            server_context,
            repository,
        };
        server.start_tick();
        server
    }

    pub fn start_tick(&self) {
        let server_context_ref = self.server_context.clone();
        thread::Builder::new().name("main tick thread".to_string()).spawn(move || {
            loop {
                let server_context_guard = server_context_ref.lock().unwrap();
                for session in server_context_guard.sessions.values() {
                    if session.map_server_socket.as_ref().is_none() {
                        continue;
                    }
                    let mut tcp_stream_guard = session.map_server_socket.as_ref().unwrap().lock().unwrap();
                    let mut chat = PacketZcNotifyChat {
                        raw: vec![],
                        packet_id: 141,
                        packet_id_raw: [0x8D, 0x00],
                        packet_length: 23,
                        packet_length_raw: [0; 2],
                        gid: 2000001,
                        gid_raw: [0; 4],
                        msg: "walkira: qwertz".to_string(),
                        msg_raw: vec![],
                    };
                    chat.fill_raw();
                    // println!("Send {:02X?} to {}", buffer, session.account_id);
                    println!("{:02X?}", chat.raw());
                    tcp_stream_guard.write(&chat.raw());
                    tcp_stream_guard.flush();
                    drop(tcp_stream_guard);
                }
                drop(server_context_guard);
                sleep(Duration::new(2, 0));
            }
        });
    }

    pub fn dispatch(&self, runtime: &Runtime, tcp_stream: Arc<Mutex<TcpStream>>, packet: &mut dyn Packet) -> FeatureState {
        packet.pretty_debug();
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
        if packet.as_any().downcast_ref::<PacketChDeleteChar4Reserved>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_delete_reserved_char(self, packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        println!("{}", packet.id());
        // Char creation
        FeatureState::Unimplemented
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<Mutex<TcpStream>>) -> Option<u32> {
        let server_context_guard = self.server_context.lock().unwrap();
        let stream_guard = tcp_stream.lock().unwrap();
        let session_option = server_context_guard.sessions.find_by_stream(&stream_guard);
        if session_option.is_none() {
            // TODO uncomment below. keep it comment while we need to proxy data to hercules, so until forever
            // stream_guard.shutdown(Both);
            return None
        }
        Some(session_option.unwrap())
    }
}