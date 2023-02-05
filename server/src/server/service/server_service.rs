use std::collections::HashMap;
use std::sync::{Arc, Once};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use enums::action::ActionType;
use crate::enums::EnumWithNumberValue;
use packets::packets::PacketZcNotifyAct;
use crate::repository::ItemRepository;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::boot::map_loader::MapLoader;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::map_instance::MapInstance;
use crate::server::model::map_item::MapItem;
use crate::server::model::position::Position;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::{CharacterAddItems, CharacterChangeMap, CharacterRemoveFromMap, GameEvent};
use crate::server::map_instance_loop::MapInstanceLoop;
use crate::server::model::events::map_event::MapEvent;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::state::character::Character;
use crate::server::state::map_instance::MapInstanceState;
use crate::server::state::server::ServerState;

static mut SERVICE_INSTANCE: Option<ServerService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct ServerService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    vm: Arc<Vm>,
    inventory_service: InventoryService,
    character_service: CharacterService,
    map_instance_service: MapInstanceService,
}

impl ServerService {
    pub fn instance() -> &'static ServerService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, server_task_queue: Arc<TasksQueue<GameEvent>>, vm: Arc<Vm>,
                      inventory_service: InventoryService, character_service: CharacterService, map_instance_service: MapInstanceService) -> Self {
        ServerService { client_notification_sender, configuration_service, server_task_queue, vm, inventory_service, character_service, map_instance_service }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, server_task_queue: Arc<TasksQueue<GameEvent>>, vm: Arc<Vm>,
                inventory_service: InventoryService, character_service: CharacterService, map_instance_service: MapInstanceService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(ServerService::new(client_notification_sender, configuration_service, server_task_queue, vm, inventory_service, character_service, map_instance_service));
        });
    }

    pub fn create_map_instance(&self, server_state: &mut ServerState, map: &'static Map, instance_id: u8) -> Arc<MapInstance> {
        info!("create map instance: {} x_size: {}, y_size {}, length: {}", map.name(), map.x_size(), map.y_size(), map.length());
        let mut map_items: HashMap<u32, MapItem> = HashMap::with_capacity(2048);

        let mut cells = MapLoader::generate_cells(map.name(), map.length() as usize);
        map.set_warp_cells(&mut cells, &mut map_items);

        let map_instance = MapInstance::from_map(self.vm.clone(), map, instance_id, cells, self.client_notification_sender.clone(), map_items, Arc::new(TasksQueue::new()));
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

    pub fn character_pickup_item(&self, server_state: &mut ServerState, character: &mut Character, map_item_id: u32, map_instance: &MapInstance, runtime: &Runtime) {
        // Avoid item to be pickup twice
        if server_state.contains_locked_map_item(map_item_id) {
            warn!("Map item {} is planned to be removed from map instance, can't pick it", map_item_id);
            return;
        }
        // Limit pickable item on items present in player fov
        if character.is_map_item_in_fov(map_item_id) {
            // TODO also check if item is locked to owner
            if let Some(dropped_item) = map_instance.state().get_dropped_item(map_item_id) {
                server_state.insert_locked_map_item(map_item_id);
                let item = self.configuration_service.get_item(dropped_item.item_id);
                self.inventory_service.add_items_in_inventory(runtime, CharacterAddItems { char_id: character.char_id, should_perform_check: true, buy: false, items: vec![
                    InventoryItemModel::from_item_model(item, dropped_item.amount as i16, !item.item_type.should_be_identified_when_dropped())
                ] }, character);
                let mut packet_zc_notify_act = PacketZcNotifyAct::default();
                packet_zc_notify_act.set_gid(character.char_id);
                packet_zc_notify_act.set_action(ActionType::Itempickup.value() as u8);
                packet_zc_notify_act.fill_raw();
                self.client_notification_sender.send(Notification::Area(
                    AreaNotification::new(map_instance.key().map_name().clone(), map_instance.key().map_instance(),
                                          AreaNotificationRangeType::Fov { x: dropped_item.x(), y: dropped_item.y(), exclude_id: None },
                                          packet_zc_notify_act.raw))).expect("Fail to send client notification");
                map_instance.add_to_next_tick(MapEvent::RemoveDroppedItemFromMap(map_item_id));
            }
        } else {
            warn!("Character {} tried to loot item with map item id {} not in his fov", character.char_id, map_item_id);
        }
    }
}