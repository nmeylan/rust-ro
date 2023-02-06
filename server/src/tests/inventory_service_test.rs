use std::sync::Arc;
use crate::repository::InventoryRepository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::mocked_repository::MockedRepository;

struct InventoryServiceTestContext {
    test_context: TestContext,
    inventory_service: InventoryService,
    server_task_queue: Arc<TasksQueue<GameEvent>>
}

fn before_each(inventory_repository: Arc<dyn InventoryRepository>) -> InventoryServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let server_task_queue = Arc::new(TasksQueue::new());
    InventoryServiceTestContext {
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender: persistence_event_sender.clone(), client_notification_receiver, persistence_event_receiver },
        inventory_service: InventoryService::new(client_notification_sender, persistence_event_sender, inventory_repository, GlobalConfigService::instance(), server_task_queue.clone()),
        server_task_queue,
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::tests::common::mocked_repository::MockedRepository;
    use crate::tests::inventory_service_test::before_each;

    #[test]
    fn test_add_items_in_inventory_should_save_added_item_in_database() {
        // Given
        let context = before_each(Arc::new(MockedRepository::default()));

        // When

        // Then

    }

    #[test]
    fn test_add_items_in_inventory_should_trigger_update_zeny_if_it_is_a_buy() {
        // Given
        let context = before_each(mocked_repository());

        // When
        // Then

    }

    #[test]
    fn test_add_items_in_inventory_should_trigger_update_weight() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_reload_inventory_should_fetch_items_from_inventory_and_reset_in_memory_inventory() {
        // Given
        let context = before_each(Arc::new(MockedRepository::default()));

        // When

        // Then

    }

    #[test]
    fn test_reload_inventory_should_trigger_update_weight() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_reload_equipped_item_sprites_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_equip_item_should_equip_item_if_base_level_requirements_is_met() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_equip_item_should_equip_item_if_class_requirements_is_met() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then
    }

    #[test]
    fn test_equip_item_should_unequip_already_equipped_at_same_slot() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_equip_item_should_equip_two_accessory_in_the_two_slots() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_equip_item_should_unequip_left_accessory_when_two_accessory_are_already_equipped() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }

    #[test]
    fn test_takeoff_equip_item_should_trigger_stat_calculation() {
        // Given
        let context = before_each(mocked_repository());

        // When

        // Then

    }


    fn mocked_repository() -> Arc<MockedRepository> {
        Arc::new(MockedRepository::default())
    }
}