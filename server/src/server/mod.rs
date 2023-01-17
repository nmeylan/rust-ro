use std::sync::{Arc, RwLock};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::collections::HashMap;
use rathena_script_lang_interpreter::lang::vm::Vm;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::thread::Scope;
use tokio::runtime::Runtime;
use packets::packets_parser::parse;
use std::io::{Read, Write};
use crate::repository::{ItemRepository, Repository};
use crate::server::core::configuration::{Config};
use crate::server::core::map::{Map, MAP_EXT, RANDOM_CELL};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::map_item::MapItem;
use crate::server::core::path::manhattan_distance;
use crate::server::core::request::Request;
use crate::server::core::response::Response;
use crate::server::core::session::{Session, SessionsIter};
use crate::server::core::tasks_queue::TasksQueue;
use crate::server::events::client_notification::{AreaNotificationRangeType, Notification};
use crate::server::events::game_event::{CharacterAddItems, CharacterChangeMap, CharacterRemoveFromMap, GameEvent};
use crate::server::events::persistence_event::PersistenceEvent;







use crate::server::map_item::ToMapItem;
use crate::server::state::character::Character;
use crate::util::cell::{MyRef, MyUnsafeCell};

use std::cell::RefCell;
use enums::item::ItemType;
use crate::server::core::position::Position;

use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::item_service::ItemService;
use script::skill::SkillService;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::script::Value;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;

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
    pub configuration: &'static Config,
    pub sessions: Arc<RwLock<HashMap<u32, Arc<Session>>>>,
    pub repository: Arc<Repository>,
    pub maps: HashMap<String, Arc<Map>>,
    map_items: MyUnsafeCell<HashMap<u32, MapItem>>,

    pub characters: MyUnsafeCell<HashMap<u32, Character>>,
    tasks_queue: Arc<TasksQueue<GameEvent>>,
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

    pub fn new(configuration: &'static Config, repository: Arc<Repository>, maps: HashMap<String, Arc<Map>>, map_items: MyUnsafeCell<HashMap<u32, MapItem>>, vm: Arc<Vm>, client_notification_sender: SyncSender<Notification>, persistence_event_sender: SyncSender<PersistenceEvent>) -> Server {
        let tasks_queue = Arc::new(TasksQueue::new());
        CharacterService::init(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance());
        InventoryService::init(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance(), tasks_queue.clone());
        ItemService::init(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance());
        SkillService::init(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), configuration);
        StatusService::init(client_notification_sender.clone(), persistence_event_sender, repository.clone(), GlobalConfigService::instance());

        Server {
            configuration,
            sessions: Arc::new(RwLock::new(HashMap::<u32, Arc<Session>>::new())),
            repository,
            maps,
            map_items,
            characters: Default::default(),
            tasks_queue,
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
    pub fn start<'server>(server_ref: Arc<Server>, single_client_notification_receiver: Receiver<Notification>, persistence_event_receiver: Receiver<PersistenceEvent>, persistence_event_sender: SyncSender<PersistenceEvent>) {
        let port = server_ref.configuration.server.port;

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
                                    let context = Request::new(&runtime, server_shared_ref.configuration, None, server_shared_ref.packetver(), tcp_stream_arc.clone(), packet.as_ref(), response_sender_clone.clone(), client_notification_sender_clone.clone());
                                    handler::handle(server_shared_ref.clone(), context);
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
            thread::Builder::new().name("game_loop_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                let runtime = Runtime::new().unwrap();
                Self::game_loop(server_ref_clone, runtime);
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
        })
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

    pub fn schedule_warp_to_walkable_cell(&self, destination_map: &str, x: u16, y: u16, char_id: u32) {
        self.add_to_next_tick(GameEvent::CharacterClearFov(char_id));
        let character_ref = self.get_character_unsafe(char_id);
        self.add_to_tick(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap { char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance() }), 0);
        drop(character_ref);

        let map_name: String = Map::name_without_ext(destination_map);
        debug!("Char enter on map {}", map_name);
        let map_ref = self.maps.get(&map_name).unwrap();
        let map_instance = map_ref.player_join_map(self);
        let (x, y) = if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
            let walkable_cell = Map::find_random_walkable_cell(&map_instance.cells, map_instance.x_size);
            (walkable_cell.0, walkable_cell.1)
        } else {
            (x, y)
        };

        self.add_to_tick(GameEvent::CharacterChangeMap(CharacterChangeMap {
            char_id,
            new_map_name: destination_map.to_owned(),
            new_instance_id: map_instance.id,
            new_position: Some(Position { x, y, dir: 3 }),
        }), 2);
    }

    pub fn schedule_get_items(&self, char_id: u32, runtime: &Runtime, item_ids_amounts: Vec<(Value, i16)>, buy: bool) {
        let mut items = runtime.block_on(async { self.repository.get_items(item_ids_amounts.iter().map(|(v, _)| v.clone()).collect()).await }).unwrap();
        items.iter_mut().for_each(|item| item.amount = item_ids_amounts.iter().find(|(id, _amount)|
            match id {
                Value::Number(v) => *v == item.id,
                Value::String(v) => v.to_lowercase() == item.name_aegis.to_lowercase(),
            }
        ).unwrap().1);
        self.add_to_next_tick(GameEvent::CharacterAddItems(CharacterAddItems {
            char_id,
            should_perform_check: true,
            buy,
            items: items.iter().map(|item| InventoryItemModel {
                id: 0,
                unique_id: 0,
                item_id: item.id,
                item_type: ItemType::from_string(item.item_type.as_str()),
                amount: item.amount,
                weight: item.weight,
                name_english: item.name_english.clone(),
                is_identified: true,
                refine: 0,
                is_damaged: false,
                card0: 0,
                card1: 0,
                card2: 0,
                equip: 0,
                card3: 0,
            }).collect(),
        }));
    }
}
