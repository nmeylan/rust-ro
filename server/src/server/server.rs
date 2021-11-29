use std::any::Any;
use packets::packets::{Packet, PacketUnknown, PacketCaLogin, PacketChEnter, PacketChMakeChar2, PacketChDeleteChar4Reserved, PacketCzEnter2, PacketChSelectChar, PacketCzRestart, PacketCzReqDisconnect2, PacketCzRequestMove2, PacketCzNotifyActorinit, PacketCzBlockingPlayCancel, PacketCzRequestAct2, PacketCzReqnameall2};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread::{JoinHandle};
use crate::repository::lib::Repository;
use sqlx::{MySql};
use std::collections::HashMap;
use std::future::{poll_fn};
use tokio::runtime::Runtime;
use std::io::{Read};
use std::net::{TcpStream, TcpListener, Shutdown};
use std::thread;
use log::{error};
use rand::{Rng};
use packets::packets_parser::parse;
use crate::server::configuration::Config;
use crate::server::core::map::{Map, MapItem};
use crate::server::core::session::{Session, SessionsIter};
use crate::server::handler::action::attack::handle_attack;
use crate::server::handler::char::{handle_blocking_play_cancel, handle_char_enter, handle_char_loaded_client_side, handle_delete_reserved_char, handle_disconnect, handle_enter_game, handle_make_char, handle_restart, handle_select_char};
use crate::server::handler::login::handle_login;
use crate::server::handler::movement::handle_char_move;
use lazy_static::lazy_static;
use crate::server::handler::map::handle_map_item_name;

// Todo make this configurable
pub const PLAYER_FOV: u16 = 14;
pub const PLAYER_FOV_SLICE_LEN: usize = ((PLAYER_FOV * 2) * (PLAYER_FOV * 2)) as usize;
pub const MOB_FOV: u16 = 14;
pub const MOB_FOV_SLICE_LEN: usize = ((MOB_FOV * 2) * (MOB_FOV * 2)) as usize;

lazy_static! {
    pub static ref UNKNOWN_MAP_ITEM: Arc<dyn MapItem> = Arc::new(UnknownMapItem {});
}

#[derive(Clone)]
pub struct Server {
    pub configuration: Config,
    pub sessions: Arc<RwLock<HashMap<u32, Arc<Session>>>>,
    pub repository: Arc<Repository<MySql>>,
    pub maps: HashMap<String, Arc<Map>>,
    pub map_items: Arc<RwLock<HashMap<u32, Arc<dyn MapItem>>>>
}

pub struct UnknownMapItem;

impl MapItem for UnknownMapItem {
    fn id(&self) -> u32 {
        0
    }
    fn x(&self) -> u16 {
        0
    }
    fn y(&self) -> u16 {
        0
    }

    fn client_item_class(&self) -> i16 {
        0
    }

    fn object_type(&self) -> i16 {
        0
    }

    fn name(&self) -> String {
        String::from("unknown")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Server {
    pub fn remove_session(&self, session_id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&session_id);
    }
}

impl Server {
    pub fn new(configuration: Config, repository: Arc<Repository<MySql>>, maps: HashMap<String, Arc<Map>>, map_items: Arc<RwLock<HashMap<u32, Arc<dyn MapItem>>>>) -> Server {
        let server = Server {
            configuration,
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<Session>>::new())),
            repository,
            maps,
            map_items
        };
        server
    }

    pub fn generate_map_item_id(&self) -> u32 {
        let mut map_items = write_lock!(self.map_items);
        Server::generate_id(&mut map_items)
    }

    pub fn insert_map_item(&self, id: u32, map_item: Arc<dyn MapItem>) {
        let mut map_items = write_lock!(self.map_items);
        map_items.insert(id, map_item);
    }

    pub fn generate_id(map_items: &mut RwLockWriteGuard<HashMap<u32, Arc<dyn MapItem>>>) -> u32 {
        let mut id: u32;
        loop {
            id = rand::thread_rng().gen::<u32>();
            if !map_items.contains_key(&id) {
                map_items.insert(id, UNKNOWN_MAP_ITEM.clone());
                break;
            }
        }
        id
    }

    pub fn start(server_ref: Arc<Server>) -> JoinHandle<()> {
        let port = server_ref.configuration.server.port.clone();
        Server::listen(server_ref, port)
    }

    fn listen(server_ref: Arc<Server>, port: u16) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        listener.set_nonblocking(true);
        let listener = tokio::net::TcpListener::from_std(listener).unwrap();
        info!("Server listen on 0.0.0.0:{}", port);
        let server_shared_ref = server_ref.clone();
        thread::Builder::new().name("server-incoming-connection".to_string()).spawn(move || {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(async {
                loop {
                    let poll_result = poll_fn(|cx| listener.poll_accept(cx)).await;
                    if poll_result.is_err() {
                        break;
                    }
                    let io_result = poll_result.unwrap();
                    let stream = io_result.0;
                    let mut tcp_stream = stream.into_std().unwrap();
                    // Receive new connection, starting new thread
                    let server_shared_ref = server_shared_ref.clone();
                    thread::Builder::new().name("server-handle-request".to_string()).spawn(move || {
                        let runtime = Runtime::new().unwrap();
                        let tcp_stream_arc = Arc::new(RwLock::new(tcp_stream.try_clone().unwrap())); // todo remove this clone
                        let mut buffer = [0; 2048];
                        loop {
                            match tcp_stream.read(&mut buffer) {
                                Ok(bytes_read) => {
                                    if bytes_read == 0 {
                                        tcp_stream.shutdown(Shutdown::Both).expect("Unable to shutdown incoming socket. Shutdown was done because remote socket seems cloded.");
                                        break;
                                    }
                                    let mut packet = parse(&mut buffer[..bytes_read]);
                                    server_shared_ref.dispatch(server_shared_ref.clone(), &runtime, tcp_stream_arc.clone(), packet.as_mut());
                                }
                                Err(err) => ()
                            }
                        }
                    }).expect("Failed to create sever-handle-request thread");
                }
            });
            info!("Server shutdown");
        }).expect("Failed to create sever-incoming-connection thread")
    }


    pub fn dispatch(&self, server: Arc<Server>, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, packet: &mut dyn Packet) {
        let self_ref = server;
        if packet.as_any().downcast_ref::<PacketUnknown>().is_some() {
            error!("Unknown packet {} of length {}: {:02X?}", packet.id(), packet.raw().len(), packet.raw());
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
            let session_id = self.ensure_session_exists(&tcp_stream);
            return handle_blocking_play_cancel(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
        }
        if packet.as_any().downcast_ref::<PacketCzRequestAct2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_attack(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }

        if packet.as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
            let session_id = self.ensure_session_exists(&tcp_stream);
            if session_id.is_some() {
                return handle_map_item_name(self_ref.clone(), packet, runtime, tcp_stream, session_id.unwrap());
            }
        }

        if packet.id() == "0x6003" // PacketCzRequestTime2
        {
            // TODO handle those packets
            return;
        }
        packet.display();
        packet.pretty_debug();
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<RwLock<TcpStream>>) -> Option<u32> {
        let session_guard = read_lock!(self.sessions);
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