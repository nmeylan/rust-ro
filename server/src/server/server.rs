use crate::packets::packets::{Packet, PacketUnknown, PacketCaLogin, PacketChEnter, PacketChMakeChar2, PacketChDeleteChar4Reserved, PacketCzEnter2, PacketChSelectChar, PacketCzRestart, PacketCzReqDisconnect2, PacketCzRequestMove2, PacketCzNotifyActorinit, PacketCzBlockingPlayCancel, PacketZcLoadConfirm, PacketZcNotifyStandentry6, PacketZcNotifyVanish};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread::{spawn, JoinHandle};
use crate::repository::lib::Repository;
use sqlx::{MySql};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::runtime::Runtime;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener, Shutdown};
use log::{error};
use rand::{Rng};
use crate::packets::packets_parser::parse;
use crate::{read_lock, socket_send, write_lock};
use crate::server::core::map::Map;
use crate::server::core::session::{Session, SessionsIter};
use crate::server::handler::char::{handle_char_enter, handle_char_loaded_client_side, handle_delete_reserved_char, handle_disconnect, handle_enter_game, handle_make_char, handle_restart, handle_select_char};
use crate::server::handler::login::handle_login;
use crate::server::handler::movement::handle_char_move;

// Todo make this configurable
pub const PLAYER_FOV: u16 = 14;

pub struct Server {
    pub sessions: Arc<RwLock<HashMap<u32, Arc<RwLock<Session>>>>>,
    pub repository: Arc<Repository<MySql>>,
    pub maps: Arc<RwLock<HashMap<String, Map>>>,
    pub map_item_ids: Arc<RwLock<Vec<u32>>>
}

impl Server {
    pub fn remove_session(&self, session_id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&session_id);
    }
}

impl Server {
    pub fn new(repository: Arc<Repository<MySql>>, maps: Arc<RwLock<HashMap<String, Map>>>, map_item_ids: Arc<RwLock<Vec<u32>>>) -> Server {
        let server = Server {
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<RwLock<Session>>>::new())),
            repository,
            maps,
            map_item_ids
        };
        server
    }

    pub fn generate_map_item_id(&self) -> u32 {
        let mut ids = write_lock!(self.map_item_ids);
        Server::generate_id(&mut ids)
    }

    pub fn generate_id(ids: &mut RwLockWriteGuard<Vec<u32>>) -> u32 {
        let mut id: u32;
        loop {
            id = rand::thread_rng().gen::<u32>();
            if !ids.contains(&id) {
                ids.push(id);
                break;
            }
        }
        id
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
                                server_shared_ref.dispatch(server_shared_ref.clone(), &runtime, tcp_stream_arc.clone(), packet.as_mut());
                            }
                            Err(err) => error!("{}", err)
                        }
                    }
                });
            }
        })
    }


    pub fn dispatch(&self, server: Arc<Server>, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, packet: &mut dyn Packet) {
        let self_ref = server;
        if packet.as_any().downcast_ref::<PacketUnknown>().is_some() {
            println!("Unknown packet {} of length {}: {:02X?}", packet.id(), packet.raw().len(), packet.raw());
        }
        // Login
        if packet.as_any().downcast_ref::<PacketCaLogin>().is_some() {
            return handle_login(self_ref.clone(), packet, runtime, tcp_stream);
        }
        // Char selection
        if packet.as_any().downcast_ref::<PacketChEnter>().is_some() {
            return handle_char_enter(self_ref.clone(), packet, runtime, tcp_stream);
        }
        // Char creation
        if packet.as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_make_char(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Delete char reservation
        if packet.as_any().downcast_ref::<PacketChDeleteChar4Reserved>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_delete_reserved_char(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Select char
        if packet.as_any().downcast_ref::<PacketChSelectChar>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_select_char(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Enter game
        if packet.as_any().downcast_ref::<PacketCzEnter2>().is_some() {
            return handle_enter_game(self_ref.clone(), packet, runtime, tcp_stream);
        }
        // Game menu "Character select"
        if packet.as_any().downcast_ref::<PacketCzRestart>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_restart(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Game menu "Exit to windows"
        if packet.as_any().downcast_ref::<PacketCzReqDisconnect2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_disconnect(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Player click on map cell
        if packet.as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_char_move(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }
        // Client notify player has been loaded
        if packet.as_any().downcast_ref::<PacketCzNotifyActorinit>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_char_loaded_client_side(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
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