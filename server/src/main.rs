#![feature(test)]
#[macro_use]
extern crate accessor;
extern crate core;
extern crate models;
extern crate packets;
extern crate test;
#[macro_use]
extern crate tracing;

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


use crate::proxy::char::CharProxy;
use crate::repository::{ItemRepository, MobRepository, PgRepository, Repository};
use base64::engine::general_purpose;
use base64::Engine;
use proxy::map::MapProxy;
use rathena_script_lang_interpreter::lang::compiler::Compiler;
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::{self, time::ChronoLocal};


use self::server::model::events::client_notification::Notification;
use self::server::model::events::persistence_event::PersistenceEvent;
use crate::repository::model::item_model::{ItemModel, ItemModels};
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::server::boot::map_loader::MapLoader;
use crate::server::boot::mob_spawn_loader::MobSpawnLoader;
use crate::server::boot::script_loader::ScriptLoader;
use crate::server::boot::warps_loader::WarpLoader;
use crate::server::model::map::Map;
use crate::server::model::map_item::MapItems;
use crate::server::model::script::Script;
use configuration::configuration::Config;
use rathena_script_lang_interpreter::lang::vm::Vm;
use server::Server;
use tokio::runtime::Runtime;
use tracing_subscriber::EnvFilter;

use self::server::script::MapScriptHandler;
use crate::server::script::PlayerScriptHandler;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;

pub static mut CONFIGS: Option<Config> = None;
pub static mut MAPS: Option<HashMap<String, &Map>> = None;
pub static mut MOB_ROOT_PATH: &str = "./config/npc";
pub static mut MAP_DIR: &str = "./config/maps/pre-re";
#[tokio::main]
pub async fn main() {
    let _start = Instant::now();
    unsafe {
        CONFIGS = Some(Config::load("").unwrap());
    }

    setup_logger(configs());
    let runtime = Arc::new(Runtime::new().unwrap());
    let repository : PgRepository = PgRepository::new_pg(&configs().database, runtime.clone()).await;
    let repository_arc = Arc::new(repository);
    // Load all items in memory, it takes only few mb
    let mut items =  repository_arc.get_all_items().await.unwrap();
    // Compiling item scripts into script VM bytecode, if needed (previous compilation are stored in db, recompilation occurs when script has changed)
    compile_item_scripts(&repository_arc, &mut items).await;
    // Load all mobs in memory, it takes only few mb
    let mobs =  repository_arc.get_all_mobs().await.unwrap();
    // Initializing global id pool
    let mut map_item_ids = MapItems::new(300000);
    // Unit Tests needs items and mob db,to avoid starting an actual db, we dump items and mob db into json files
    update_item_and_mob_static_db(&mut items, &mobs);


    // Setup script virtual machine for NPC
    let npc_script_vm = create_script_vm("native_functions_list.txt");
    let scripts = load_scripts(npc_script_vm.clone());

    // Loading configs
    let skills_config = Config::load_skills_config(".").unwrap();
    let job_configs = Config::load_jobs_config(".").unwrap();
    let job_skills_tree = Config::load_jobs_skill_tree(".").unwrap();
    // Loading map-cache and warps
    let start = Instant::now();
    let warps = unsafe { WarpLoader::load_warps(CONFIGS.as_ref().unwrap()).await };
    let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
    let mob_spawns = unsafe { MobSpawnLoader::load_mob_spawns(CONFIGS.as_ref().unwrap(), mobs_map, MOB_ROOT_PATH, runtime.clone()).join().unwrap() };
    let maps = MapLoader::load_maps(warps, mob_spawns, scripts, &mut map_item_ids, unsafe { MAP_DIR });
    info!("Loaded {} map-cache in {}ms", maps.len(), start.elapsed().as_millis());
    // Executing items' script and cache result when possible (e.g: script like `bonus bStr, 3;` result will be cached and item will have a bonus +3 str associated)
    let start = Instant::now();
    let item_script_executed = ItemService::convert_script_into_bonuses(&mut items, "native_functions_list.txt");
    info!("Executed and cached {} item scripts, skipped {} item scripts (requiring runtime data) in {}ms", item_script_executed.0, item_script_executed.1, start.elapsed().as_millis());

    // Creating global config instance, used by all services
    unsafe {
        GlobalConfigService::init(CONFIGS.clone().unwrap(), items, mobs, job_configs, job_skills_tree, skills_config, maps);
    }
    // Init channel for inter-thread communication
    let (client_notification_sender, single_client_notification_receiver) = std::sync::mpsc::sync_channel::<Notification>(2048);
    let (persistence_event_sender, persistence_event_receiver) = std::sync::mpsc::sync_channel::<PersistenceEvent>(2048);
    // Create server
    let item_script_vm = create_script_vm("native_functions_list.txt");
    let server = Server::new(configs(), repository_arc.clone(), map_item_ids, npc_script_vm, item_script_vm, client_notification_sender.clone(), persistence_event_sender.clone(), runtime);
    let server_ref = Arc::new(server);
    PlayerScriptHandler::init(GlobalConfigService::instance(), server_ref.clone());
    let server_ref_clone = server_ref;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    // Create proxies for other emulator (rathena/hercules). TODO: add a configuration to disable this.
    let char_proxy = CharProxy::new(&configs().proxy);
    let map_proxy = MapProxy::new(&configs().proxy);
    let _ = &handles.push(char_proxy.proxy(configs().server.packetver));
    let _ = &handles.push(map_proxy.proxy(configs().server.packetver));

    if configs().server.enable_visual_debugger {
        #[cfg(feature = "visual_debugger")]
        {
            crate::debugger::visual_debugger::VisualDebugger::run(server_ref_clone.clone()).await;
        }
        #[cfg(not(feature = "visual_debugger"))]
        {
            warn!("Visual debugger has been enable in configuration, but feature has not been compiled. Please consider enabling \"visual-debugger\" feature.");
        }
    }
    info!("Server started in {}ms", _start.elapsed().as_millis());
    Server::start(server_ref_clone, client_notification_sender, single_client_notification_receiver, persistence_event_receiver, persistence_event_sender, true);
    map_proxy.shutdown();
    char_proxy.shutdown();
}

fn update_item_and_mob_static_db(items: &mut Vec<ItemModel>, mobs: &Vec<MobModel>) {
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
}

async fn compile_item_scripts(repository_arc: &Arc<PgRepository>, items: &mut Vec<ItemModel>) {
    let start = Instant::now();
    let mut script_compilation_to_update: Vec<(i32, Vec<u8>, u128)> = vec![];
    let mut item_script_compiled = 0;
    let mut item_script_skipped = 0;
    for item in items.iter_mut() {
        if let Some(script) = &item.script {
            let script_hash = fastmurmur3::hash(script.as_bytes());
            if item.script_compilation_hash.is_none() || script_hash != item.script_compilation_hash.unwrap() {
                let compilation_result = Compiler::compile_script_into_binary(format!("itemscript{}", item.id), script.as_str(), "./native_functions_list.txt", rathena_script_lang_interpreter::lang::compiler::DebugFlag::None.value());
                if let Ok(res) = compilation_result {
                    item_script_compiled += 1;
                    item.script_compilation_hash = Some(script_hash);
                    item.script_compilation = Some(general_purpose::STANDARD.encode(res.clone()));
                    script_compilation_to_update.push((item.id, res, script_hash));
                }
            } else {
                item_script_skipped += 1;
            }
        }
    }
    debug!("test log");
    repository_arc.update_script_compilation(script_compilation_to_update).await.unwrap();
    info!("Compiled {} item scripts compiled, skipped {} item scripts compilation (already compiled) in {}ms", item_script_compiled, item_script_skipped, start.elapsed().as_millis());
}

pub fn setup_logger(config: &'static Config) {
    let filter = EnvFilter::builder()
        .with_default_directive(config.server.log_level.as_ref().unwrap().to_lowercase().parse().unwrap())
        .parse_lossy(config.server.log_level_module_override.join(",").trim());
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stdout)
        .compact()
        .with_line_number(true)
        .with_thread_ids(false)
        .with_file(false)
        .with_thread_names(true)
        .with_target(true)
        .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string()))
        .init();
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
    info!("load {} scripts in {}ms", scripts.len(), start.elapsed().as_millis());

    Vm::bootstrap(vm, class_files, Box::new(&MapScriptHandler {}));
    scripts
}

pub fn configs() -> &'static Config {
    unsafe { CONFIGS.as_ref().unwrap() }
}


pub fn create_script_vm(native_function_file_path: &str) -> Arc<Vm> {
    Arc::new(Vm::new(native_function_file_path, rathena_script_lang_interpreter::lang::vm::DebugFlag::None.value()))
}
