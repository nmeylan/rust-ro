// This is the core of this server. As all feature won't be implemented in one shot, the idea is to proxy unimplemented feature to hercules server.
// For the moment I won't implement TCPListener in this file, but in the "future" proxies will be removed and only this file will have a TCPListener.

use crate::packets::packets::{Packet, PacketUnknown, PacketZcNotifyTime, PacketZcNotifyChat, PacketCaLogin};
use std::sync::{Arc, Mutex};
use crate::proxy::proxy::ServerContext;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::io::Write;
use crate::repository::lib::Repository;
use sqlx::{Database, MySql};
use crate::server::login::authenticate;

pub struct Server {
    pub server_context: Arc<Mutex<ServerContext>>,
    repository: Arc<Repository<MySql>>,
}

pub enum FeatureState {
    Implemented(Box<dyn Packet>),
    Unimplemented,
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
        if packet.as_any().downcast_ref::<PacketUnknown>().is_some() {
            println!("Unknown packet {} of length {}: {:02X?}", packet.id(), packet.raw().len(), packet.raw());
        } else if packet.as_any().downcast_ref::<PacketCaLogin>().is_some() {
            let res = authenticate(packet.as_any().downcast_ref::<PacketCaLogin>().unwrap(), &self.repository);
            println!("Response");
            res.pretty_debug();
            // return FeatureState::Implemented(res);
        } else if packet.as_any().downcast_ref::<PacketZcNotifyTime>().is_none() {
            packet.display()
        }
        FeatureState::Unimplemented
    }
}