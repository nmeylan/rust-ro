use std::any::Any;
use packets::packets::{Packet, PacketUnknown, PacketCaLogin, PacketChEnter, PacketChMakeChar2, PacketChDeleteChar4Reserved, PacketCzEnter2, PacketChSelectChar, PacketCzRestart, PacketCzReqDisconnect2, PacketCzRequestMove2, PacketCzNotifyActorinit, PacketCzBlockingPlayCancel, PacketCzRequestAct2, PacketCzReqnameall2, PacketCzPlayerChat, PacketChMakeChar3, PacketChMakeChar, PacketCzRequestMove, PacketCzReqname, PacketCzRequestTime, PacketZcNotifyTime, PacketCzContactnpc, PacketCzReqNextScript, PacketCzChooseMenu, PacketCzInputEditdlg, PacketCzInputEditdlgstr, PacketCzAckSelectDealtype};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread::{JoinHandle};
use crate::repository::lib::Repository;
use sqlx::{MySql};
use std::collections::HashMap;
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
use crate::server::handler::char::{handle_blocking_play_cancel, handle_char_enter, handle_delete_reserved_char, handle_disconnect, handle_enter_game, handle_make_char, handle_restart, handle_select_char};
use crate::server::handler::login::handle_login;
use crate::server::handler::movement::handle_char_move;
use lazy_static::lazy_static;
use crate::server::enums::map_item::MapItemType;
use std::cell::RefCell;
use crate::server::handler::atcommand::handle_atcommand;
use crate::server::handler::map::{handle_char_loaded_client_side, handle_map_item_name};
use crate::util::tick::get_tick;
use std::io::Write;
use rathena_script_lang_interpreter::lang::vm::Vm;
use crate::server::handler::action::npc::{handle_contact_npc, handle_player_choose_menu, handle_player_input_number, handle_player_input_string, handle_player_next, handle_player_select_deal_type};

// Todo make this configurable
pub const PLAYER_FOV: u16 = 14;
pub const PLAYER_FOV_SLICE_LEN: usize = ((PLAYER_FOV * 2) * (PLAYER_FOV * 2)) as usize;
pub const MOB_FOV: u16 = 14;
pub const MOB_FOV_SLICE_LEN: usize = ((MOB_FOV * 2) * (MOB_FOV * 2)) as usize;

lazy_static! {
    pub static ref UNKNOWN_MAP_ITEM: Arc<dyn MapItem> = Arc::new(UnknownMapItem {});
}
thread_local!(pub static PACKETVER: RefCell<u32> = RefCell::new(0));

#[derive(Clone)]
pub struct Server {
    pub configuration: Config,
    pub sessions: Arc<RwLock<HashMap<u32, Arc<Session>>>>,
    pub repository: Arc<Repository>,
    pub maps: HashMap<String, Arc<Map>>,
    pub map_items: Arc<RwLock<HashMap<u32, Arc<dyn MapItem>>>>,
    pub vm: Arc<Vm>,
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
        MapItemType::Unknown.value()
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
    pub fn get_session(&self, session_id: u32) -> Arc<Session> {
        let sessions = self.sessions.read().unwrap();
        let session_ref = sessions.get(&session_id).unwrap();
        session_ref.clone()
    }

    pub fn get_map_socket_for_char_id(&self, char_id: u32) -> Option<Arc<RwLock<TcpStream>>> {
        let sessions = self.sessions.read().unwrap();
        let maybe_session = sessions.iter().find(|(_, session)| {
            return if let Some(char) = session.character.as_ref() {
                char.char_id == char_id
            } else {
                false
            };
        });
        if let Some((_, session)) = maybe_session {
            session.map_server_socket.clone()
        } else {
            None
        }
    }

    pub fn get_map_socket_for_char_with_name(&self, name: &String) -> Option<Arc<RwLock<TcpStream>>> {
        let sessions = self.sessions.read().unwrap();
        let maybe_session = sessions.iter().find(|(_, session)| {
            return if let Some(char) = session.character.as_ref() {
                char.name.eq(name)
            } else {
                false
            };
        });
        if let Some((_, session)) = maybe_session {
            session.map_server_socket.clone()
        } else {
            None
        }
    }

    pub fn packetver(&self) -> u32 {
        self.configuration.server.packetver
    }
}

impl Server {
    pub fn new(configuration: Config, repository: Arc<Repository>, maps: HashMap<String, Arc<Map>>, map_items: Arc<RwLock<HashMap<u32, Arc<dyn MapItem>>>>, vm: Arc<Vm>) -> Server {
        Server {
            configuration,
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<Session>>::new())),
            repository,
            maps,
            map_items,
            vm
        }
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
        let port = server_ref.configuration.server.port;
        Server::listen(server_ref, port)
    }

    fn listen(server_ref: Arc<Server>, port: u16) -> JoinHandle<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
        info!("Server listen on 0.0.0.0:{}", port);
        let server_shared_ref = server_ref;
        thread::Builder::new().name("server-incoming-connection".to_string()).spawn(move || {
            for tcp_stream in listener.incoming() {
                // Receive new connection, starting new thread
                let server_shared_ref = server_shared_ref.clone();
                debug!("Received new connection");
                thread::Builder::new().name("server-handle-request".to_string()).spawn(move || {
                    PACKETVER.with(|ver| *ver.borrow_mut() = server_shared_ref.packetver());
                    let runtime = Runtime::new().unwrap();
                    let mut tcp_stream = tcp_stream.unwrap();
                    let tcp_stream_arc = Arc::new(RwLock::new(tcp_stream.try_clone().unwrap())); // todo remove this clone
                    let mut buffer = [0; 2048];
                    loop {
                        match tcp_stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                if bytes_read == 0 {
                                    tcp_stream.shutdown(Shutdown::Both).expect("Unable to shutdown incoming socket. Shutdown was done because remote socket seems cloded.");
                                    break;
                                }
                                let mut packet = parse(&mut buffer[..bytes_read], server_shared_ref.packetver());
                                server_shared_ref.dispatch(server_shared_ref.clone(), &runtime, tcp_stream_arc.clone(), packet.as_mut());
                            }
                            Err(err) => error!("{}", err)
                        }
                    }
                }).expect("Failed to create sever-handle-request thread");
            }
        }).expect("Failed to create sever-incoming-connection thread")
    }


    pub fn dispatch(&self, server: Arc<Server>, runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, packet: &mut dyn Packet) {
        let self_ref = server;
        if packet.as_any().downcast_ref::<PacketUnknown>().is_some() {
            error!("Unknown packet {} of length {}: {:02X?}", packet.id(), packet.raw().len(), packet.raw());
        }
        // Login
        if packet.as_any().downcast_ref::<PacketCaLogin>().is_some() {
            debug!("PacketCaLogin");
            return handle_login(self_ref, packet, runtime, tcp_stream);
        }
        // Char selection
        if packet.as_any().downcast_ref::<PacketChEnter>().is_some() {
            debug!("PacketChEnter");
            return handle_char_enter(self_ref, packet, runtime, tcp_stream);
        }

        // Enter game
        if packet.as_any().downcast_ref::<PacketCzEnter2>().is_some() {
            debug!("PacketCzEnter2");
            // A char session exist, but not yet map session
            return handle_enter_game(self_ref, packet, tcp_stream);
        }
        /*
         *  Having a session is required for any packets below
         */
        let session_id = self.ensure_session_exists(&tcp_stream);
        if session_id.is_none() {
            return;
        }
        let session = self_ref.get_session(session_id.unwrap());
        // Char creation
        if packet.as_any().downcast_ref::<PacketChMakeChar>().is_some() {
            debug!("PacketChMakeChar");
            return handle_make_char(self_ref, packet, runtime, tcp_stream, session);
        }
        if packet.as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
            debug!("PacketChMakeChar2");
            return handle_make_char(self_ref, packet, runtime, tcp_stream, session);
        }
        if packet.as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
            debug!("PacketChMakeChar3");
            return handle_make_char(self_ref, packet, runtime, tcp_stream, session);
        }
        // Delete char reservation
        if packet.as_any().downcast_ref::<PacketChDeleteChar4Reserved>().is_some() {
            debug!("PacketChDeleteChar4Reserved");
            return handle_delete_reserved_char(self_ref, packet, runtime, tcp_stream, session);
        }
        // Select char
        if packet.as_any().downcast_ref::<PacketChSelectChar>().is_some() {
            debug!("PacketChSelectChar");
            return handle_select_char(self_ref, packet, runtime, tcp_stream, session);
        }
        // Game menu "Character select"
        if packet.as_any().downcast_ref::<PacketCzRestart>().is_some() {
            debug!("PacketCzRestart");
            return handle_restart(self_ref, packet, tcp_stream, session);
        }
        // Game menu "Exit to windows"
        if packet.as_any().downcast_ref::<PacketCzReqDisconnect2>().is_some() {
            debug!("PacketCzReqDisconnect2");
            return handle_disconnect(self_ref, tcp_stream, session);
        }
        // Player click on map cell
        if packet.as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
            debug!("PacketCzRequestMove2");
            return handle_char_move(self_ref, packet, runtime, tcp_stream, session);
        }
        if packet.as_any().downcast_ref::<PacketCzRequestMove>().is_some() {
            debug!("PacketCzRequestMove");
            return handle_char_move(self_ref, packet, runtime, tcp_stream, session);
        }
        // Client notify player has been loaded
        if packet.as_any().downcast_ref::<PacketCzNotifyActorinit>().is_some() {
            debug!("PacketCzNotifyActorinit");
            return handle_char_loaded_client_side(self_ref, tcp_stream, session);
        }
        // Client send PACKET_CZ_BLOCKING_PLAY_CANCEL after char has loaded
        if packet.as_any().downcast_ref::<PacketCzBlockingPlayCancel>().is_some() {
            debug!("PacketCzBlockingPlayCancel");
            return handle_blocking_play_cancel(tcp_stream);
        }
        if packet.as_any().downcast_ref::<PacketCzRequestAct2>().is_some() {
            debug!("PacketCzRequestAct2");
            return handle_attack(packet, runtime, tcp_stream, session);
        }

        if packet.as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
            debug!("PacketCzReqnameall2");
            return handle_map_item_name(self_ref, packet, tcp_stream);
        }

        if packet.as_any().downcast_ref::<PacketCzReqname>().is_some() {
            debug!("PacketCzReqname");
            return handle_map_item_name(self_ref, packet, tcp_stream);
        }

        if packet.as_any().downcast_ref::<PacketCzPlayerChat>().is_some() {
            debug!("PacketCzPlayerChat");
            let packet_player_char = cast!(packet, PacketCzPlayerChat);
            if packet_player_char.msg.starts_with(format!("{} : @", session.character.as_ref().unwrap().name).as_str()) { // TODO make symbol configurable
                return handle_atcommand(self_ref, packet_player_char, runtime, tcp_stream, session);
            }
        }

        if packet.as_any().downcast_ref::<PacketCzRequestTime>().is_some() {
            let mut packet_zc_notify_time = PacketZcNotifyTime::new();
            packet_zc_notify_time.set_time(get_tick());
            packet_zc_notify_time.fill_raw();
            socket_send!(tcp_stream, packet_zc_notify_time.raw());
        }

        // NPC interactions
        if packet.as_any().downcast_ref::<PacketCzContactnpc>().is_some() {
            debug!("PacketCzContactnpc");
            return handle_contact_npc(self_ref, packet, tcp_stream, session);
        }

        if packet.as_any().downcast_ref::<PacketCzReqNextScript>().is_some() {
            debug!("PacketCzReqNextScript");
            return handle_player_next(session);
        }
        if packet.as_any().downcast_ref::<PacketCzChooseMenu>().is_some() {
            debug!("PacketCzChooseMenu");
            return handle_player_choose_menu(packet, session);
        }
        if packet.as_any().downcast_ref::<PacketCzInputEditdlg>().is_some() {
            debug!("PacketCzInputEditdlg");
            return handle_player_input_number(packet, session);
        }
        if packet.as_any().downcast_ref::<PacketCzInputEditdlgstr>().is_some() {
            debug!("PacketCzInputEditdlgstr");
            return handle_player_input_string(packet, session);
        }
        if packet.as_any().downcast_ref::<PacketCzAckSelectDealtype>().is_some() {
            debug!("PacketCzAckSelectDealtype");
            return handle_player_select_deal_type(packet, session);
        }
        // End NPC interaction
        if packet.as_any().downcast_ref::<PacketCzRequestTime>().is_some() {
            return;
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
            debug!("Session does not exist! for socket {:?}", stream_guard);
            return None
        }
        Some(session_option.unwrap())
    }
}