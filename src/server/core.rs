// This is the core of this server. As all feature won't be implemented in one shot, the idea is to proxy unimplemented feature to hercules server.
// For the moment I won't implement TCPListener in this file, but in the "future" proxies will be removed and only this file will have a TCPListener.

use crate::packets::packets::{Packet, PacketUnknown, PacketZcNotifyTime, PacketZcNotifyChat, PacketCaLogin, PacketAcAcceptLogin2, PacketAcRefuseLoginR2, PacketAcRefuseLoginR3};
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

pub struct Server {
    pub server_context: Arc<Mutex<ServerContext>>,
    repository: Arc<Repository<MySql>>,
}

pub enum FeatureState {
    Implemented(Box<dyn Packet>),
    Unimplemented,
}

pub struct ServerContext {
    pub sessions: HashMap<u32, Session>,
}

pub struct Session {
    pub char_server_socket: Option<Arc<Mutex<TcpStream>>>,
    pub map_server_socket: Option<Arc<Mutex<TcpStream>>>,
    pub account_id: u32,
    pub login_id1: i32,
    pub login_id2: u32,
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

    pub fn dispatch(&self, packet: &mut dyn Packet) -> FeatureState {
        let rt = tokio::runtime::Runtime::new().unwrap();
        packet.pretty_debug();
        if packet.as_any().downcast_ref::<PacketUnknown>().is_some() {
            println!("Unknown packet {} of length {}: {:02X?}", packet.id(), packet.raw().len(), packet.raw());
        }
        else if packet.as_any().downcast_ref::<PacketCaLogin>().is_some() {
            let res = rt.block_on( async {
                authenticate(packet.as_any().downcast_ref::<PacketCaLogin>().unwrap(), &self.repository).await
            });
            if res.as_any().downcast_ref::<PacketAcAcceptLogin2>().is_some() {
                let packet_response = res.as_any().downcast_ref::<PacketAcAcceptLogin2>().unwrap();
                let new_user_session = Session {
                    char_server_socket: None,
                    map_server_socket: None,
                    account_id: packet_response.aid,
                    login_id1: packet_response.auth_code,
                    login_id2: packet_response.user_level
                };
                let mut server_context_guard = self.server_context.lock().unwrap();
                server_context_guard.sessions.insert(packet_response.aid.clone(), new_user_session);
                // Currently only respond for this account to be able to still use proxy in other accounts
                if packet_response.aid == 2000000 {
                    return FeatureState::Implemented(res);
                }
            } else if res.as_any().downcast_ref::<PacketAcRefuseLoginR3>().is_some() {
                return FeatureState::Implemented(res);
            }
        }
        else if packet.as_any().downcast_ref::<PacketZcNotifyTime>().is_none() {
            packet.display()
        }
        FeatureState::Unimplemented
    }
}