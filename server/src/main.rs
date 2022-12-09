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

extern crate packets;
extern crate sqlx;
extern crate core;


use std::collections::HashMap;

use std::thread::{JoinHandle};
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use std::sync::{Arc};
use crate::repository::Repository;
use std::time::{Instant};
use flexi_logger::Logger;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use tokio::runtime::Runtime;
use crate::server::npc::warps::Warp;
use server::Server;
use crate::server::core::configuration::Config;
use crate::server::core::map::Map;
use server::events::client_notification::Notification;
use server::events::persistence_event::PersistenceEvent;
use self::server::core::map_item::MapItem;
use self::server::script::ScriptHandler;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::script::Script;
use crate::util::cell::MyUnsafeCell;
use crate::util::log_filter::LogFilter;

#[tokio::main]
pub async fn main() {
    let config = Config::load().unwrap();
    let logger= Logger::try_with_str(config.server.log_level.as_ref().unwrap()).unwrap();
    logger.filter(Box::new(LogFilter::new())).start().unwrap();
    let repository : Repository = Repository::new_pg(&config.database, Runtime::new().unwrap()).await;
    let repository_arc = Arc::new(repository);
    let warps = Warp::load_warps().await;
    let mob_spawns = MobSpawn::load_mob_spawns(repository_arc.clone()).join().unwrap();
    // let mob_spawns = Default::default();
    let (scripts, class_files, compilation_errors) = Script::load_scripts();
    for class_errors in compilation_errors {
        error!("Error while compiling {}", class_errors.0);
        for compilation_error in class_errors.1 {
            error!("{}", compilation_error);
        }
    }
    // let scripts = Default::default();
    // let class_files = Default::default();
    let map_item_ids = MyUnsafeCell::new(HashMap::<u32, MapItem>::new());
    let start = Instant::now();
    let maps = Map::load_maps(warps, mob_spawns, scripts, map_item_ids.clone());
    let maps = maps.into_iter().map(|(k, v)| (k, Arc::new(v))).collect::<HashMap<String, Arc<Map>>>();
    info!("load {} map-cache in {} secs", maps.len(), start.elapsed().as_millis() as f32 / 1000.0);
    let vm = Arc::new(Vm::new("native_functions_list.txt", DebugFlag::None.value()));
    Vm::bootstrap(vm.clone(), class_files, Box::new(&ScriptHandler{}));

    let (client_notification_sender, single_client_notification_receiver) = std::sync::mpsc::sync_channel::<Notification>(0);
    let server = Server::new(config.clone(), repository_arc.clone(), maps, map_item_ids, vm, client_notification_sender);
    let server_ref = Arc::new(server);
    let server_ref_clone = server_ref;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let char_proxy = CharProxy::new(&config.proxy);
    let map_proxy = MapProxy::new(&config.proxy);
    let _ = &handles.push(char_proxy.proxy(config.server.packetver));
    let _ = &handles.push(map_proxy.proxy(config.server.packetver));

    Server::start(server_ref_clone, single_client_notification_receiver);
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