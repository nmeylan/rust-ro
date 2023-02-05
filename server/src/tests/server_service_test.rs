use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use rathena_script_lang_interpreter::lang::vm::{DebugFlag, Vm};
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::server::service::server_service::ServerService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::mocked_repository::MockedRepository;

struct ServerServiceTestContext {
    test_context: TestContext,
    server_service: ServerService,
    client_notification_sender: SyncSender<Notification>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

fn before_each() -> ServerServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let server_task_queue = Arc::new(TasksQueue::new());
    ServerServiceTestContext {
        client_notification_sender: client_notification_sender.clone(),
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender: persistence_event_sender.clone(), client_notification_receiver, persistence_event_receiver },
        server_task_queue: server_task_queue.clone(),
        server_service: ServerService::new(client_notification_sender.clone(), GlobalConfigService::instance(), server_task_queue.clone(), Arc::new(Vm::new("../native_functions_list.txt", DebugFlag::None.value())),
                                           InventoryService::new(client_notification_sender.clone(), persistence_event_sender.clone(), Arc::new(MockedRepository::default()), GlobalConfigService::instance(), server_task_queue.clone()),
                                           CharacterService::new(client_notification_sender.clone(), persistence_event_sender.clone(), Arc::new(MockedRepository::default()), GlobalConfigService::instance()),
                                           MapInstanceService::new(client_notification_sender.clone(), GlobalConfigService::instance(), MobService::new(client_notification_sender.clone(), GlobalConfigService::instance()), server_task_queue.clone()), ),
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use crate::server::model::events::map_event::MapEvent;
    use crate::server::model::item::DroppedItem;
    use crate::server::model::map_item::ToMapItem;
    use crate::server::model::position::Position;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::tests::common::assert_helper::task_queue_contains_event_at_tick;
    use crate::tests::common::character_helper::create_character;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::common::server_helper::create_empty_server_state;
    use crate::tests::server_service_test::before_each;

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_when_item_in_fov() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(),Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, };
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
    fn character_pickup_item_should_prevent_pickup_when_item_not_in_fov() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(),Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        map_instance.state_mut().insert_dropped_item(DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, });
        // When
        context.server_service.character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, &runtime);
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
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
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, };
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
        task_queue_contains_event_at_tick::<MapEvent>(task_queue.clone(), MapEvent::RemoveDroppedItemFromMap(map_item_id), 0);
    }

    #[test]
    fn character_pickup_item_should_be_at_most_called_once() {
        // Given
        let context = before_each();
        let runtime = Runtime::new().unwrap();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(),Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, };
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
}
