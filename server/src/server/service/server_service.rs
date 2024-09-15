use std::sync::{Arc, Once};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use models::enums::action::ActionType;

use models::enums::EnumWithNumberValue;
use models::enums::skill::SkillType;
use packets::packets::{PacketZcNotifyAct};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::boot::map_loader::MapLoader;
use crate::server::model::map::{Map, RANDOM_CELL};
use crate::server::model::map_instance::{MapInstance};
use crate::server::model::map_item::{CHARACTER_MAX_MAP_ITEM_ID, MAP_INSTANCE_MAX_MAP_ITEM_ID, MapItems, MapItemSnapshot, MapItemType};
use models::position::Position;
use models::status::{Status, StatusSnapshot};
use crate::MAP_DIR;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
use crate::server::model::events::game_event::{CharacterAddItems, CharacterChangeMap, CharacterMovement, CharacterRemoveFromMap, CharacterUseSkill, GameEvent};
use crate::server::map_instance_loop::MapInstanceLoop;
use crate::server::model::action::{AddBonuses, Damage, SkillUsed};
use crate::server::model::events::map_event::{MapEvent};

use crate::server::model::movement::{Movable, Movement};
use crate::server::model::path::{manhattan_distance, path_search_client_side_algorithm};

use crate::server::service::battle_service::BattleService;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::server::state::character::Character;


use crate::server::state::server::ServerState;

use crate::util::tick::get_tick;

static mut SERVICE_INSTANCE: Option<ServerService> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

#[allow(dead_code)]
pub struct ServerService {
    client_notification_sender: SyncSender<Notification>,
    configuration_service: &'static GlobalConfigService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    movement_task_queue: Arc<TasksQueue<GameEvent>>,
    vm: Arc<Vm>,
    inventory_service: InventoryService,
    character_service: CharacterService,
    map_instance_service: MapInstanceService,
    skill_service: SkillService,
    battle_service: BattleService,
    status_service: &'static StatusService,
}

impl ServerService {
    pub fn instance() -> &'static ServerService {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub(crate) fn new(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, server_task_queue: Arc<TasksQueue<GameEvent>>, movement_task_queue: Arc<TasksQueue<GameEvent>>, vm: Arc<Vm>,
                      inventory_service: InventoryService, character_service: CharacterService, map_instance_service: MapInstanceService, battle_service: BattleService, skill_service: SkillService, status_service: &'static StatusService) -> Self {
        ServerService { client_notification_sender, configuration_service, server_task_queue, movement_task_queue, vm, inventory_service, character_service, map_instance_service, battle_service, skill_service, status_service }
    }

    pub fn init(client_notification_sender: SyncSender<Notification>, configuration_service: &'static GlobalConfigService, server_task_queue: Arc<TasksQueue<GameEvent>>, movement_task_queue: Arc<TasksQueue<GameEvent>>, vm: Arc<Vm>,
                inventory_service: InventoryService, character_service: CharacterService, map_instance_service: MapInstanceService, battle_service: BattleService, skill_service: SkillService, status_service: &'static StatusService) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(ServerService::new(client_notification_sender, configuration_service, server_task_queue, movement_task_queue, vm, inventory_service, character_service, map_instance_service, battle_service, skill_service, status_service));
        });
    }

    pub fn create_map_instance(&self, server_state: &mut ServerState, map: &'static Map, instance_id: u8) -> Arc<MapInstance> {
        info!("create map instance: {} x_size: {}, y_size {}, length: {}", map.name(), map.x_size(), map.y_size(), map.length());
        let start_sequence = CHARACTER_MAX_MAP_ITEM_ID + server_state.map_instances().len() as u32 * MAP_INSTANCE_MAX_MAP_ITEM_ID;
        let end_sequence = start_sequence + MAP_INSTANCE_MAX_MAP_ITEM_ID - 1;
        let mut map_items = MapItems::new(start_sequence, end_sequence);

        let mut cells = MapLoader::generate_cells(map.name(), map.length() as usize, unsafe { MAP_DIR });
        map.set_warp_cells(&mut cells, &mut map_items);

        let map_instance = MapInstance::from_map(self.vm.clone(), map, instance_id, cells, self.client_notification_sender.clone(), map_items, Arc::new(TasksQueue::new()));
        server_state.map_instances_count().fetch_add(1, Relaxed);
        let map_instance_ref = Arc::new(map_instance);
        let entry = server_state.map_instances_mut().entry(map.name().to_string()).or_default();
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

    pub fn character_attack(&self, server_state: &ServerState, tick: u128, character: &mut Character) {
        if !character.is_attacking() {
            return;
        }

        let map_item = server_state.map_item(character.attack().target, character.current_map_name(), character.current_map_instance());
        if let Some(map_item) = map_item {
            let range = character.status.attack_range();
            let target_position = server_state.map_item_x_y(&map_item, character.current_map_name(), character.current_map_instance()).unwrap();
            let is_in_range = range as i16 >= manhattan_distance(character.x, character.y, target_position.x, target_position.y) as i16 - 1;
            let maybe_map_instance = server_state.get_map_instance(character.current_map_name(), character.current_map_instance());
            let map_instance = maybe_map_instance.as_ref().unwrap();
            if !is_in_range && !character.is_moving() {
                let path = path_search_client_side_algorithm(map_instance.x_size(), map_instance.y_size(), map_instance.state().cells(), character.x, character.y, target_position.x, target_position.y);
                let path = Movement::from_path(path, tick);
                let current_position = Position { x: character.x, y: character.y, dir: 0 };
                debug!("Too far from target, moving from {} toward it: {}", current_position, target_position);
                self.movement_task_queue.add_to_first_index(GameEvent::CharacterMove(CharacterMovement {
                    char_id: character.char_id,
                    start_at: tick,
                    destination: target_position,
                    current_position,
                    path,
                    cancel_attack: false,
                }));
            } else if is_in_range {
                character.clear_movement();
                let snapshot = server_state.map_item_snapshot(map_item.id(), character.current_map_name(), character.current_map_instance()).unwrap();
                let mut maybe_damage = None;
                if matches!(*map_item.object_type(), MapItemType::Mob) {
                    let mob_status = server_state.map_item_mob_status(&map_item, character.current_map_name(), character.current_map_instance()).unwrap();
                    maybe_damage = self.battle_service.basic_attack(character, snapshot, &self.get_status_snapshot(&character.status, tick), &mob_status, tick);
                }
                if let Some(damage) = maybe_damage {
                    self.apply_damage(*map_item.object_type(), map_instance, damage);
                }
            }
        } else {
            character.clear_attack();
        }
    }

    fn apply_damage(&self, map_item_type: MapItemType, map_instance: &Arc<MapInstance>, damage: Damage) {
        if matches!(map_item_type, MapItemType::Mob) {
            map_instance.add_to_next_tick(MapEvent::MobDamage(damage));
        }
    }
    fn apply_bonuses(&self, map_item_type: MapItemType, _map_instance: &Arc<MapInstance>, add_bonuses: AddBonuses) {
        if matches!(map_item_type, MapItemType::Character) {
            self.server_task_queue.add_to_first_index(GameEvent::CharacterAddBonuses(add_bonuses));
        }
    }

    fn get_status_snapshot(&self, status: &Status, _tick: u128) -> StatusSnapshot {
        self.status_service.to_snapshot(status)
    }

    pub fn character_start_use_skill(&self, server_state: &ServerState, character: &mut Character, character_use_skill: CharacterUseSkill, tick: u128) {
        if character.is_using_skill() {
            return;
        }
        let target = Self::get_target(server_state, character, Some(character_use_skill.target_id));
        if target.is_none() {
            return;
        }
        let skill_use_response = self.skill_service.start_use_skill(character, target, &self.get_status_snapshot(&character.status, tick),
                                                                    self.get_target_status(server_state, character, Some(character_use_skill.target_id), tick).as_ref(), character_use_skill.skill_id, character_use_skill.skill_level, tick);
        self.after_used_skill(server_state, character, target, skill_use_response);
    }

    pub fn character_use_skill(&self, server_state: &ServerState, tick: u128, character: &mut Character) {
        if !character.is_using_skill() {
            return;
        }
        if character.skill_has_been_used() {
            self.skill_service.after_skill_used(character, tick);
        } else {
            let target = Self::get_target(server_state, character, character.skill_in_use().target);
            let skill_use_response = self.skill_service.do_use_skill(character, target, &self.get_status_snapshot(&character.status, tick), self.get_target_status(server_state, character, character.skill_in_use().target, tick).as_ref(), tick);
            self.after_used_skill(server_state, character, target, skill_use_response);
        }
    }

    fn after_used_skill(&self, server_state: &ServerState, character: &mut Character, target: Option<MapItemSnapshot>, skill_use_response: Option<SkillUsed>) {
        if let Some(skill_use_response) = skill_use_response {
            match skill_use_response.skill_type {
                SkillType::Offensive => {
                    let maybe_map_instance = server_state.get_map_instance(character.current_map_name(), character.current_map_instance());
                    let map_instance = maybe_map_instance.as_ref().unwrap();
                    self.apply_damage(*target.unwrap().map_item.object_type(), map_instance, skill_use_response.to_damage());
                }
                _ => {}
            }
            if !skill_use_response.bonuses.is_empty() {
                character.status.temporary_bonuses.merge(skill_use_response.bonuses)
                // TODO send packet
            }
        }
    }

    // TODO cache per tick
    fn get_target(server_state: &ServerState, character: &Character, target_id: Option<u32>) -> Option<MapItemSnapshot> {
        if let Some(target_id) = target_id {
            server_state.map_item_snapshot(target_id, character.current_map_name(), character.current_map_instance())
        } else {
            None
        }
    }

    // TODO cache per tick
    fn get_target_status(&self, server_state: &ServerState, character: &Character, target_id: Option<u32>, tick: u128) -> Option<StatusSnapshot> {
        if let Some(target_id) = target_id {
            let map_item = server_state.map_item(target_id, character.current_map_name(), character.current_map_instance());
            if let Some(map_item) = map_item {
                match map_item.object_type() {
                    MapItemType::Character => {
                        return Some(self.get_status_snapshot(&server_state.get_character_unsafe(target_id).status, tick));
                    }
                    MapItemType::Mob => {
                        return Some(server_state.map_item_mob_status(&map_item, character.current_map_name(), character.current_map_instance()).unwrap());
                    }
                    _ => { return None; }
                }
            }
            None
        } else {
            None
        }
    }

    pub fn character_pickup_item(&self, server_state: &mut ServerState, character: &mut Character, map_item_id: u32, map_instance: &MapInstance, runtime: &Runtime) {
        // Avoid item to be pickup twice
        if server_state.contains_locked_map_item(map_item_id) {
            warn!("Map item {} is planned to be removed from map instance, can't pick it", map_item_id);
            return;
        }
        // Limit pickable item on items present in player fov
        if character.is_map_item_in_fov(map_item_id) {
            if let Some(dropped_item) = map_instance.state().get_dropped_item(map_item_id) {
                if let Some(owner) = dropped_item.owner_id {
                    if owner != character.char_id && (get_tick() - dropped_item.dropped_at) < (self.configuration_service.config().game.mob_dropped_item_locked_to_owner_duration_in_secs as u128 * 1000) {
                        return;
                    }
                }
                server_state.insert_locked_map_item(map_item_id);
                let item = self.configuration_service.get_item(dropped_item.item_id);
                self.inventory_service.add_items_in_inventory(runtime, CharacterAddItems {
                    char_id: character.char_id,
                    should_perform_check: true,
                    buy: false,
                    items: vec![
                        InventoryItemModel::from_item_model(item, dropped_item.amount as i16, dropped_item.is_identified)
                    ],
                }, character);
                let mut packet_zc_notify_act = PacketZcNotifyAct::new(self.configuration_service.packetver());
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