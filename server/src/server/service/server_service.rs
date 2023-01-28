use std::collections::HashMap;
use std::sync::{Arc, Once};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::vm::Vm;
use crate::repository::ItemRepository;
use crate::server::core::map::{Map, RANDOM_CELL};
use crate::server::core::map_instance::MapInstance;
use crate::server::core::map_item::MapItem;
use crate::server::core::position::Position;
use crate::server::core::tasks_queue::TasksQueue;
use crate::server::events::client_notification::Notification;
use crate::server::events::game_event::{CharacterChangeMap, CharacterRemoveFromMap, GameEvent};
use crate::server::map_instance_loop::MapInstanceLoop;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::server::ServerState;

static mut SERVICE_INSTANCE: Option<ServerService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct ServerService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    repository: Arc<dyn ItemRepository>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    vm: Arc<Vm>,
}

impl ServerService {
    pub fn instance() -> &'static ServerService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, repository: Arc<dyn ItemRepository>, server_task_queue: Arc<TasksQueue<GameEvent>>, vm: Arc<Vm>) -> Self {
        ServerService { client_notification_sender, configuration_service, repository, server_task_queue, vm }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, repository: Arc<dyn ItemRepository>, server_task_queue: Arc<TasksQueue<GameEvent>>, vm: Arc<Vm>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(ServerService::new(client_notification_sender, configuration_service, repository, server_task_queue, vm));
        });
    }

    pub fn create_map_instance(&self, server_state: &mut ServerState, map: &'static Map, instance_id: u8) -> Arc<MapInstance> {
        info!("create map instance: {} x_size: {}, y_size {}, length: {}", map.name(), map.x_size(), map.y_size(), map.length());
        let mut map_items: HashMap<u32, MapItem> = HashMap::with_capacity(2048);
        let cells = map.generate_cells(&mut map_items);

        let map_instance = MapInstance::from_map(self.vm.clone(), map, instance_id, cells, self.client_notification_sender.clone(), map_items);
        server_state.map_instances_count().fetch_add(1, Relaxed);
        let map_instance_ref = Arc::new(map_instance);
        let entry = server_state.map_instances_mut().entry(map.name().to_string()).or_insert(Default::default());
        entry.push(map_instance_ref.clone());
        MapInstanceLoop::start_map_instance_thread(map_instance_ref.clone());
        map_instance_ref
    }

    pub fn schedule_warp_to_walkable_cell(&self, server_state: &mut ServerState, destination_map: &str, x: u16, y: u16, char_id: u32) {
        self.server_task_queue.add_to_first_index(GameEvent::CharacterClearFov(char_id));
        let character_ref = server_state.get_character_unsafe(char_id);
        self.server_task_queue.add_to_index(GameEvent::CharacterRemoveFromMap(CharacterRemoveFromMap { char_id, map_name: character_ref.current_map_name().clone(), instance_id: character_ref.current_map_instance() }), 0);

        let map_name: String = Map::name_without_ext(destination_map);
        debug!("Char enter on map {}", map_name);
        let map_instance = server_state.get_map_instance(&map_name, 0)
            .unwrap_or_else(|| ServerService::instance().create_map_instance(server_state, self.configuration_service.get_map(map_name.as_str()), 0));
        let (x, y) = if x == RANDOM_CELL.0 && y == RANDOM_CELL.1 {
            let walkable_cell = Map::find_random_walkable_cell(map_instance.state().cells(), map_instance.x_size());
            (walkable_cell.0, walkable_cell.1)
        } else {
            (x, y)
        };

        self.server_task_queue.add_to_index(GameEvent::CharacterChangeMap(CharacterChangeMap {
            char_id,
            new_map_name: destination_map.to_owned(),
            new_instance_id: map_instance.id(),
            new_position: Some(Position { x, y, dir: 3 }),
        }), 2);
    }

}