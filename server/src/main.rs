mod proxy;
#[macro_use]
mod util;
mod server;
mod repository;

#[macro_use]
extern crate log;
#[macro_use]
extern crate accessor;

use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::thread::{JoinHandle};
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use std::sync::{Arc, RwLock};
use crate::repository::lib::Repository;
use sqlx::MySql;
use std::time::{Instant};
use flexi_logger::filter::LogLineFilter;
use flexi_logger::Logger;
use crate::server::core::map::{Map, MapItem};
use crate::server::npc::warps::Warp;
use crate::server::server::Server;
use crate::server::configuration::Config;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::util::log_filter::LogFilter;


#[tokio::main]
pub async fn main() {
    let config = Config::load().unwrap();
    let logger= Logger::try_with_str(config.server.log_level.as_ref().unwrap()).unwrap();
    logger.filter(Box::new(LogFilter::new())).start().unwrap();
    let repository : Repository<MySql> = Repository::<MySql>::new_mysql(&config.database).await;
    let warps = Warp::load_warps().await;
    let mob_spawns = MobSpawn::load_mob_spawns().await;
    let map_item_ids = RwLock::new(HashMap::<u32, Arc<dyn MapItem>>::new());
    let start = Instant::now();
    let maps = Map::load_maps(warps, mob_spawns, &map_item_ids);
    let maps = maps.into_iter().map(|(k, v)| (k.to_string(), Arc::new(v))).collect::<HashMap<String, Arc<Map>>>();
    info!("load {} map-cache in {} secs", maps.len(), start.elapsed().as_millis() as f32 / 1000.0);

    let server = Server::new(config.clone(), Arc::new(repository), maps, Arc::new(map_item_ids));
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let _ = &handles.push(server.start());
    let char_proxy = CharProxy::new(&config.proxy);
    let map_proxy = MapProxy::new(&config.proxy);
    let _ = &handles.push(char_proxy.proxy());
    let _ = &handles.push(map_proxy.proxy());

    for handle in handles {
        handle.join();
    }
}