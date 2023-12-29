#![feature(future_join)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate accessor;

extern crate enums;
extern crate packets;
extern crate core;

mod proxy;
#[macro_use]
mod util;
pub mod server;
mod repository;
#[cfg(feature = "visual_debugger")]
mod debugger;
mod tests;

use std::collections::HashMap;

#[cfg(feature = "static_db_update")]
use std::fs::File;
#[cfg(feature = "static_db_update")]
use std::io::Write;
#[cfg(feature = "static_db_update")]
use std::path::Path;


use std::thread::{JoinHandle};
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use std::sync::{Arc};
use crate::repository::{ItemRepository, Repository};
use std::time::{Instant};
use flexi_logger::Logger;


use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use tokio::runtime::Runtime;
use server::Server;




use configuration::configuration::{Config};
use crate::repository::model::item_model::ItemModels;
use crate::repository::model::mob_model::MobModels;
use crate::server::model::map::Map;
use self::server::model::events::client_notification::Notification;
use self::server::model::events::persistence_event::PersistenceEvent;


use crate::server::boot::map_loader::MapLoader;
use crate::server::boot::mob_spawn_loader::MobSpawnLoader;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::boot::warps_loader::WarpLoader;
use crate::server::model::script::Script;


use self::server::model::map_item::MapItem;
use self::server::script::ScriptHandler;
use crate::server::service::global_config_service::GlobalConfigService;

use crate::util::log_filter::LogFilter;

pub static mut CONFIGS: Option<Config> = None;
pub static mut MAPS: Option<HashMap<String, &Map>> = None;
pub static mut MOB_ROOT_PATH: &str = "./config/npc";
pub static mut MAP_DIR: &str = "./config/maps/pre-re";
#[tokio::main]
pub async fn main() {
    unsafe {
        CONFIGS = Some(Config::load().unwrap());
    }

    let logger= Logger::try_with_str(configs().server.log_level.as_ref().unwrap()).unwrap();
    logger.filter(Box::new(LogFilter::new(configs().server.log_exclude_pattern.as_ref().unwrap().clone()))).start().unwrap();
    let repository : Repository = Repository::new_pg(&configs().database, Runtime::new().unwrap()).await;
    let repository_arc = Arc::new(repository);
    let items =  repository_arc.get_all_items().await.unwrap();
    let mobs =  repository_arc.get_all_mobs().await.unwrap();
    let mut map_item_ids = HashMap::<u32, MapItem>::new();
    #[cfg(feature = "static_db_update")]
    {
        // items.json is used in tests
        let item_db: ItemModels = items.clone().into();
        let json = serde_json::to_string_pretty(&item_db).unwrap();
        let output_path = Path::new("config");
        let mut file = File::create(output_path.join("items.json")).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        // mobs.json is used in tests
        let mob_db: MobModels = mobs.clone().into();
        let json = serde_json::to_string_pretty(&mob_db).unwrap();
        let output_path = Path::new("config");
        let mut file = File::create(output_path.join("mobs.json")).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
    let skills_config = Config::load_skills_config(".").unwrap();
    let job_configs = Config::load_jobs_config(".").unwrap();
    let job_skills_tree = Config::load_jobs_skill_tree(".").unwrap();

    let vm = Arc::new(Vm::new("native_functions_list.txt", DebugFlag::None.value()));
    let scripts = load_scripts(vm.clone());

    let start = Instant::now();
    let warps = unsafe { WarpLoader::load_warps(CONFIGS.as_ref().unwrap()).await };
    let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
    let mob_spawns = unsafe { MobSpawnLoader::load_mob_spawns(CONFIGS.as_ref().unwrap(), mobs_map, MOB_ROOT_PATH).join().unwrap() };
    let maps = MapLoader::load_maps(warps, mob_spawns, scripts, &mut map_item_ids, unsafe { MAP_DIR });
    info!("load {} map-cache in {} secs", maps.len(), start.elapsed().as_millis() as f32 / 1000.0);
    unsafe {
        GlobalConfigService::init(CONFIGS.clone().unwrap(),
                                  items,
                                  mobs,
                                  job_configs,
                                  job_skills_tree,
                                  skills_config,
                                  maps
        );
    }
    // let mob_spawns = Default::default();

    // let scripts = Default::default();
    // let class_files = Default::default();
    let (client_notification_sender, single_client_notification_receiver) = std::sync::mpsc::sync_channel::<Notification>(2048);
    let (persistence_event_sender, persistence_event_receiver) = std::sync::mpsc::sync_channel::<PersistenceEvent>(2048);
    let server = Server::new(configs(), repository_arc.clone(), map_item_ids, vm, client_notification_sender, persistence_event_sender.clone());
    let server_ref = Arc::new(server);
    let server_ref_clone = server_ref;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let char_proxy = CharProxy::new(&configs().proxy);
    let map_proxy = MapProxy::new(&configs().proxy);
    let _ = &handles.push(char_proxy.proxy(configs().server.packetver));
    let _ = &handles.push(map_proxy.proxy(configs().server.packetver));

    if configs().server.enable_visual_debugger {
        #[cfg(feature = "visual_debugger")]
        {
            crate::debugger::visual_debugger::VisualDebugger::run(server_ref_clone.clone());
        }
        #[cfg(not(feature = "visual_debugger"))]
        {
            warn!("Visual debugger has been enable in configuration, but feature has not been compiled. Please consider enabling \"visual-debugger\" feature.");
        }
    }
    Server::start(server_ref_clone, single_client_notification_receiver, persistence_event_receiver, persistence_event_sender, true);

    for handle in handles {
        handle.join().expect("Failed await server and proxy threads");
    }
}

pub fn load_scripts(vm: Arc<Vm>) -> HashMap<String, Vec<Script>> {
    let start = Instant::now();
    let (scripts, class_files, compilation_errors) = ScriptLoader::load_scripts();
    for class_errors in compilation_errors {
        error!("Error while compiling {}", class_errors.0);
        for compilation_error in class_errors.1 {
            error!("{}", compilation_error);
        }
    }
    info!("load {} scripts in {} secs", scripts.len(), start.elapsed().as_millis() as f32 / 1000.0);

    Vm::bootstrap(vm, class_files, Box::new(&ScriptHandler {}));
    scripts
}

pub fn configs() -> &'static Config {
    unsafe { CONFIGS.as_ref().unwrap() }
}