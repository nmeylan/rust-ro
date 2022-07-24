mod proxy;
#[macro_use]
mod util;
mod server;
mod repository;
#[cfg(feature = "visual_debugger")]
mod debugger;

#[macro_use]
extern crate log;
#[macro_use]
extern crate accessor;

#[macro_use]
extern crate metrics;
extern crate packets;
extern crate sqlx;

use std::collections::HashMap;

use std::thread::{JoinHandle};
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use std::sync::{Arc, RwLock};
use crate::repository::lib::Repository;
use sqlx::MySql;
use std::time::{Instant};
use flexi_logger::Logger;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::server::core::map::{Map, MapItem};
use crate::server::npc::warps::Warp;
use crate::server::server::Server;
use crate::server::configuration::Config;
use self::server::script::script::ScriptHandler;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::script::Script;
use crate::util::log_filter::LogFilter;

#[tokio::main]
pub async fn main() {
    let config = Config::load().unwrap();
    let logger= Logger::try_with_str(config.server.log_level.as_ref().unwrap()).unwrap();
    logger.filter(Box::new(LogFilter::new())).start().unwrap();
    let repository : Repository<MySql> = Repository::<MySql>::new_mysql(&config.database).await;
    let repository_arc = Arc::new(repository);
    let warps = Warp::load_warps().await;
    let mob_spawns = MobSpawn::load_mob_spawns(repository_arc.clone()).join().unwrap();
    let (scripts, class_files, compilation_errors) = Script::load_scripts();
    for class_errors in compilation_errors {
        error!("Error while compiling {}", class_errors.0);
        for compilation_error in class_errors.1 {
            error!("{}", compilation_error);
        }
    }
    let map_item_ids = RwLock::new(HashMap::<u32, Arc<dyn MapItem>>::new());
    let start = Instant::now();
    let maps = Map::load_maps(warps, mob_spawns, scripts, &map_item_ids);
    let maps = maps.into_iter().map(|(k, v)| (k.to_string(), Arc::new(v))).collect::<HashMap<String, Arc<Map>>>();
    info!("load {} map-cache in {} secs", maps.len(), start.elapsed().as_millis() as f32 / 1000.0);
    let vm = Arc::new(Vm::new("native_functions_list.txt", DebugFlag::None.value()));
    Vm::bootstrap(vm.clone(), class_files, Box::new(&ScriptHandler{}));
    let server = Server::new(config.clone(), repository_arc.clone(), maps, Arc::new(map_item_ids), vm);
    let server_ref = Arc::new(server);
    let server_ref_clone = server_ref.clone();
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let _ = &handles.push(Server::start(server_ref_clone));
    let char_proxy = CharProxy::new(&config.proxy);
    let map_proxy = MapProxy::new(&config.proxy);
    let _ = &handles.push(char_proxy.proxy(config.server.packetver));
    let _ = &handles.push(map_proxy.proxy(config.server.packetver));

    if config.server.enable_visual_debugger {
        #[cfg(feature = "visual_debugger")]
        {
            crate::debugger::visual_debugger::VisualDebugger::run(server_ref.clone());
        }
        #[cfg(not(feature = "visual_debugger"))]
        {
            warn!("Visual debugger has been enable in configuration, but feature has not been compiled. Please consider enabling \"visual-debugger\" feature.");
        }
    }

    for handle in handles {
        handle.join().expect("Failed await server and proxy threads");
    }
}