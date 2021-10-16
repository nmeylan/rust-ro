// This is the core of this server. As all feature won't be implemented in one shot, the idea is to proxy unimplemented feature to hercules server.
// For the moment I won't implement TCPListener in this file, but in the "future" proxies will be removed and only this file will have a TCPListener.

use std::cmp;
use crate::packets::packets::{Packet, PacketUnknown, PacketCaLogin, PacketChEnter, PacketChMakeChar2, PacketChDeleteChar4Reserved, PacketCzEnter2, PacketChSelectChar, PacketCzRestart, PacketCzReqDisconnect2, PacketCzRequestMove2, PacketCzNotifyActorinit, PacketCzBlockingPlayCancel, PacketZcLoadConfirm, PacketZcNotifyStandentry6, PacketZcNotifyVanish};
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread::{spawn, JoinHandle};
use crate::repository::lib::Repository;
use sqlx::{MySql};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::runtime::Runtime;
use crate::server::login::handle_login;
use crate::server::char::{handle_char_enter, handle_make_char, handle_delete_reserved_char, handle_select_char, handle_enter_game, handle_restart, handle_disconnect, handle_char_loaded_client_side};
use crate::server::movement::{handle_char_move, Position};
use crate::server::map::{Map, MapItem};
use accessor::Setters;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener, Shutdown};
use log::{error};
use rand::{Rng};
use crate::packets::packets_parser::parse;
use crate::{read_lock, socket_send, write_lock};
use crate::util::coordinate;
use crate::util::string::StringUtil;

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
    pub movement_task_id: Option<u128>,
    pub map_view: HashMap<usize, Arc<dyn MapItem>>
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

    pub fn get_map_item_at(&self, x: u16, y: u16) -> Option<&Arc<MapItem>> {
        coordinate::get_item_at(x, y, PLAYER_FOV , &self.map_view)
    }
    pub fn set_map_item_at(&mut self, x: u16, y: u16, item: Arc<dyn MapItem>) {
        let i = coordinate::get_cell_index_of(x, y, PLAYER_FOV);
        self.map_view.insert(i, item);
    }

    pub fn load_units_in_fov(&mut self, map: &Map, session_guard: &RwLockReadGuard<Session>) {
        let old_map_view = self.map_view.clone();
        self.map_view.clear();
        let start_x = cmp::max(self.current_position.x - PLAYER_FOV, 0);
        let end_x = cmp::min(self.current_position.x + PLAYER_FOV, map.x_size);
        let start_y = cmp::max(self.current_position.y - PLAYER_FOV, 0);
        let end_y = cmp::min(self.current_position.y + PLAYER_FOV, map.y_size);
        for x in start_x..end_x {
            for y in start_y..end_y {
                if map.is_warp_cell(x, y) {
                    let warp = map.get_warp_at(x, y).unwrap();
                    if coordinate::get_item_at(warp.x, warp.y, PLAYER_FOV, &old_map_view).is_none() {
                        let mut warp_name = [0 as char; 24];
                        warp.name.fill_char_array(warp_name.as_mut());
                        let mut packet_zc_notify_standentry = PacketZcNotifyStandentry6::new();
                        packet_zc_notify_standentry.set_job(warp.client_item_class());
                        packet_zc_notify_standentry.set_packet_length(108);
                        packet_zc_notify_standentry.set_objecttype(6);
                        packet_zc_notify_standentry.set_aid(warp.id());
                        packet_zc_notify_standentry.set_pos_dir(Position { x: warp.x, y: warp.y, dir: 0 }.to_pos());
                        packet_zc_notify_standentry.set_name(warp_name);
                        packet_zc_notify_standentry.fill_raw();

                        let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                        socket_send!(tcp_stream, packet_zc_notify_standentry.raw());
                    }
                    self.set_map_item_at(warp.x, warp.y, warp as Arc<dyn MapItem>);
                }
            }
        }
        let items = old_map_view.keys().map(|k| *k).collect::<Vec<usize>>();
        for item in items {
            if !self.map_view.contains_key(&item) {
                let mut packet_zc_notify_vanish = PacketZcNotifyVanish::new();
                packet_zc_notify_vanish.set_gid(old_map_view.get(&item).unwrap().id());
                packet_zc_notify_vanish.fill_raw();
                let tcp_stream = session_guard.map_server_socket.as_ref().unwrap();
                socket_send!(tcp_stream, packet_zc_notify_vanish.raw());
            }
        }
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