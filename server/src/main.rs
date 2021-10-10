mod proxy;
mod util;
mod packets;
mod server;
mod repository;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};
use std::thread::{JoinHandle};
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

    let mut server = Server::new(Arc::new(repository), Arc::new(RwLock::new(maps)), Arc::new(warps));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    &handles.push(server.start(6901));
    let char_proxy = CharProxy::new();
    let map_proxy = MapProxy::new();
    &handles.push(char_proxy.proxy());
    &handles.push(map_proxy.proxy());

    for handle in handles {
        handle.join();
    }
}