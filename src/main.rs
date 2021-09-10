mod proxy;
mod util;
mod packets;
mod server;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::thread::{JoinHandle};
use proxy::login::LoginProxy;
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use crate::proxy::proxy::ServerContext;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::core::Server;

fn main() {
    let server_context = ServerContext{
        sessions: HashMap::new()
    };
    let server_context_arc = Arc::new(Mutex::new(server_context));
    let server = Server::new(server_context_arc.clone());
    let server_arc = Arc::new(server);
    let login_proxy = proxy::proxy::Proxy {
        name: "login".to_string(),
        local_port: 6901,
        target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6900),
        server: server_arc.clone(),
        specific_proxy: LoginProxy
    };
    let char_proxy = CharProxy::new(server_context_arc.clone(), server_arc.clone());
    let map_proxy = MapProxy::new(server_context_arc.clone(), server_arc.clone());
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    &handles.push(login_proxy.proxy());
    &handles.push(char_proxy.proxy());
    &handles.push(map_proxy.proxy());

    for handle in handles {
        handle.join();
    }
}