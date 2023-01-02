use std::sync::{Arc, RwLock};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::collections::HashMap;
use rathena_script_lang_interpreter::lang::vm::Vm;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::thread::Scope;
use tokio::runtime::Runtime;
use packets::packets::{Packet, PacketCaLogin, PacketChDeleteChar4Reserved, PacketChEnter, PacketChMakeChar, PacketChMakeChar2, PacketChMakeChar3, PacketChSelectChar, PacketCzAckSelectDealtype, PacketCzBlockingPlayCancel, PacketCzChooseMenu, PacketCzContactnpc, PacketCzEnter2, PacketCzInputEditdlg, PacketCzInputEditdlgstr, PacketCzNotifyActorinit, PacketCzPcPurchaseItemlist, PacketCzPlayerChat, PacketCzReqDisconnect2, PacketCzReqname, PacketCzReqnameall2, PacketCzReqNextScript, PacketCzRequestAct
                       , PacketCzRequestMove, PacketCzRequestMove2, PacketCzRequestTime, PacketCzRestart, PacketCzUseItem, PacketUnknown, PacketZcNotifyTime};
use packets::packets_parser::parse;
use std::io::{Read, Write};
use crate::repository::Repository;
use crate::server::core::configuration::{Config};
use crate::server::core::map::{Map, MAP_EXT};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::map_item::MapItem;
use crate::server::core::path::manhattan_distance;
use crate::server::core::request::Request;
use crate::server::core::response::Response;
use crate::server::core::session::{Session, SessionsIter};
use crate::server::core::tasks_queue::TasksQueue;
use crate::server::events::client_notification::{AreaNotificationRangeType, Notification};
use crate::server::events::game_event::GameEvent;
use crate::server::events::persistence_event::PersistenceEvent;
use crate::server::handler::action::action::handle_action;
use crate::server::handler::action::npc::{handle_contact_npc, handle_player_choose_menu, handle_player_input_number, handle_player_input_string, handle_player_next, handle_player_purchase_items, handle_player_select_deal_type};
use crate::server::handler::char::{handle_blocking_play_cancel, handle_char_enter, handle_delete_reserved_char, handle_disconnect, handle_enter_game, handle_make_char, handle_restart, handle_select_char};
use crate::server::handler::chat::handle_chat;
use crate::server::handler::login::handle_login;
use crate::server::handler::map::{handle_char_loaded_client_side, handle_map_item_name};
use crate::server::handler::movement::handle_char_move;
use crate::server::map_item::ToMapItem;
use crate::server::state::character::Character;
use crate::util::cell::{MyRef, MyUnsafeCell};
use crate::util::tick::get_tick_client;
use std::cell::RefCell;
use crate::server::handler::action::item::handle_player_use_item;

pub mod npc;
pub mod handler;
pub mod core;
pub mod script;
mod game_loop;
pub mod map_item;
pub mod persistence;
pub mod service;
pub mod events;
pub mod state;

thread_local!(pub static PACKETVER: RefCell<u32> = RefCell::new(0));
// Todo make this configurable
pub const PLAYER_FOV: u16 = 14;
pub const MOB_FOV: u16 = 14;
pub const UNKNOWN_MAP_ITEM: MapItem = MapItem::unknown();

pub struct Server {
    pub configuration: Config,
    pub sessions: Arc<RwLock<HashMap<u32, Arc<Session>>>>,
    pub repository: Arc<Repository>,
    pub maps: HashMap<String, Arc<Map>>,
    map_items: MyUnsafeCell<HashMap<u32, MapItem>>,

    pub characters: MyUnsafeCell<HashMap<u32, Character>>,
    tasks_queue: TasksQueue<GameEvent>,
    movement_tasks_queue: TasksQueue<GameEvent>,
    pub vm: Arc<Vm>,
    client_notification_sender: SyncSender<Notification>,
}

unsafe impl Sync for Server {}

unsafe impl Send for Server {}

impl Server {
    pub fn remove_session(&self, session_id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&session_id);
    }
    fn get_session(&self, session_id: u32) -> Arc<Session> {
        let sessions = self.sessions.read().unwrap();
        let session_ref = sessions.get(&session_id).unwrap();
        session_ref.clone()
    }

    pub(crate) fn pop_task(&self) -> Option<Vec<GameEvent>> {
        self.tasks_queue.pop()
    }

    pub(crate) fn pop_movement_task(&self) -> Option<Vec<GameEvent>> {
        self.movement_tasks_queue.pop()
    }

    pub fn get_character_unsafe(&self, char_id: u32) -> MyRef<Character> {
        MyRef::map(self.characters.borrow(), |characters| characters.get(&char_id).unwrap())
    }

    pub fn get_character_from_context_unsafe(&self, context: &Request) -> MyRef<Character> {
        let char_id = context.session().char_id.unwrap();
        MyRef::map(self.characters.borrow(), |characters| characters.get(&char_id).unwrap())
    }

    #[inline]
    pub fn get_map_instance_from_character(&self, character: &Character) -> Option<Arc<MapInstance>> {
        self.get_map_instance(character.current_map_name(), character.current_map_instance())
    }
    #[inline]
    pub fn get_map_instance(&self, map_name: &String, map_instance_id: u8) -> Option<Arc<MapInstance>> {
        let map_name = if map_name.ends_with(MAP_EXT) {
            &map_name[..(map_name.len() - 4)]
        } else {
            map_name.as_str()
        };

        if let Some(map) = self.maps.get(map_name) {
            return map.get_instance(map_instance_id, self);
        }
        error!("Can't find map instance {}:{}",map_name, map_instance_id);
        None
    }

    pub fn insert_character(&self, character: Character) {
        self.map_items.borrow_mut().insert(character.char_id, character.to_map_item());
        self.characters.borrow_mut().insert(character.char_id, character);
    }

    pub fn get_map_socket_for_char_id(&self, char_id: u32) -> Option<Arc<RwLock<TcpStream>>> {
        let account_id = self.get_character_unsafe(char_id).account_id;
        let sessions = self.sessions.read().unwrap();
        let maybe_session = sessions.get(&account_id);
        if let Some(session) = maybe_session {
            session.map_server_socket.clone()
        } else {
            None
        }
    }

    pub fn packetver(&self) -> u32 {
        self.configuration.server.packetver
    }

    pub fn new(configuration: Config, repository: Arc<Repository>, maps: HashMap<String, Arc<Map>>, map_items: MyUnsafeCell<HashMap<u32, MapItem>>, vm: Arc<Vm>, client_notification_sender: SyncSender<Notification>) -> Server {
        Server {
            configuration,
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<Session>>::new())),
            repository,
            maps,
            map_items,
            characters: Default::default(),
            tasks_queue: TasksQueue::new(),
            movement_tasks_queue: TasksQueue::new(),
            vm,
            client_notification_sender,
        }
    }

    pub fn add_to_next_tick(&self, event: GameEvent) {
        self.tasks_queue.add_to_first_index(event)
    }

    pub fn add_to_tick(&self, event: GameEvent, index: usize) {
        self.tasks_queue.add_to_index(event, index)
    }

    pub fn add_to_next_movement_tick(&self, event: GameEvent) {
        self.movement_tasks_queue.add_to_first_index(event)
    }

    pub fn insert_map_item(&self, id: u32, map_item: MapItem) {
        self.map_items.borrow_mut().insert(id, map_item);
    }

    pub fn client_notification_sender(&self) -> SyncSender<Notification> {
        self.client_notification_sender.clone()
    }
    #[allow(unused_lifetimes)]
    pub fn start<'server>(server_ref: Arc<Server>, single_client_notification_receiver: Receiver<Notification>) {
        let port = server_ref.configuration.server.port;

        let (persistence_event_sender, persistence_event_receiver) = std::sync::mpsc::sync_channel::<PersistenceEvent>(0);
        let (response_sender, single_response_receiver) = std::sync::mpsc::sync_channel::<Response>(0);
        let client_notification_sender_clone = server_ref.client_notification_sender();
        thread::scope(|server_thread_scope: &Scope| {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
            info!("Server listen on 0.0.0.0:{}", port);
            let server_shared_ref = server_ref.clone();

            thread::Builder::new().name("client_connection_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                for tcp_stream in listener.incoming() {
                    // Receive new connection, starting new thread
                    let server_shared_ref = server_shared_ref.clone();
                    debug!("Received new connection");
                    let response_sender_clone = response_sender.clone();
                    let client_notification_sender_clone = client_notification_sender_clone.clone();
                    server_thread_scope.spawn(move || {
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
                                    let packet = parse(&buffer[..bytes_read], server_shared_ref.packetver());
                                    let context = Request::new(&runtime, &server_shared_ref.configuration, None, server_shared_ref.packetver(), tcp_stream_arc.clone(), packet.as_ref(), response_sender_clone.clone(), client_notification_sender_clone.clone());
                                    server_shared_ref.handle(server_shared_ref.clone(), context);
                                }
                                Err(err) => error!("{}", err)
                            }
                        }
                    });
                }
            }).unwrap();
            // Start a thread sending response packet to client request

            thread::Builder::new().name("client_response_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                for response in single_response_receiver.iter() {
                    let tcp_stream = &response.socket();
                    let data = response.serialized_packet();
                    let mut tcp_stream_guard = tcp_stream.write().unwrap();
                    debug!("Respond to {:?} with: {:02X?}", tcp_stream_guard.peer_addr(), data);
                    tcp_stream_guard.write_all(data).unwrap();
                    tcp_stream_guard.flush().unwrap();
                }
            }).unwrap();
            // Start a thread sending packet to notify client from game update
            let server_ref_clone = server_ref.clone();
            thread::Builder::new().name("client_notification_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                PACKETVER.with(|ver| *ver.borrow_mut() = server_ref_clone.packetver());
                let server_ref = server_ref_clone;
                for response in single_client_notification_receiver.iter() {
                    match response {
                        Notification::Char(char_notification) => {
                            if let Some(tcp_stream) = server_ref.get_map_socket_for_char_id(char_notification.char_id()) {
                                let data = char_notification.serialized_packet();
                                let mut tcp_stream_guard = tcp_stream.write().unwrap();
                                if tcp_stream_guard.peer_addr().is_ok() {
                                    debug!("Respond to {:?} with: {:02X?}", tcp_stream_guard.peer_addr(), data);
                                    tcp_stream_guard.write_all(data).unwrap();
                                    tcp_stream_guard.flush().unwrap();
                                } else {
                                    error!("{:?} socket has been closed", tcp_stream_guard.peer_addr().err());
                                }
                            }
                        }
                        Notification::Area(area_notification) => {
                            match area_notification.range_type {
                                AreaNotificationRangeType::Map => {}
                                AreaNotificationRangeType::Fov { x, y } => {
                                    server_ref.characters.borrow().iter()
                                        .filter(|(_, character)| character.current_map_name() == &area_notification.map_name
                                            && character.current_map_instance() == area_notification.map_instance_id
                                            && manhattan_distance(character.x(), character.y(), x, y) <= PLAYER_FOV)
                                        .for_each(|(_, character)| {
                                            let tcp_stream = server_ref.get_map_socket_for_char_id(character.char_id).expect("Expect to found a socket for account");
                                            let data = area_notification.serialized_packet();
                                            let mut tcp_stream_guard = tcp_stream.write().unwrap();
                                            if tcp_stream_guard.peer_addr().is_ok() {
                                                debug!("Area - Respond to {:?} with: {:02X?}", tcp_stream_guard.peer_addr(), data);
                                                tcp_stream_guard.write_all(data).unwrap();
                                                tcp_stream_guard.flush().unwrap();
                                            } else {
                                                error!("{:?} socket has been closed", tcp_stream_guard.peer_addr().err());
                                            }
                                        })
                                }
                            }
                        }
                    }
                }
            }).unwrap();
            let server_ref_clone = server_ref.clone();
            let client_notification_sender_clone = server_ref.client_notification_sender();
            let persistence_event_sender_clone = persistence_event_sender.clone();
            thread::Builder::new().name("game_loop_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                let runtime = Runtime::new().unwrap();
                Self::game_loop(server_ref_clone, client_notification_sender_clone, persistence_event_sender_clone, runtime)
                ;
            }).unwrap();
            let server_ref_clone = server_ref.clone();
            let client_notification_sender_clone = server_ref.client_notification_sender();
            let persistence_event_sender_clone = persistence_event_sender.clone();
            thread::Builder::new().name("movement_loop_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                Self::character_movement_loop(server_ref_clone, client_notification_sender_clone, persistence_event_sender_clone);
            }).unwrap();
            let server_ref_clone = server_ref.clone();
            thread::Builder::new().name("persistence_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                let runtime = Runtime::new().unwrap();
                Self::persistence_thread(persistence_event_receiver, runtime, server_ref_clone.repository.clone());
            }).unwrap();
        });
    }

    pub fn handle(&self, server: Arc<Server>, mut context: Request) {
        let self_ref = server;
        if context.packet().as_any().downcast_ref::<PacketUnknown>().is_some() {
            error!("Unknown packet {} of length {}: {:02X?}", context.packet().id(), context.packet().raw().len(), context.packet().raw());
            return;
        }
        // Login
        if context.packet().as_any().downcast_ref::<PacketCaLogin>().is_some() {
            debug!("PacketCaLogin");
            return handle_login(self_ref, context);
        }
        // Char selection
        if context.packet().as_any().downcast_ref::<PacketChEnter>().is_some() {
            debug!("PacketChEnter");
            return handle_char_enter(self_ref.as_ref(), context);
        }

        // Enter game
        if context.packet().as_any().downcast_ref::<PacketCzEnter2>().is_some() {
            debug!("PacketCzEnter2");
            // A char session exist, but not yet map session
            return handle_enter_game(self_ref.as_ref(), context);
        }
        /*
         *  Having a session is required for any packets below
         */
        let session_id = self.ensure_session_exists(&context.socket());
        if session_id.is_none() {
            return;
        }
        let session = self_ref.get_session(session_id.unwrap());
        context.set_session(session);
        // Char creation
        if context.packet().as_any().downcast_ref::<PacketChMakeChar>().is_some() {
            debug!("PacketChMakeChar");
            return handle_make_char(self_ref.as_ref(), context);
        }
        if context.packet().as_any().downcast_ref::<PacketChMakeChar2>().is_some() {
            debug!("PacketChMakeChar2");
            return handle_make_char(self_ref.as_ref(), context);
        }
        if context.packet().as_any().downcast_ref::<PacketChMakeChar3>().is_some() {
            debug!("PacketChMakeChar3");
            return handle_make_char(self_ref.as_ref(), context);
        }
        // Delete char reservation
        if context.packet().as_any().downcast_ref::<PacketChDeleteChar4Reserved>().is_some() {
            debug!("PacketChDeleteChar4Reserved");
            return handle_delete_reserved_char(self_ref.as_ref(), context);
        }
        // Select char
        if context.packet().as_any().downcast_ref::<PacketChSelectChar>().is_some() {
            debug!("PacketChSelectChar");
            return handle_select_char(self_ref.as_ref(), context);
        }
        // Game menu "Character select"
        if context.packet().as_any().downcast_ref::<PacketCzRestart>().is_some() {
            debug!("PacketCzRestart");
            return handle_restart(self_ref.as_ref(), context);
        }
        // Game menu "Exit to windows"
        if context.packet().as_any().downcast_ref::<PacketCzReqDisconnect2>().is_some() {
            debug!("PacketCzReqDisconnect2");
            return handle_disconnect(self_ref.as_ref(), context);
        }
        // Player click on map cell
        if context.packet().as_any().downcast_ref::<PacketCzRequestMove2>().is_some() {
            debug!("PacketCzRequestMove2");
            return handle_char_move(self_ref.as_ref(), context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzRequestMove>().is_some() {
            debug!("PacketCzRequestMove");
            return handle_char_move(self_ref.as_ref(), context);
        }
        // Client notify player has been loaded
        if context.packet().as_any().downcast_ref::<PacketCzNotifyActorinit>().is_some() {
            debug!("PacketCzNotifyActorinit");
            return handle_char_loaded_client_side(self_ref.as_ref(), context);
        }
        // Client send PACKET_CZ_BLOCKING_PLAY_CANCEL after char has loaded
        if context.packet().as_any().downcast_ref::<PacketCzBlockingPlayCancel>().is_some() {
            debug!("PacketCzBlockingPlayCancel");
            return handle_blocking_play_cancel(context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzRequestAct>().is_some() {
            debug!("PacketCzRequestAct");
            return handle_action(self_ref.as_ref(), context);
        }

        if context.packet().as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
            debug!("PacketCzReqnameall2");
            return handle_map_item_name(self_ref.as_ref(), context);
        }

        if context.packet().as_any().downcast_ref::<PacketCzReqname>().is_some() {
            debug!("PacketCzReqname");
            return handle_map_item_name(self_ref.as_ref(), context);
        }

        if context.packet().as_any().downcast_ref::<PacketCzPlayerChat>().is_some() {
            debug!("PacketCzPlayerChat");
            return handle_chat(self_ref.as_ref(), context);
        }

        // NPC interactions
        if context.packet().as_any().downcast_ref::<PacketCzContactnpc>().is_some() {
            debug!("PacketCzContactnpc");
            return handle_contact_npc(self_ref, context);
        }

        if context.packet().as_any().downcast_ref::<PacketCzReqNextScript>().is_some() {
            debug!("PacketCzReqNextScript");
            return handle_player_next(context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzChooseMenu>().is_some() {
            debug!("PacketCzChooseMenu");
            return handle_player_choose_menu(context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzInputEditdlg>().is_some() {
            debug!("PacketCzInputEditdlg");
            return handle_player_input_number(context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzInputEditdlgstr>().is_some() {
            debug!("PacketCzInputEditdlgstr");
            return handle_player_input_string(context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzAckSelectDealtype>().is_some() {
            debug!("PacketCzAckSelectDealtype");
            return handle_player_select_deal_type(context);
        }
        if context.packet().as_any().downcast_ref::<PacketCzPcPurchaseItemlist>().is_some() {
            debug!("PacketCzPcPurchaseItemlist");
            return handle_player_purchase_items(context);
        }
        // End NPC interaction

        // Item interaction
        if context.packet().as_any().downcast_ref::<PacketCzUseItem>().is_some() {
            debug!("PacketCzUseItem");
            return handle_player_use_item(self_ref.as_ref(), context);
        }
        // End Item interaction

        if context.packet().as_any().downcast_ref::<PacketCzRequestTime>().is_some() {
            let mut packet_zc_notify_time = PacketZcNotifyTime::new();
            packet_zc_notify_time.set_time(get_tick_client());
            packet_zc_notify_time.fill_raw();
            socket_send!(context, packet_zc_notify_time);
        }

        if context.packet().id() == "0x6003" // PacketCzRequestTime2
            || context.packet().id() == "0x187" // PacketPing
        {
            // TODO handle those packets
            return;
        }
        context.packet().display();
        context.packet().pretty_debug();
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<RwLock<TcpStream>>) -> Option<u32> {
        let session_guard = read_lock!(self.sessions);
        let stream_guard = read_lock!(tcp_stream);
        let session_option = session_guard.find_by_stream(&stream_guard);
        if session_option.is_none() {
            // TODO uncomment below. keep it comment while we need to proxy data to hercules, so until forever
            // stream_guard.shutdown(Both);
            debug!("Session does not exist! for socket {:?}", stream_guard);
            return None;
        }
        Some(session_option.unwrap())
    }
}
