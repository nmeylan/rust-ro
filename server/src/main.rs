#![feature(future_join)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

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

extern crate enums;
extern crate metrics;

extern crate packets;
extern crate sqlx;
extern crate core;

use std::collections::HashMap;

use std::thread::{JoinHandle};
use proxy::map::MapProxy;
use crate::proxy::char::CharProxy;
use std::sync::{Arc, Once};
use crate::repository::Repository;
use std::time::{Instant};
use flexi_logger::Logger;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use tokio::runtime::Runtime;
use crate::server::npc::warps::Warp;
use server::Server;
use crate::server::core::configuration::{Config, JobConfig, SkillConfig};
use crate::server::core::map::Map;
use server::events::client_notification::Notification;
use server::events::persistence_event::PersistenceEvent;
use crate::repository::model::item_model::ItemModel;
use self::server::core::map_item::MapItem;
use self::server::script::ScriptHandler;
use crate::server::npc::mob_spawn::MobSpawn;
use crate::server::npc::script::Script;
use crate::util::cell::MyUnsafeCell;
use crate::util::log_filter::LogFilter;

pub static mut JOB_CONFIGS: Vec<JobConfig> = Vec::new();
pub static JOB_CONFIGS_INIT: Once = Once::new();
pub static mut SKILL_CONFIGS: Option<HashMap<String, SkillConfig>> = None;
pub static mut SKILL_CONFIGS_ID_NAME: Option<HashMap<u32, String>> = None;
pub static SKILL_CONFIGS_INIT: Once = Once::new();
pub static SKILL_CONFIGS_ID_NAME_INIT: Once = Once::new();
pub static mut ITEMS_CACHE: Option<HashMap<u32, ItemModel>> = None;
pub static ITEMS_CACHE_INIT: Once = Once::new();


#[tokio::main]
pub async fn main() {
    let config = Config::load().unwrap();
    let logger= Logger::try_with_str(config.server.log_level.as_ref().unwrap()).unwrap();
    logger.filter(Box::new(LogFilter::new(config.server.log_exclude_pattern.as_ref().unwrap().clone()))).start().unwrap();
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

    let items =  repository_arc.get_all_items().await.unwrap();
    init_items_cache(items);
    skill_configs();
    job_configs();

    if config.server.enable_visual_debugger {
        #[cfg(feature = "visual_debugger")]
        {
            crate::debugger::visual_debugger::VisualDebugger::run(server_ref_clone.clone());
        }
        #[cfg(not(feature = "visual_debugger"))]
        {
            warn!("Visual debugger has been enable in configuration, but feature has not been compiled. Please consider enabling \"visual-debugger\" feature.");
        }
    }
    Server::start(server_ref_clone, single_client_notification_receiver);

    for handle in handles {
        handle.join().expect("Failed await server and proxy threads");
    }
}

fn job_configs() -> &'static Vec<JobConfig> {
    JOB_CONFIGS_INIT.call_once(|| unsafe {
        JOB_CONFIGS = Config::load_jobs_config().unwrap();
    });
    unsafe { &JOB_CONFIGS }
}

fn init_items_cache(items: Vec<ItemModel>) {
    ITEMS_CACHE_INIT.call_once(|| unsafe {
        ITEMS_CACHE = Some(items.into_iter().map(|item| (item.id as u32, item)).collect());
    });
}

fn skill_configs() -> &'static HashMap<String, SkillConfig> {
    SKILL_CONFIGS_INIT.call_once(|| unsafe {
        SKILL_CONFIGS = Some(Config::load_skills_config().unwrap());
    });
    unsafe { SKILL_CONFIGS.as_ref().unwrap() }
}

fn skill_id_name() -> &'static HashMap<u32, String> {
    SKILL_CONFIGS_ID_NAME_INIT.call_once(|| unsafe {
        SKILL_CONFIGS_ID_NAME = Default::default();
        skill_configs().values().for_each(|skill_config| {
            SKILL_CONFIGS_ID_NAME.as_mut().unwrap().insert(skill_config.id, skill_config.name.clone());
        });
    });
    unsafe { SKILL_CONFIGS_ID_NAME.as_ref().unwrap() }
}

pub fn get_job_config(id: u32) -> &'static JobConfig {
    job_configs().iter().find(|config| *config.id() == id).unwrap_or_else(|| panic!("Expected to find job config for id {} but found none", id))
}

pub fn get_skill_config(name: &str) -> &'static SkillConfig {
    skill_configs().get(name).unwrap_or_else(|| panic!("Expected to find skill config for name {} but found none", name))
}

pub fn get_skill_config_by_id(id: u32) -> &'static SkillConfig {
    let name = skill_id_name().get(&id).unwrap_or_else(|| panic!("Expected to find skill config for id {} but found none", id));
    skill_configs().get(name).unwrap_or_else(|| panic!("Expected to find skill config for name {} but found none", name))
}

pub fn get_item(id: i32) -> &'static ItemModel {
    unsafe { ITEMS_CACHE.as_ref().unwrap().get(&(id as u32)).unwrap_or_else(|| panic!("Expected to find item for id {} but found none", id)) }
}