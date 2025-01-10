use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{mpsc, Arc, RwLock};

use crate::repository::Repository;
use configuration::configuration::Config;
use packets::packets_parser::parse;
use rathena_script_lang_interpreter::lang::vm::Vm;
use std::io::{Read, Write};
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::thread::Scope;
use tokio::runtime::Runtime;


use crate::server::model::map_item::MapItems;
use crate::server::model::path::manhattan_distance;
use crate::server::model::request::Request;
use crate::server::model::response::Response;
use crate::server::model::session::{SessionRecord, SessionsIter};
use crate::server::model::tasks_queue::TasksQueue;
use model::events::client_notification::{AreaNotificationRangeType, Notification};
use model::events::game_event::GameEvent;
use model::events::persistence_event::PersistenceEvent;


use crate::server::game_loop::GAME_TICK_RATE;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::item_service::ItemService;
use crate::util::cell::{MyRefMut, MyUnsafeCell};
use script::skill::ScriptSkillService;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;


use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_service::ScriptService;
use crate::server::service::server_service::ServerService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;

use crate::server::state::server::ServerState;

use crate::util::packet::{debug_packets_from_vec, PacketDirection, PacketsBuffer};
use crate::util::tick::delayed_tick;

pub mod boot;
pub mod request_handler;
pub mod model;
pub mod script;
mod game_loop;
pub mod persistence;
pub mod service;
pub mod state;
pub mod map_instance_loop;

thread_local!(pub static PACKETVER: RefCell<u32> = const { RefCell::new(0) });
// Todo make this configurable
pub const PLAYER_FOV: u16 = 14;
pub const MOB_FOV: u16 = 14;

pub struct Server {
    pub configuration: &'static Config,
    pub repository: Arc<dyn Repository>,
    state: MyUnsafeCell<ServerState>,
    tasks_queue: Arc<TasksQueue<GameEvent>>,
    movement_tasks_queue: Arc<TasksQueue<GameEvent>>,
    server_service: ServerService,
    shutdown: AtomicBool,
    runtime: Arc<Runtime>,
    recording_sessions: MyUnsafeCell<Vec<SessionRecord>>,
}

unsafe impl Sync for Server {}

unsafe impl Send for Server {}

impl Server {
    pub fn state(&self) -> &ServerState {
        self.state.borrow().as_ref()
    }
    pub fn state_mut(&self) -> MyRefMut<ServerState> {
        self.state.borrow_mut()
    }

    pub(crate) fn pop_task(&self) -> Option<Vec<GameEvent>> {
        self.tasks_queue.pop()
    }

    pub(crate) fn pop_movement_task(&self) -> Option<Vec<GameEvent>> {
        self.movement_tasks_queue.pop()
    }


    pub fn packetver(&self) -> u32 {
        self.configuration.server.packetver
    }

    pub fn new(configuration: &'static Config, repository: Arc<dyn Repository>, map_items: MapItems, npc_script_vm: Arc<Vm>, item_script_vm: Arc<Vm>, client_notification_sender: SyncSender<Notification>,
               persistence_event_sender: SyncSender<PersistenceEvent>, runtime: Arc<Runtime>) -> Server {
        let tasks_queue = Arc::new(TasksQueue::new());
        let movement_tasks_queue = Arc::new(TasksQueue::new());
        StatusService::init(GlobalConfigService::instance(), item_script_vm.clone());

        let server_service = ServerService::new(client_notification_sender.clone(), GlobalConfigService::instance(), tasks_queue.clone(), movement_tasks_queue.clone(), npc_script_vm.clone(),
                                                InventoryService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance(), tasks_queue.clone()),
                                                BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal),
                                                SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal), StatusService::instance(), GlobalConfigService::instance()),
                                                StatusService::instance(),
                                                ScriptService::new(client_notification_sender.clone(), GlobalConfigService::instance(), repository.clone(), tasks_queue.clone(), npc_script_vm.clone()),
                                                CharacterService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance(),
                                                                      SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()), StatusService::instance(),
                                                                      tasks_queue.clone()),
                                                SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
                                                ItemService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), item_script_vm, GlobalConfigService::instance()),
                                                ScriptSkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance()),
        );
        Server {
            configuration,
            repository,
            tasks_queue,
            state: MyUnsafeCell::new(ServerState::new(map_items)),
            movement_tasks_queue,
            server_service,
            shutdown: AtomicBool::new(false),
            recording_sessions: MyUnsafeCell::new(vec![]),
            runtime
        }
    }

    pub fn new_without_service_init(configuration: &'static Config, repository: Arc<dyn Repository>, map_items: MapItems, tasks_queue: Arc<TasksQueue<GameEvent>>, server_service: ServerService, runtime: Arc<Runtime>) -> Server {
        Server {
            configuration,
            repository,
            state: MyUnsafeCell::new(ServerState::new(map_items)),
            tasks_queue,
            movement_tasks_queue: Arc::new(Default::default()),
            server_service,
            shutdown: AtomicBool::new(false),
            recording_sessions: MyUnsafeCell::new(vec![]),
            runtime
        }
    }

    pub async fn shutdown(&self) {
        self.shutdown.store(true, Ordering::Relaxed);
        self.state.borrow().map_instances().iter().for_each(|(_, instances)| instances.iter().for_each(|instance| instance.shutdown()));
        self.character_service().save_characters_state(self.state.borrow().characters().values().collect()).await;
        self.state_mut().sessions().write().unwrap().iter().for_each(|(_, session)| session.disconnect());
    }

    pub fn is_alive(&self) -> bool {
        !self.shutdown.load(Ordering::Relaxed)
    }

    pub fn runtime(&self) -> &Runtime {
        self.runtime.as_ref()
    }

    #[inline]
    pub fn skill_service(&self) -> &SkillService {
        self.server_service.skill_service()
    }
    #[inline]
    pub fn server_service(&self) -> &ServerService {
        &self.server_service
    }
    #[inline]
    pub fn item_service(&self) -> &ItemService {
        self.server_service.item_service()
    }

    #[inline]
    pub fn battle_service(&self) -> &BattleService {
        self.server_service.battle_service()
    }

    #[inline]
    pub fn script_service(&self) -> &ScriptService {
        self.server_service.script_service()
    }

    #[inline]
    pub fn inventory_service(&self) -> &InventoryService {
        self.server_service.inventory_service()
    }
    #[inline]
    pub fn character_service(&self) -> &CharacterService {
        self.server_service.character_service()
    }
    #[inline]
    pub fn skill_tree_service(&self) -> &SkillTreeService {
        self.server_service.skill_tree_service()
    }

    // TODO: This service should be removed and replace by skill implementation
    #[inline]
    pub fn script_skill_service(&self) -> &ScriptSkillService {
        self.server_service.script_skill_service()
    }

    pub fn add_to_next_tick(&self, event: GameEvent) {
        self.tasks_queue.add_to_first_index(event)
    }

    pub fn add_to_tick(&self, event: GameEvent, index: usize) {
        self.tasks_queue.add_to_index(event, index)
    }

    pub fn add_to_delayed_tick(&self, event: GameEvent, delay: u128) {
        self.add_to_tick(event, delayed_tick(delay, GAME_TICK_RATE));
    }

    pub fn add_to_next_movement_tick(&self, event: GameEvent) {
        self.movement_tasks_queue.add_to_first_index(event)
    }

    pub fn start_recording_session(&self, char_id: u32) {
        let session_id = self.state().characters().get(&char_id).unwrap().account_id;
        if !self.is_recording_session(char_id) {
            self.recording_sessions.borrow_mut().push(SessionRecord::new(session_id, char_id));
        }
    }

    pub fn stop_recording_session(&self, char_id: u32) {
        let session_id = self.state().characters().get(&char_id).unwrap().account_id;
        if let Some(recording) = self.get_recording_session(session_id) {
            recording.finish();
        }
        self.recording_sessions.borrow_mut().retain(|recording_session_id| recording_session_id.session_id != session_id);
    }

    pub fn is_recording_session(&self, char_id: u32) -> bool {
        let session_id = self.state().characters().get(&char_id).unwrap().account_id;
        self.recording_sessions.borrow().as_ref().iter().any(|recording_session_id| recording_session_id.session_id == session_id)
    }

    pub fn get_recording_session(&self, session_id: u32) -> Option<&SessionRecord> {
        self.recording_sessions.borrow().as_ref().iter().find(|recording_session_id| recording_session_id.session_id == session_id)
    }

    pub fn disconnect_character(&self, char_id: u32) {
        {
            let character = self.state().get_character(char_id);
            if let Some(character) = character {
                self.runtime.block_on(async {
                    self.character_service().save_characters_state(vec![character]).await;
                    self.repository.save_hotkeys(char_id, &character.hotkeys).await.unwrap();
                });
            }
        }
        self.state_mut().characters_mut().remove(&char_id);
    }

    #[allow(unused_lifetimes)]
    pub fn start<'server>(server_ref: Arc<Server>,
                          client_notification_sender: SyncSender<Notification>,
                          single_client_notification_receiver: Receiver<Notification>,
                          persistence_event_receiver: Receiver<PersistenceEvent>,
                          persistence_event_sender: SyncSender<PersistenceEvent>, enable_client_interfaces: bool) {
        let port = server_ref.configuration.server.port;

        let (response_sender, single_response_receiver) = std::sync::mpsc::sync_channel::<Response>(0);
        let client_notification_sender_clone = client_notification_sender.clone();
        thread::scope(|server_thread_scope: &Scope| {
            let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();
            // listener.set_nonblocking(true);
            let server_shared_ref = server_ref.clone();
            if enable_client_interfaces {
                info!("Server listen on 0.0.0.0:{}", port);
                thread::Builder::new().name("client_connection_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                    for tcp_stream in listener.incoming() {
                        if !server_shared_ref.is_alive() {
                            break;
                        }
                        // Receive new connection, starting new thread
                        let server_shared_ref = server_shared_ref.clone();
                        debug!("Received new connection");
                        let response_sender_clone = response_sender.clone();
                        let client_notification_sender_clone = client_notification_sender_clone.clone();
                        let mut tcp_stream = tcp_stream.unwrap();
                        thread::Builder::new().name(format!("client_{}_thread", tcp_stream.peer_addr().unwrap())).spawn_scoped(server_thread_scope, move || {
                            PACKETVER.with(|ver| *ver.borrow_mut() = server_shared_ref.packetver());

                            let tcp_stream_arc = Arc::new(RwLock::new(tcp_stream.try_clone().unwrap())); // todo remove this clone
                            let mut buffer = [0; 2048];
                            loop {
                                if !server_shared_ref.is_alive() {
                                    let _ = tcp_stream.shutdown(Shutdown::Both);
                                    break;
                                }
                                match tcp_stream.read(&mut buffer) {
                                    Ok(bytes_read) => {
                                        if bytes_read == 0 {
                                            tcp_stream.shutdown(Shutdown::Both).expect("Unable to shutdown incoming socket. Shutdown was done because remote socket seems closed.");
                                            break;
                                        }
                                        let packet = parse(&buffer[..bytes_read], server_shared_ref.packetver());
                                        let context = Request::new(server_shared_ref.configuration, None, server_shared_ref.packetver(), tcp_stream_arc.clone(), packet.as_ref(), response_sender_clone.clone(), client_notification_sender_clone.clone());
                                        request_handler::handle(server_shared_ref.clone(), context);
                                    }
                                    Err(err) => {
                                        error!("{}", err);
                                        let _ = tcp_stream.shutdown(Shutdown::Both);
                                        break;
                                    }
                                }
                            }
                        });
                    }
                    info!("Shutdown client_connection_thread");
                }).unwrap();
                // Start a thread sending response packet to client request

                let server_ref_clone = server_ref.clone();
                thread::Builder::new().name("client_response_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                    loop {
                        if let Ok(response) = single_response_receiver.try_recv() {
                            let tcp_stream = &response.socket();
                            let data = response.serialized_packet();
                            let mut tcp_stream_guard = tcp_stream.write().unwrap();
                            debug!("Respond to {:?} with: {:02X?}", tcp_stream_guard.peer_addr(), data);
                            if GlobalConfigService::instance().config().server.trace_packet {
                                debug_packets_from_vec(Some(tcp_stream_guard.peer_addr().as_ref().unwrap()), PacketDirection::Backward,
                                                       GlobalConfigService::instance().packetver(), data, &Option::None);
                            }
                            tcp_stream_guard.write_all(data).unwrap();
                            tcp_stream_guard.flush().unwrap();
                        } else if !server_ref_clone.is_alive() {
                            break;
                        }
                    }
                    info!("Shutdown client_response_thread");
                }).unwrap();
                // Start a thread sending packet to notify client from game update
                let server_ref_clone = server_ref.clone();
                thread::Builder::new().name("client_notification_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                    let server_ref = server_ref_clone;
                    let mut packets_by_session: Vec<PacketsBuffer> = Vec::new();
                    loop {
                        if !server_ref.is_alive() {
                            thread::sleep(Duration::from_millis(500));
                        }
                        packets_by_session.retain(|buffer| {
                            if buffer.should_flush() {
                                if let Some(tcp_stream) = server_ref.state().get_map_socket_for_char_id(buffer.session_id()) {
                                    let mut tcp_stream_guard = tcp_stream.write().unwrap();
                                    if tcp_stream_guard.peer_addr().is_ok() {
                                        debug!("Respond to {:?} with {} bytes with: {:02X?}", tcp_stream_guard.peer_addr(), buffer.data().len(), buffer.data());
                                        if buffer.data().is_empty() {
                                            debug!("{} - {:?}", buffer.session_id(), buffer.data());
                                        }
                                        if GlobalConfigService::instance().config().server.trace_packet {
                                            debug_packets_from_vec(Some(tcp_stream_guard.peer_addr().as_ref().unwrap()), PacketDirection::Backward,
                                                                   GlobalConfigService::instance().packetver(), buffer.data(), &Option::None);
                                            info!("Flushing {} {}bytes - {:02X?}", buffer.session_id(), buffer.data().len(), buffer.data());
                                        }
                                        if tcp_stream_guard.write_all(buffer.data()).is_ok() {
                                            tcp_stream_guard.flush().unwrap();
                                        }
                                    } else {
                                        error!("{:?} socket has been closed", tcp_stream_guard.peer_addr().err());
                                    }
                                }
                                return false;
                            }
                            true
                        });
                        match single_client_notification_receiver.recv_timeout(Duration::from_millis(16)) {
                            Ok(response) => {
                                match response {
                                    Notification::Char(char_notification) => {
                                        Self::buffer_packets(&mut packets_by_session, char_notification.char_id(), char_notification.serialized_packet().as_slice());
                                    }
                                    Notification::Area(area_notification) => {
                                        match area_notification.range_type {
                                            AreaNotificationRangeType::Map => {}
                                            AreaNotificationRangeType::Fov { x, y, exclude_id } => {
                                                server_ref.state().characters().iter()
                                                    .filter(|(_, character)| character.current_map_name() == &area_notification.map_name
                                                        && character.current_map_instance() == area_notification.map_instance_id
                                                        && manhattan_distance(character.x(), character.y(), x, y) <= PLAYER_FOV
                                                        && (exclude_id.is_none() || exclude_id.unwrap() != character.char_id)
                                                    )
                                                    .for_each(|(_, character)| {
                                                        if GlobalConfigService::instance().config().server.trace_packet {
                                                            debug_packets_from_vec(None, PacketDirection::Backward,
                                                                                   GlobalConfigService::instance().packetver(), area_notification.serialized_packet(), &Some(String::from("Enqueu in buffer")));
                                                        }
                                                        Self::buffer_packets(&mut packets_by_session, character.char_id, area_notification.serialized_packet().as_slice());
                                                    });
                                            }
                                        }
                                    }
                                }
                            }
                            Err(mpsc::RecvTimeoutError::Timeout) => {}
                            _ => {}
                        }
                        if !server_ref.is_alive() {
                            break;
                        }
                    }
                    info!("Shutdown client_notification_thread");
                }).unwrap();
            } else {
                info!("Server does not listen client requests");
            }
            let server_ref_clone = server_ref.clone();
            thread::Builder::new().name("game_loop_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                Self::game_loop(server_ref_clone);
                info!("Shutdown game_loop_thread");
            }).unwrap();
            let server_ref_clone = server_ref.clone();
            let client_notification_sender_clone = client_notification_sender.clone();
            let persistence_event_sender_clone = persistence_event_sender.clone();
            thread::Builder::new().name("movement_loop_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                Self::character_movement_loop(server_ref_clone, client_notification_sender_clone, persistence_event_sender_clone);
                info!("Shutdown movement_loop_thread");
            }).unwrap();
            let server_ref_clone = server_ref.clone();
            thread::Builder::new().name("persistence_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                Self::persistence_thread(server_ref_clone.clone(), persistence_event_receiver, server_ref_clone.repository.clone());
                info!("Shutdown persistence_thread");
            }).unwrap();
            let server_ref_clone = server_ref.clone();
            thread::Builder::new().name("shutdown_thread".to_string()).spawn_scoped(server_thread_scope, move || {
                server_ref_clone.runtime.block_on(async {
                    tokio::signal::ctrl_c().await.unwrap();
                    server_ref_clone.shutdown().await;
                    info!("Hello ctrl+c");
                    TcpStream::connect(format!("127.0.0.1:{port}")).map(|mut stream| stream.flush()).ok();
                });
                info!("Shutdown shutdown_thread");
            }).unwrap();
        });
        info!("Shutdown server");
    }

    fn buffer_packets(packets_by_session: &mut Vec<PacketsBuffer>, char_id: u32, data: &[u8]) {
        let maybe_buffer = packets_by_session.iter_mut().find(|buffer| buffer.session_id() == char_id);
        if maybe_buffer.is_none() {
            let mut buffer = PacketsBuffer::new(char_id, 2048);
            buffer.push(data);
            packets_by_session.push(buffer);
        } else {
            let buffer = maybe_buffer.unwrap();
            if buffer.can_contain(data.len()) {
                buffer.push(data);
            } else {
                let mut buffer = PacketsBuffer::new(char_id, 2048);
                buffer.push(data);
                packets_by_session.push(buffer);
            }
        }
    }

    pub fn ensure_session_exists(&self, tcp_stream: &Arc<RwLock<TcpStream>>) -> Option<u32> {
        if let Ok(session_guard) = self.state().sessions().read() {
            let stream_guard = read_lock!(tcp_stream);
            let session_option = session_guard.find_by_stream(&stream_guard);
            if session_option.is_none() {
                debug!("Session does not exist! for socket {:?}", stream_guard);
                return None;
            }
            return Some(session_option.unwrap())
        }
        None
    }
}
