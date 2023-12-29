use std::collections::{HashMap, HashSet};
use std::{env, fs, thread};
use std::sync::{Arc, Mutex, Once};
use flexi_logger::Logger;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use testcontainers::{Container, RunnableImage};
use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};
use tokio::runtime::Runtime;
use configuration::configuration::{DatabaseConfig};
use models::status::KnownSkill;
use crate::MAP_DIR;
use crate::repository::{CharacterRepository, Repository};
use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::mob_model::{MobModel, MobModels};
use crate::server::boot::map_loader::MapLoader;
use crate::server::boot::mob_spawn_loader::MobSpawnLoader;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent::CharacterJoinGame;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::map::Map;
use crate::server::model::map_instance::MapInstanceKey;
use crate::server::model::map_item::MapItem;
use crate::server::model::status::StatusFromDb;
use crate::server::script::ScriptGlobalVariableStore;
use crate::server::Server;
use crate::server::service::server_service::ServerService;
use crate::server::state::character::Character;
use crate::tests::common;
use crate::tests::common::{CONFIGS, create_mpsc};
use crate::util::log_filter::LogFilter;

static INIT: Once = Once::new();
pub static mut SERVER: Option<Arc<Server>> = None;
pub static mut POSTGRES_CONTAINER: Option<Container<Postgres>> = None;
pub static mut DOCKER_CLI: Option<Cli> = None;


pub async fn before_all() -> Arc<Server> {
    INIT.call_once(|| unsafe {
        common::before_all();
        MAP_DIR = "../config/maps/pre-re";
        let logger = Logger::try_with_str(CONFIGS.as_ref().unwrap().server.log_level.as_ref().unwrap()).unwrap();
        logger.filter(Box::new(LogFilter::new(CONFIGS.as_ref().unwrap().server.log_exclude_pattern.as_ref().unwrap().clone()))).start().unwrap();
        let vm = Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value()));

        DOCKER_CLI = Some(Cli::default());
        let image = RunnableImage::from(Postgres::default()).with_tag("15-alpine")
            .with_volume((env::current_dir().unwrap().join("../db/pg.sql").to_str().unwrap(), "/pg.sql"))
            .with_volume((env::current_dir().unwrap().join("../docker/volumes/create_role.sql").to_str().unwrap(), "/create_role.sql"))
            .with_volume((env::current_dir().unwrap().join("../docker/volumes/init.sh").to_str().unwrap(), "/docker-entrypoint-initdb.d/init.sh"))
            ;
        let node = DOCKER_CLI.as_ref().unwrap().run(image);

        let database_config = DatabaseConfig {
            db: "ragnarok".to_string(),
            host: "127.0.0.1".to_string(),
            port: node.get_host_port_ipv4(5432),
            username: "ragnarok".to_string(),
            password: Some("ragnarok".to_string()),
        };
        POSTGRES_CONTAINER = Some(node);
        let repository: Repository = Repository::new_pg_lazy(&database_config, Runtime::new().unwrap());
        let repository_arc = Arc::new(repository);
        let mut map_item_ids = HashMap::<u32, MapItem>::new();

        let mob_models = serde_json::from_str::<MobModels>(&fs::read_to_string("../config/mobs.json").unwrap());
        let mobs: Vec<MobModel> = mob_models.unwrap().into();

        let mobs_map = mobs.clone().into_iter().map(|mob| (mob.id as u32, mob)).collect();
        let mob_spawns = unsafe { MobSpawnLoader::load_mob_spawns(CONFIGS.as_ref().unwrap(), mobs_map, "../config/npc").join().unwrap() };
        let maps = MapLoader::load_maps(Default::default(), mob_spawns, Default::default(), &mut map_item_ids, "../config/maps/pre-re");
        unsafe {
            crate::GlobalConfigService::instance_mut().maps = maps;
        }
        let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
        let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
        let server = Server::new(CONFIGS.as_ref().unwrap(), repository_arc.clone(), map_item_ids, vm, client_notification_sender, persistence_event_sender.clone());
        SERVER = Some(Arc::new(server));
        thread::spawn(move || {
            info!("Starting server");
            Server::start(SERVER.clone().unwrap(), client_notification_receiver, persistence_event_receiver, persistence_event_sender, false);
        });
    });
    return server();
}

pub fn server() -> Arc<Server> {
    unsafe { SERVER.clone().unwrap() }
}

pub async fn character_join_game() {
    let server = server();
    let char_model: CharSelectModel = server.repository.character_fetch(2000000, 0).await.unwrap();
    let char_id = char_model.char_id as u32;
    let skills: Vec<KnownSkill> = server.repository.character_skills(char_id).await.unwrap();
    let character = Character {
        name: char_model.name.clone(),
        char_id,
        status: StatusFromDb::from_char_model(&char_model, &server.configuration.game, skills),
        loaded_from_client_side: true,
        x: char_model.last_x as u16,
        y: char_model.last_y as u16,
        dir: 0,
        movements: vec![],
        attack: None,
        skill_in_use: None,
        inventory: vec![],
        map_view: HashSet::new(),
        script_variable_store: Mutex::new(ScriptGlobalVariableStore::default()),
        account_id: char_model.account_id as u32,
        map_instance_key: MapInstanceKey::new("prt_fild09".to_string(), 0),
        last_moved_at: 0,
    };
    server.state_mut().insert_character(character);
    let character = server.state().get_character_unsafe(char_id);
    server.add_to_next_tick(CharacterJoinGame(character.char_id));
    ServerService::instance().schedule_warp_to_walkable_cell(server.state_mut().as_mut(), &Map::name_without_ext(character.current_map_name()), character.x(), character.y(), char_id);
}