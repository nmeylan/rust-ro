#![feature(future_join)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate accessor;

extern crate enums;
extern crate packets;

mod proxy;
#[macro_use]
mod util;
pub mod server;
mod repository;
#[cfg(feature = "visual_debugger")]
mod debugger;
mod tests;

use crate::enums::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
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
use crate::server::model::configuration::{Config};
use crate::server::model::map::Map;
use self::server::model::events::client_notification::Notification;
use self::server::model::events::persistence_event::PersistenceEvent;
use crate::repository::model::item_model::ItemModels;
use crate::repository::model::mob_model::MobModels;
use crate::server::boot::map_loader::MapLoader;
use crate::server::boot::mob_spawn_loader::MobSpawnLoader;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::boot::warps_loader::WarpLoader;


use self::server::model::map_item::MapItem;
use self::server::script::ScriptHandler;
use crate::server::service::global_config_service::GlobalConfigService;

use crate::util::log_filter::LogFilter;

pub static mut CONFIGS: Option<Config> = None;
pub static mut MAPS: Option<HashMap<String, &Map>> = None;
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
    let mut skill_configs_id_name: HashMap<String, u32> = Default::default();
    skills_config.values().for_each(|skill_config| {
        skill_configs_id_name.insert(skill_config.name.clone(), skill_config.id);
    });
    let job_configs = Config::load_jobs_config(".").unwrap();

    let mut items_id_name: HashMap<String, u32> = Default::default();
    items.iter().for_each(|item| {
        items_id_name.insert(item.name_aegis.clone(), item.id as u32);
    });
    let mut mobs_id_name: HashMap<String, u32> = Default::default();
    mobs.iter().for_each(|mob| {
        mobs_id_name.insert(mob.name.clone(), mob.id as u32);
    });

    let start = Instant::now();
    let (scripts, class_files, compilation_errors) = ScriptLoader::load_scripts();
    for class_errors in compilation_errors {
        error!("Error while compiling {}", class_errors.0);
        for compilation_error in class_errors.1 {
            error!("{}", compilation_error);
        }
    }
    info!("load {} scripts in {} secs", scripts.len(), start.elapsed().as_millis() as f32 / 1000.0);
    let start = Instant::now();
    let warps = unsafe { WarpLoader::load_warps(CONFIGS.as_ref().unwrap()).await };
    let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
    let mob_spawns = unsafe { MobSpawnLoader::load_mob_spawns(CONFIGS.as_ref().unwrap(), mobs_map).join().unwrap() };
    let maps = MapLoader::load_maps(warps, mob_spawns, scripts, &mut map_item_ids);
    info!("load {} map-cache in {} secs", maps.len(), start.elapsed().as_millis() as f32 / 1000.0);
    unsafe {
        GlobalConfigService::init(CONFIGS.as_ref().unwrap(),
                                  items.into_iter().map(|item| (item.id as u32, item)).collect(),
                                  items_id_name,
                                  mobs.into_iter().map(|mob| (mob.id as u32, mob)).collect(),
                                  mobs_id_name,
                                  job_configs,
                                  skills_config,
                                  skill_configs_id_name,
                                  maps
        );
    }

    // let mob_spawns = Default::default();

    // let scripts = Default::default();
    // let class_files = Default::default();
    let vm = Arc::new(Vm::new("native_functions_list.txt", DebugFlag::None.value()));
    Vm::bootstrap(vm.clone(), class_files, Box::new(&ScriptHandler{}));
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
    Server::start(server_ref_clone, single_client_notification_receiver, persistence_event_receiver, persistence_event_sender);

    for handle in handles {
        handle.join().expect("Failed await server and proxy threads");
    }
}

fn configs() -> &'static Config {
    unsafe { CONFIGS.as_ref().unwrap() }
}