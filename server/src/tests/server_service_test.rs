#![allow(dead_code)]
use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::Server;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::server::service::server_service::ServerService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::mocked_repository::MockedRepository;
use crate::tests::common::sync_helper::CountDownLatch;

struct ServerServiceTestContext {
    test_context: TestContext,
    server_service: ServerService,
    client_notification_sender: SyncSender<Notification>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    movement_task_queue: Arc<TasksQueue<GameEvent>>,
}

fn before_each() -> ServerServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> ServerServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let server_task_queue = Arc::new(TasksQueue::new());
    let movement_task_queue = Arc::new(TasksQueue::new());
    let count_down_latch = CountDownLatch::new(latch_size);
    StatusService::init(GlobalConfigService::instance(), "../native_functions_list.txt");
    ServerServiceTestContext {
        client_notification_sender: client_notification_sender.clone(),
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        server_task_queue: server_task_queue.clone(),
        movement_task_queue: movement_task_queue.clone(),
        server_service: ServerService::new(client_notification_sender.clone(), GlobalConfigService::instance(), server_task_queue.clone(), movement_task_queue, Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value())),
                                           InventoryService::new(client_notification_sender.clone(), persistence_event_sender.clone(), Arc::new(MockedRepository), GlobalConfigService::instance(), server_task_queue.clone()),
                                           CharacterService::new(client_notification_sender.clone(), persistence_event_sender.clone(), Arc::new(MockedRepository), GlobalConfigService::instance(),
                                                                 SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()), StatusService::instance(), server_task_queue.clone()),
                                           MapInstanceService::new(client_notification_sender.clone(), GlobalConfigService::instance(), MobService::new(client_notification_sender.clone(), GlobalConfigService::instance()), server_task_queue),
                                           BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal),
                                           SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal), StatusService::instance(),GlobalConfigService::instance()),
                                           StatusService::instance(),
        ),
    }
}


#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use crate::server::model::events::map_event::{MapEvent};
    use models::item::DroppedItem;
    use crate::server::model::map_item::ToMapItem;
    use models::position::Position;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::Server;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::common::assert_helper::task_queue_contains_event_at_tick;
    use crate::tests::common::character_helper::{create_character};
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::common::mocked_repository;
    use crate::tests::common::server_helper::create_empty_server_state;
    use crate::tests::server_service_test::before_each;
    use crate::util::tick::get_tick;

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_when_item_in_fov() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, 501);
        assert_eq!(item_from_inventory.amount, 2);
    }

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_and_keep_is_identified_status_from_item_drop() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let red_potion_map_item_id = 1000;
        let clover_map_item_id = 1001;
        let knife_map_item_id = 1002;
        let red_potion = DroppedItem { map_item_id: red_potion_map_item_id, item_id: GlobalConfigService::instance().get_item_id_from_name("Red_Potion") as i32, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        let clover = DroppedItem { map_item_id: clover_map_item_id, item_id: GlobalConfigService::instance().get_item_id_from_name("Clover") as i32, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        let knife = DroppedItem { map_item_id: knife_map_item_id, item_id: GlobalConfigService::instance().get_item_id_from_name("Knife") as i32, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 1, is_identified: false };
        // Add dropped item in character fov
        character_state.map_view.insert(red_potion.to_map_item());
        map_instance.state_mut().insert_dropped_item(red_potion);
        character_state.map_view.insert(clover.to_map_item());
        map_instance.state_mut().insert_dropped_item(clover);
        character_state.map_view.insert(knife.to_map_item());
        map_instance.state_mut().insert_dropped_item(knife);
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, red_potion_map_item_id, &map_instance, &runtime);
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, clover_map_item_id, &map_instance, &runtime);
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, knife_map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, GlobalConfigService::instance().get_item_id_from_name("Red_Potion") as i32);
        assert!(item_from_inventory.is_identified);
        let item_from_inventory = character_state.get_item_from_inventory(1).unwrap();
        assert_eq!(item_from_inventory.item_id, GlobalConfigService::instance().get_item_id_from_name("Clover") as i32);
        assert!(item_from_inventory.is_identified);
        let item_from_inventory = character_state.get_item_from_inventory(2).unwrap();
        assert_eq!(item_from_inventory.item_id, GlobalConfigService::instance().get_item_id_from_name("Knife") as i32);
        assert!(!item_from_inventory.is_identified);
    }

    #[test]
    fn character_pickup_item_should_prevent_pickup_when_item_not_in_fov() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        map_instance.state_mut().insert_dropped_item(DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true });
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
    }


    #[test]
    fn character_pickup_item_should_prevent_pickup_when_item_is_still_locked_by_another_player() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: Some(15001), dropped_at: get_tick() - 10, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
    }

    #[test]
    fn character_pickup_item_should_pickup_when_item_is_no_longer_locked_by_another_player() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: Some(15001), dropped_at: get_tick() - 10 - (GlobalConfigService::instance().config().game.mob_dropped_item_locked_to_owner_duration_in_secs as u128 * 1000), amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_some());
    }

    #[test]
    fn character_pickup_item_should_remove_map_item_from_map_instance() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let task_queue = Arc::new(TasksQueue::new());
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), task_queue.clone());
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        assert!(map_instance.state().get_map_item(map_item_id).is_some());
        assert!(map_instance.state().get_dropped_item(map_item_id).is_some());
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_some());
        task_queue_contains_event_at_tick::<MapEvent>(task_queue, MapEvent::RemoveDroppedItemFromMap(map_item_id), 0);
    }

    #[test]
    fn character_pickup_item_should_be_at_most_called_once() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, 501);
        assert_eq!(item_from_inventory.amount, 2);
    }

    #[test]
    fn character_use_support_skill_should_send_add_bonuses_packet() {
        // Given
        // Server::new_without_service_init(GlobalConfigService::instance(), mocked_repository(), )
        // When
        // Then
    }

}
