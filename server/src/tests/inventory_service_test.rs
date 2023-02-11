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
use crate::tests::common::sync_helper::CountDownLatch;

struct InventoryServiceTestContext {
    test_context: TestContext,
    inventory_service: InventoryService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

fn before_each(inventory_repository: Arc<dyn InventoryRepository + Sync>) -> InventoryServiceTestContext {
    before_each_with_latch(inventory_repository, 0)
}

fn before_each_with_latch(inventory_repository: Arc<dyn InventoryRepository + Sync>, latch_size: usize) -> InventoryServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let server_task_queue = Arc::new(TasksQueue::new());
    let count_down_latch = CountDownLatch::new(latch_size);
    InventoryServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch.clone()),
        inventory_service: InventoryService::new(client_notification_sender, persistence_event_sender, inventory_repository, GlobalConfigService::instance(), server_task_queue.clone()),
        server_task_queue,
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use async_trait::async_trait;
    use crate::{assert_sent_packet_in_current_packetver, assert_sent_persistence_event, assert_task_queue_contains_event_at_tick};
    use sqlx::Error;
    use sqlx::postgres::PgQueryResult;
    use tokio::runtime::Runtime;
    use enums::class::JobName;
    use packets::packets::PacketZcSpriteChange2;
    use packets::packets::PacketZcReqWearEquipAck2;
    use crate::enums::EnumWithNumberValue;
    use crate::server::model::events::game_event::CharacterZeny;
    use crate::repository::InventoryRepository;
    use crate::repository::model::item_model::InventoryItemModel;
    use crate::server::model::events::client_notification::{AreaNotification, AreaNotificationRangeType, Notification};
    use crate::server::model::events::game_event::CharacterAddItems;
    use crate::server::model::events::game_event::CharacterEquipItem;
    use crate::tests::inventory_service_test::GameEvent;
    use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate, PersistenceEvent};
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::common::assert_helper::{has_sent_notification, has_sent_persistence_event, NotificationExpectation, task_queue_contains_event_at_tick};
    use crate::tests::common::character_helper::{add_item_in_inventory, create_character, equip_item};
    use crate::tests::common::item_helper::create_inventory_item;
    use crate::tests::common::mocked_repository;
    use crate::tests::common::mocked_repository::MockedRepository;
    use crate::tests::common::sync_helper::CountDownLatch;
    use crate::tests::inventory_service_test::{before_each, before_each_with_latch};

    #[test]
    fn test_add_items_in_inventory_should_add_items_in_memory_save_added_item_in_database() {
        // Given
        struct MockedInventoryRepository {
            inventory_update_items: Mutex<Vec<InventoryItemUpdate>>,
        }
        ;
        #[async_trait]
        impl InventoryRepository for MockedInventoryRepository {
            async fn character_inventory_update(&self, inventory_update_items: &[InventoryItemUpdate], buy: bool) -> Result<(), Error> {
                let mut guard = self.inventory_update_items.lock().unwrap();
                guard.extend(inventory_update_items.to_vec());
                Ok(())
            }
        }
        let inventory_repository = Arc::new(MockedInventoryRepository { inventory_update_items: Mutex::new(vec![]) });
        let context = before_each(inventory_repository.clone());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_add_items = CharacterAddItems {
            char_id: character.char_id,
            should_perform_check: false,
            buy: false,
            items: vec![create_inventory_item("Blade", 1), create_inventory_item("Red_Potion", 10), create_inventory_item("Red_Potion", 10), create_inventory_item("Banana", 15)],
        };
        // When
        context.inventory_service.add_items_in_inventory(&runtime, character_add_items, &mut character);
        // Then
        let items_to_store_in_inventory_db = inventory_repository.inventory_update_items.lock().unwrap();
        assert_eq!(items_to_store_in_inventory_db.len(), 4);
        // Blade is a weapon which is not stackable so we generate a random unique id
        assert_ne!(items_to_store_in_inventory_db.iter().find(|item| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Blade")).unwrap().unique_id, 0);
        //Other item are stackable so we don't generate a random unique id
        assert_eq!(items_to_store_in_inventory_db.iter().find(|item| item.item_id as u32 != GlobalConfigService::instance().get_item_id_from_name("Blade")).unwrap().unique_id, 0);
        assert_eq!(character.inventory.len(), 3);
        let red_potion = character.inventory.iter().find(|item| item.as_ref().unwrap().item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Red_Potion")).unwrap().as_ref().unwrap();
        let blade = character.inventory.iter().find(|item| item.as_ref().unwrap().item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Blade")).unwrap().as_ref().unwrap();
        let banana = character.inventory.iter().find(|item| item.as_ref().unwrap().item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Banana")).unwrap().as_ref().unwrap();
        assert_eq!(red_potion.amount, 20);
        assert_eq!(blade.amount, 1);
        assert_eq!(banana.amount, 15);
    }

    #[test]
    fn test_add_items_in_inventory_should_trigger_update_zeny_if_it_is_a_buy() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_add_items = CharacterAddItems {
            char_id: character.char_id,
            should_perform_check: false,
            buy: true,
            items: vec![create_inventory_item("Red_Potion", 10)],
        };
        // When
        context.inventory_service.add_items_in_inventory(&runtime, character_add_items, &mut character);
        // Then
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterUpdateZeny(CharacterZeny { char_id: character.char_id, zeny: None }), 0);
    }

    #[test]
    fn test_add_items_in_inventory_should_trigger_update_weight() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_add_items = CharacterAddItems {
            char_id: character.char_id,
            should_perform_check: false,
            buy: false,
            items: vec![create_inventory_item("Red_Potion", 10)],
        };
        // When
        context.inventory_service.add_items_in_inventory(&runtime, character_add_items, &mut character);
        // Then
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterUpdateWeight(character.char_id), 0);
    }

    #[test]
    fn test_reload_inventory_should_fetch_items_from_inventory_db_and_reset_in_memory_inventory() {
        // Given
        struct MockedInventoryRepository {
            has_fetched_items: AtomicBool,
        }
        ;
        #[async_trait]
        impl InventoryRepository for MockedInventoryRepository {
            async fn character_inventory_update(&self, inventory_update_items: &[InventoryItemUpdate], buy: bool) -> Result<(), Error> {
                Ok(())
            }

            async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
                self.has_fetched_items.store(true, Ordering::Relaxed);
                Ok(vec![])
            }
        }
        let runtime = Runtime::new().unwrap();
        let inventory_repository = Arc::new(MockedInventoryRepository { has_fetched_items: Default::default() });
        let context = before_each(inventory_repository.clone());
        let mut character = create_character();
        let character_add_items = CharacterAddItems { char_id: character.char_id, should_perform_check: false, buy: false, items: vec![create_inventory_item("Red_Potion", 10)] };
        context.inventory_service.add_items_in_inventory(&runtime, character_add_items, &mut character);
        // When
        context.inventory_service.reload_inventory(&runtime, character.char_id, &mut character);
        // Then
        assert_eq!(character.inventory.len(), 0);
        assert!(inventory_repository.has_fetched_items.load(Ordering::Relaxed));
    }

    #[test]
    fn test_reload_inventory_should_trigger_update_weight() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        // When
        context.inventory_service.reload_inventory(&runtime, character.char_id, &mut character);
        // Then
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterUpdateWeight(character.char_id), 0);
    }

    #[test]
    fn test_reload_equipped_item_sprites_should_notify_area() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 1);
        let mut character = create_character();
        equip_item(&mut character, "Guard");
        equip_item(&mut character, "Knife");
        // When
        context.inventory_service.reload_equipped_item_sprites(&character);
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![PacketZcSpriteChange2::packet_id()]));
    }

    #[test]
    fn test_equip_item_should_not_equip_item_if_item_is_not_in_inventory() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: 0 });
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert!(character.inventory.is_empty());
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![PacketZcReqWearEquipAck2::packet_id()]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![]));
    }

    #[test]
    fn test_equip_item_should_equip_item_if_base_level_requirements_is_met() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();
        let item = GlobalConfigService::instance().get_item_by_name("Knife");
        let inventory_index = add_item_in_inventory(&mut character, "Knife");
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: inventory_index });
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert_eq!(character.inventory[inventory_index].as_ref().unwrap().equip, item.location as i32);
        assert_ne!(character.inventory[inventory_index].as_ref().unwrap().equip, 0);
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![PacketZcReqWearEquipAck2::packet_id()]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
    }

    #[test]
    fn test_equip_item_should_not_equip_item_if_base_level_requirements_is_not_met() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();
        character.status.base_level = 0;
        let item = GlobalConfigService::instance().get_item_by_name("Knife");
        let inventory_index = add_item_in_inventory(&mut character, "Knife");
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: inventory_index });
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert_eq!(character.inventory[inventory_index].as_ref().unwrap().equip, 0);
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![PacketZcReqWearEquipAck2::packet_id()]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
    }

    #[test]
    fn test_equip_item_should_equip_item_if_class_requirements_is_met() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();
        let item = GlobalConfigService::instance().get_item_by_name("Bow");
        character.status.base_level = (item.equip_level_min.unwrap_or(1) + 1) as u32;
        character.status.job = JobName::Archer.value() as u32;
        let inventory_index = add_item_in_inventory(&mut character, "Bow");
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: inventory_index });
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert_eq!(character.inventory[inventory_index].as_ref().unwrap().equip, item.location as i32);
        assert_ne!(character.inventory[inventory_index].as_ref().unwrap().equip, 0);
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![PacketZcReqWearEquipAck2::packet_id()]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
    }

    #[test]
    fn test_equip_item_should_not_equip_item_if_class_requirements_is_not_met() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();
        let item = GlobalConfigService::instance().get_item_by_name("Bow");
        character.status.base_level = (item.equip_level_min.unwrap_or(1) + 1) as u32;
        let inventory_index = add_item_in_inventory(&mut character, "Bow");
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: inventory_index });
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert_eq!(character.inventory[inventory_index].as_ref().unwrap().equip, 0);
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![PacketZcReqWearEquipAck2::packet_id()]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
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
}