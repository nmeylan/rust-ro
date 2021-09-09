mod server;
mod util;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::thread::{JoinHandle};
use server::login::LoginServer;
use server::map::MapServer;
use crate::server::char::CharServer;
use crate::server::server::ServerContext;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

fn main() {
    let login = server::server::Server {
        name: "login".to_string(),
        local_port: 6901,
        target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6900),
        packet_handler: LoginServer
    };
    let server_context = ServerContext{
        sessions: HashMap::new()
    };
    let server_context_arc = Arc::new(Mutex::new(server_context));
    let char = CharServer::new(server_context_arc.clone());
    let map = MapServer::new(server_context_arc.clone());
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    &handles.push(login.proxy());
    &handles.push(char.proxy());
    &handles.push(map.proxy());

    for handle in handles {
        handle.join();
    }
}