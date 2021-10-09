mod proxy;
mod util;
mod packets;
mod server;
mod repository;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::thread::{JoinHandle};
use proxy::login::LoginProxy;
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use crate::server::core::{Server};
use crate::repository::lib::Repository;
use sqlx::MySql;
use std::time::{SystemTime, Instant};
use crate::server::map::Map;
use crate::server::scripts::warps::Warp;

#[tokio::main]
pub async fn main() {
    let repository : Repository<MySql> = Repository::<MySql>::new_mysql().await;

    let start = Instant::now();
    let warps = Warp::load_warps().await;
    println!("load {} warps in {} secs", warps.iter().fold(0, |memo, curr| memo + curr.1.len()), start.elapsed().as_millis() as f32 / 1000.0);
    let maps = Map::load_maps();
    println!("load {} map-cache in {} secs", maps.len(), start.elapsed().as_millis() as f32 / 1000.0);

    let server = Server::new(Arc::new(repository), Arc::new(RwLock::new(maps)), Arc::new(warps));
    let server_arc = Arc::new(server);
    let login_proxy = proxy::proxy::Proxy {
        name: "login".to_string(),
        local_port: 6901,
        target: SocketAddr::new(IpAddr::from(Ipv4Addr::new(127, 0, 0, 1)), 6900),
        server: server_arc.clone(),
        specific_proxy: LoginProxy
    };
    let char_proxy = CharProxy::new(server_arc.clone());
    let map_proxy = MapProxy::new(server_arc.clone());
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    &handles.push(login_proxy.proxy());
    &handles.push(char_proxy.proxy());
    &handles.push(map_proxy.proxy());

    for handle in handles {
        handle.join();
    }
}