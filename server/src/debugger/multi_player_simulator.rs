use crate::server::model::session::{Session, SessionRecord};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::Server;
use packets::packets::{Packet, PacketChSelectChar, PacketCzEnter2};
use rand::{thread_rng, RngCore};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct MultiPlayerSimulator {
    sessions: Vec<Arc<Session>>,
    server: Arc<Server>,
    pub sender: SyncSender<SimlatorEvent>,
    pub session_replaying: Vec<u32>
}

pub struct SimulatedSession {
    session: Arc<Session>,
    char_name: String,
    char_class: String,
}
enum SimlatorEvent {
    AddSession(Arc<Session>),
    SessionReplay(Arc<Session>, SessionRecord),
    SessionReplayStop(Arc<Session>),
}

struct ReplaySession {
    session_record: SessionRecord,
    current_entry: usize,
    next_entry_at: u128,
}

impl ReplaySession {
    pub fn new(session_record: SessionRecord) -> Self {
        Self {
            session_record,
            current_entry: 0,
            next_entry_at: 0,
        }
    }
}

const SIMULATOR_TICK_RATE: u128 = 40;
impl MultiPlayerSimulator {
    pub fn new(server: Arc<Server>) -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel::<SimlatorEvent>(0);
        Self::simulation_loop(receiver);
        Self { sessions: vec![], server, sender, session_replaying: vec![] }
    }
    pub fn simulate(&mut self, char_id: u32) {
        let client_socket = TcpStream::connect(format!("127.0.0.1:{}", GlobalConfigService::instance().config().server.port)).unwrap();
        info!("Start simulation for char {}, created socket: {}", char_id, client_socket.local_addr().unwrap());
        let account_id = thread_rng().next_u32();
        let session = Session::create_with_map_socket_and_char_id(account_id, char_id, GlobalConfigService::instance().packetver(),
                                                                  Arc::new(RwLock::new(client_socket)));
        let session = Arc::new(session);
        self.sessions.push(session.clone());
        self.server.state().add_session(account_id, session.clone());

        self.sender.send(SimlatorEvent::AddSession(session.clone()));
    }

    pub fn replay_packet(&mut self, session_record: SessionRecord, sessions_index: Vec<usize>) {
        for session_index in sessions_index {
            info!("Will replay session {} for session index {}", &session_record.entries_formatted, session_index);
            let session = self.sessions[session_index].clone();
            self.session_replaying.push(session.account_id);
            self.sender.send(SimlatorEvent::SessionReplay(session, session_record.clone()));
        }
    }

    pub fn stop_session_replay(&mut self, session_id: u32) {
        self.sessions.iter().filter(|session| session.account_id == session_id).for_each(|session| {
            self.sender.send(SimlatorEvent::SessionReplayStop(session.clone()));
        });
        self.session_replaying.retain(|replaying_session_id| *replaying_session_id != session_id);
    }

    pub fn sessions(&self) -> &Vec<Arc<Session>> {
        &self.sessions
    }

    pub fn character_simulated(&self, char_id: u32) -> bool {
        self.sessions.iter().any(|s| s.char_id.unwrap().eq(&char_id))
    }

    pub fn simulation_loop(receiver: Receiver<SimlatorEvent>) {
        thread::Builder::new().name("simulator-thread".to_string()).spawn(move || {
            let mut sessions_with_replay: HashMap<Arc<Session>, ReplaySession> = HashMap::new();
            loop {
                let tick = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

                if let Ok(event) = receiver.try_recv() {
                    match event {
                        SimlatorEvent::AddSession(session) => {
                            let mut packet = PacketChSelectChar::new(GlobalConfigService::instance().packetver());
                            packet.fill_raw();
                            session.map_server_socket.as_ref().unwrap().write().unwrap().write_all(packet.raw().as_slice());
                            sleep(Duration::from_millis(500));
                            let mut packet = PacketCzEnter2::new(GlobalConfigService::instance().packetver());
                            packet.aid = session.account_id;
                            packet.auth_code = session.auth_code;
                            packet.fill_raw();
                            session.map_server_socket.as_ref().unwrap().write().unwrap().write_all(packet.raw().as_slice());
                        }
                        SimlatorEvent::SessionReplay(session, session_record) => {
                            sessions_with_replay.insert(session, ReplaySession::new(session_record));
                        }
                        SimlatorEvent::SessionReplayStop(session) => {
                            sessions_with_replay.remove(&session);
                        }
                    }
                }

                for (session, replay) in sessions_with_replay.iter_mut() {
                    if replay.next_entry_at <= tick {
                        let mut entries_guard = replay.session_record.entries.lock().unwrap();
                        if let Some(entry) = entries_guard.get_mut(replay.current_entry) {
                            let mut packet = entry.packet();
                            debug!("Simulator - Sending packet {} - {}", packet.name(), replay.current_entry);
                            session.map_server_socket.as_ref().unwrap().write().unwrap().write_all(packet.raw().as_slice());
                        }
                        if let Some(entry) = entries_guard.get(replay.current_entry) {
                            let mut next_entry = replay.current_entry + 1;
                            if next_entry >= entries_guard.len() {
                                next_entry = 0;
                                replay.next_entry_at = tick + 200; // TODO make this configurable
                            } else {
                                let diff = entries_guard.get(next_entry).unwrap().time - entry.time;
                                replay.next_entry_at = tick + diff;
                            }
                            replay.current_entry = next_entry;
                        }
                    }
                }

                let time_spent = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() - tick;
                let sleep_duration = (SIMULATOR_TICK_RATE as i128 - time_spent as i128).max(0) as u64;
                if sleep_duration < 5 {
                    warn!("Less than 5 milliseconds of sleep, game loop is too slow - {}ms because game loop took {}ms", sleep_duration, time_spent);
                }
                sleep(Duration::from_millis(sleep_duration));
            }
        }).unwrap();
    }
}