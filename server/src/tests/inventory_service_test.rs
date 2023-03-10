use std::sync::Arc;
use crate::repository::InventoryRepository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::global_config_service::GlobalConfigService;

use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};

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
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        inventory_service: InventoryService::new(client_notification_sender, persistence_event_sender, inventory_repository, GlobalConfigService::instance(), server_task_queue.clone()),
        server_task_queue,
    }
}


#[cfg(test)]
mod tests {
    
    use std::sync::{Arc, Mutex};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;
    use async_trait::async_trait;
    use crate::{assert_not_sent_packet_in_current_packetver, assert_sent_packet_in_current_packetver, assert_sent_persistence_event, assert_task_queue_contains_event, assert_task_queue_contains_event_at_tick, assert_task_queue_does_not_contains_event, assert_task_queue_is_empty};
    use crate::tests::common::assert_helper::task_queue_contains_event;
    use crate::tests::common::assert_helper::task_queue_not_contains_event;
    use sqlx::Error;
    
    use tokio::runtime::Runtime;
    use enums::class::JobName;
    use enums::item::EquipmentLocation;
    use packets::packets::{PacketZcReqTakeoffEquipAck2, PacketZcSpriteChange2, PacketZcItemThrowAck, PacketZcItemFallEntry};
    use packets::packets::PacketZcReqWearEquipAck2;
    use crate::enums::EnumWithNumberValue;
    use crate::enums::EnumWithMaskValueU64;
    use crate::enums::EnumWithStringValue;
    use crate::server::model::events::game_event::{CharacterRemoveItem, CharacterRemoveItems, CharacterZeny};
    use crate::repository::InventoryRepository;
    use crate::repository::model::item_model::InventoryItemModel;
    use crate::repository::persistence_error::PersistenceError;
    use crate::server::model::events::map_event::MapEvent;
    use crate::server::model::events::game_event::CharacterAddItems;
    use crate::server::model::events::game_event::CharacterEquipItem;
    use crate::server::model::events::map_event::CharacterDropItems;
    use crate::tests::inventory_service_test::GameEvent;
    use crate::server::model::events::persistence_event::{InventoryItemUpdate, PersistenceEvent};
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::state::character::Character;
    use crate::tests::common::assert_helper::{has_sent_notification, has_sent_persistence_event, NotificationExpectation, task_queue_contains_event_at_tick, SentPacket};
    use crate::tests::common::character_helper::{add_item_in_inventory, add_items_in_inventory, create_character, equip_item};
    use crate::tests::common::item_helper::create_inventory_item;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::common::mocked_repository;
    
    
    use crate::tests::inventory_service_test::{before_each, before_each_with_latch};

    #[test]
    fn test_add_items_in_inventory_should_add_items_in_memory_save_added_item_in_database() {
        // Given
        struct MockedInventoryRepository {
            inventory_update_items: Mutex<Vec<InventoryItemUpdate>>,
        }

        #[async_trait]
        impl InventoryRepository for MockedInventoryRepository {
            async fn character_inventory_update_add(&self, inventory_update_items: &[InventoryItemUpdate], _buy: bool) -> Result<(), Error> {
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
            async fn character_inventory_update_add(&self, _inventory_update_items: &[InventoryItemUpdate], _buy: bool) -> Result<(), Error> {
                Ok(())
            }

            async fn character_inventory_fetch(&self, _char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
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
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id(GlobalConfigService::instance().packetver()))]));
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
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqWearEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
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
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqWearEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
    }

    #[test]
    fn test_equip_item_should_not_equip_item_if_base_level_requirements_is_not_met() {
        // Given
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();
        character.status.base_level = 0;
        let _item = GlobalConfigService::instance().get_item_by_name("Knife");
        let inventory_index = add_item_in_inventory(&mut character, "Knife");
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: inventory_index });
        // Then
        context.test_context.countdown_latch().wait_with_timeout(Duration::from_millis(200));
        assert_eq!(character.inventory[inventory_index].as_ref().unwrap().equip, 0);
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqWearEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
    }

    #[test]
    fn test_check_job_requirement() {
        // Given
        struct JobWeapon<'a> {job: &'a str, weapon: &'a str, expected_can_equip: bool}
        let jobs_weapons = vec![
            JobWeapon{ job: "Archer", weapon: "Bow", expected_can_equip: true},
            JobWeapon{ job: "Archer", weapon: "Sword", expected_can_equip: false},
            JobWeapon{ job: "Bard", weapon: "Whip", expected_can_equip: false},
            JobWeapon{ job: "Bard", weapon: "Guitar", expected_can_equip: true},
            JobWeapon{ job: "Clown", weapon: "Guitar", expected_can_equip: true},
            JobWeapon{ job: "Dancer", weapon: "Whip", expected_can_equip: true},
            JobWeapon{ job: "Dancer", weapon: "Guitar", expected_can_equip: false},
        ];
        let context = before_each_with_latch(mocked_repository(), 2);
        let mut character = create_character();

        for job_weapon in jobs_weapons {
            let item = GlobalConfigService::instance().get_item_by_name(job_weapon.weapon);
            character.status.job = JobName::from_string(job_weapon.job).value() as u32;
            // When
            let res = context.inventory_service.check_job_requirement(&character, item);
            // Then
            assert_eq!(res, job_weapon.expected_can_equip, "Expected {} to be {} by {} but was not", item.name_aegis, if job_weapon.expected_can_equip { "wearable"} else { "not wearable"}, job_weapon.job);
        }
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
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqWearEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
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
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqWearEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateEquippedItems(vec![character.inventory[inventory_index].as_ref().unwrap().clone()]));
    }

    #[test]
    fn test_equip_item_should_unequip_already_equipped_at_same_slot() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job = JobName::Crusader.value() as u32;
        character.status.base_level = 80;
        let char_id = character.char_id;
        let knife_index = add_item_in_inventory(&mut character, "Knife");
        let sword_index = add_item_in_inventory(&mut character, "Sword");
        let guard_index = add_item_in_inventory(&mut character, "Guard");
        let two_h_sword_index = add_item_in_inventory(&mut character, "Two_Hand_Sword");
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: knife_index });
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 1);
        assert!(character.inventory_equipped().any(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Knife")));
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_not_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqTakeoffEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
        context.test_context.clear_sent_packet();

        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: sword_index });
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 1);
        assert!(character.inventory_equipped().any(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Sword")));
        context.test_context.increment_latch().wait_expected_count_with_timeout(4, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcReqTakeoffEquipAck2::packet_id(GlobalConfigService::instance().packetver()), 1)]));
        context.test_context.clear_sent_packet();

        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: guard_index });
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 2);
        assert!(character.inventory_equipped().any(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Guard")));
        assert!(character.inventory_equipped().any(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Sword")));
        context.test_context.increment_latch().wait_expected_count_with_timeout(6, Duration::from_millis(200));
        assert_not_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcReqTakeoffEquipAck2::packet_id(GlobalConfigService::instance().packetver()))]));
        context.test_context.clear_sent_packet();

        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: two_h_sword_index });
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 1);
        assert!(character.inventory_equipped().any(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Two_Hand_Sword")));
        context.test_context.increment_latch().wait_expected_count_with_timeout(8, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcReqTakeoffEquipAck2::packet_id(GlobalConfigService::instance().packetver()), 2)]));
        context.test_context.clear_sent_packet();
    }

    #[test]
    fn test_equip_item_should_equip_two_accessory_in_the_two_slots() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job = JobName::Hunter.value() as u32;
        character.status.base_level = 90;
        let char_id = character.char_id;
        let glove_index = add_item_in_inventory(&mut character, "Glove");
        let rosary_index = add_item_in_inventory(&mut character, "Rosary");
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: glove_index });
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: rosary_index });
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 2);
        let glove_item = character.inventory_equipped().find(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Glove"));
        let rosary_item = character.inventory_equipped().find(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Rosary"));
        assert!(glove_item.is_some());
        assert!(glove_item.unwrap().1.equip == EquipmentLocation::AccessoryLeft.as_flag() as i32);
        assert!(rosary_item.is_some());
        assert!(rosary_item.unwrap().1.equip == EquipmentLocation::AccessoryRight.as_flag() as i32);
    }

    #[test]
    fn test_equip_item_should_unequip_left_accessory_when_two_accessory_are_already_equipped() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job = JobName::Hunter.value() as u32;
        character.status.base_level = 90;
        let char_id = character.char_id;
        let glove_index = add_item_in_inventory(&mut character, "Glove");
        let rosary_index = add_item_in_inventory(&mut character, "Rosary");
        let belt_index = add_item_in_inventory(&mut character, "Belt");
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: glove_index });
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: rosary_index });
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: belt_index });
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 2);
        let belt_item = character.inventory_equipped().find(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Belt"));
        let rosary_item = character.inventory_equipped().find(|(_, item)| item.item_id as u32 == GlobalConfigService::instance().get_item_id_from_name("Rosary"));
        assert!(belt_item.is_some());
        assert!(belt_item.unwrap().1.equip == EquipmentLocation::AccessoryLeft.as_flag() as i32);
        assert!(rosary_item.is_some());
        assert!(rosary_item.unwrap().1.equip == EquipmentLocation::AccessoryRight.as_flag() as i32);
    }

    #[test]
    fn test_equip_item_should_trigger_stat_calculation() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let inventory_index = add_item_in_inventory(&mut character, "Knife");
        let char_id = character.char_id;
        // When
        context.inventory_service.equip_item(&mut character, CharacterEquipItem { char_id, index: inventory_index });
        // Then
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterCalculateStats(char_id), 0);
    }

    #[test]
    fn test_takeoff_equip_item_should_unequip_item() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let _char_id = character.char_id;
        let knife_index = equip_item(&mut character, "Knife");
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 1);
        // When
        context.inventory_service.takeoff_equip_item(&mut character, knife_index);
        // Then
        assert_eq!(character.inventory_equipped().collect::<Vec<_>>().len(), 0);
    }

    #[test]
    fn test_takeoff_equip_item_should_trigger_stat_calculation() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let char_id = character.char_id;
        let knife_index = equip_item(&mut character, "Knife");
        // When
        context.inventory_service.takeoff_equip_item(&mut character, knife_index);
        // Then
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterCalculateStats(char_id), 0);
    }

    #[test]
    fn test_remove_item_from_inventory_should_remove_it_from_inventory_and_save_in_db() {
        // Given
        struct MockedInventoryRepository {
            inventory_update_items: Mutex<Vec<InventoryItemModel>>,
        }

        #[async_trait]
        impl InventoryRepository for MockedInventoryRepository {
            async fn character_inventory_update_remove(&self, inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>, _buy: bool) -> Result<(), Error> {
                let mut guard = self.inventory_update_items.lock().unwrap();
                guard.extend(inventory_update_items.iter().map(|(item, _)| (*item).clone()).collect::<Vec<InventoryItemModel>>());
                Ok(())
            }
        }
        let inventory_repository = Arc::new(MockedInventoryRepository { inventory_update_items: Mutex::new(vec![]) });
        let context = before_each(inventory_repository.clone());
        let runtime = Runtime::new().unwrap();
        let mut character = create_character();
        add_items_in_inventory(&mut character, "Jellopy", 10);
        add_items_in_inventory(&mut character, "Knife", 1);
        // When
        context.inventory_service.remove_item_from_inventory(&runtime, CharacterRemoveItems{
            char_id: character.char_id,
            sell: false,
            items: vec![CharacterRemoveItem{ char_id: character.char_id, index: 0, amount: 7, price: 0 }, CharacterRemoveItem{ char_id: character.char_id, index: 1, amount: 1, price: 0 },]
        }, &mut character);
        // Then
        assert_eq!(character.inventory.len(), 2);
        character.inventory.iter().for_each(|i| println!("{:?}", i));
        assert!(character.get_item_from_inventory(0).is_some());
        assert_eq!(character.get_item_from_inventory(0).unwrap().name_english, "Jellopy".to_string());
        assert_eq!(character.get_item_from_inventory(0).unwrap().amount, 3);
        assert!(character.get_item_from_inventory(1).is_none());
        let repository_guard = inventory_repository.inventory_update_items.lock().unwrap();
        assert_eq!(repository_guard.len(), 2);
        assert_eq!(repository_guard[0].name_english, "Jellopy");
        assert_eq!(repository_guard[0].amount, 3);
        assert_eq!(repository_guard[1].name_english, "Knife");
        assert_eq!(repository_guard[1].amount, 0);
    }

    #[test]
    fn test_remove_item_from_inventory_should_notify_char_weight_update_and_item_removed_from_inventory() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        add_items_in_inventory(&mut character, "Jellopy", 10);
        add_items_in_inventory(&mut character, "Knife", 1);
        // When
        context.inventory_service.remove_item_from_inventory(&runtime, CharacterRemoveItems{
            char_id: character.char_id,
            sell: false,
            items: vec![CharacterRemoveItem{ char_id: character.char_id, index: 0, amount: 7, price: 0 }, CharacterRemoveItem{ char_id: character.char_id, index: 1, amount: 1, price: 0 },]
        }, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterUpdateWeight(character.char_id), 0);
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcItemThrowAck::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_remove_item_from_inventory_should_return_inventory_item_with_removal_information() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        add_items_in_inventory(&mut character, "Jellopy", 10);
        add_items_in_inventory(&mut character, "Knife", 1);
        // When
        let inventory_items = context.inventory_service.remove_item_from_inventory(&runtime, CharacterRemoveItems {
            char_id: character.char_id,
            sell: false,
            items: vec![CharacterRemoveItem { char_id: character.char_id, index: 0, amount: 7, price: 0 }, CharacterRemoveItem { char_id: character.char_id, index: 1, amount: 1, price: 0 }, ]
        }, &mut character).unwrap();
        // Then
        let inventory_item_model = &inventory_items[0].0;
        let removal_information = &inventory_items[0].1;
        assert_eq!(inventory_item_model.item_id, GlobalConfigService::instance().get_item_id_from_name("Jellopy") as i32);
        assert_eq!(removal_information.amount, 7);
        assert_eq!(removal_information.index, 0);
        let inventory_item_model = &inventory_items[1].0;
        let removal_information = &inventory_items[1].1;
        assert_eq!(inventory_item_model.item_id, GlobalConfigService::instance().get_item_id_from_name("Knife") as i32);
        assert_eq!(removal_information.amount, 1);
        assert_eq!(removal_information.index, 1);
    }

    #[test]
    fn test_character_drop_items_should_trigger_map_instance_character_drop_item_event() {
        // Given
        let context = before_each(mocked_repository());
        let runtime = Runtime::new().unwrap();
        let mut character = create_character();
        let task_queue = Arc::new(TasksQueue::new());
        let map_instance = create_empty_map_instance(context.test_context.client_notification_sender(), task_queue.clone());
        let index = add_items_in_inventory(&mut character, "Jellopy", 10);
        let mut jellopy_item_model = character.get_item_from_inventory(index).unwrap().clone();
        let index = add_items_in_inventory(&mut character, "Knife", 1);
        let mut knife_item_model = character.get_item_from_inventory(index).unwrap().clone();
        let char_id = character.char_id;
        // When
        context.inventory_service.character_drop_items(&runtime, &mut character, CharacterRemoveItems {
            char_id,
            sell: false,
            items: vec![CharacterRemoveItem { char_id, index: 0, amount: 7, price: 0 }, CharacterRemoveItem { char_id, index: 1, amount: 1, price: 0 }],
        }, &map_instance);
        // Then
        // We should update amount from cloned inventory item for the comparison below
        jellopy_item_model.amount = 3; // 10 - 7
        knife_item_model.amount = 0; // 1 - 1
        let drop_items = CharacterDropItems {
            owner_id: char_id,
            char_x: character.x,
            char_y: character.y,
            item_removal_info: vec![(jellopy_item_model, CharacterRemoveItem { char_id, index: 0, amount: 7, price: 0 }), (knife_item_model, CharacterRemoveItem { char_id, index: 1, amount: 1, price: 0 })]
        };
        assert_task_queue_contains_event!(task_queue.clone(), MapEvent::CharDropItems(drop_items));
    }
    #[test]
    fn test_character_drop_items_should_not_trigger_map_instance_character_drop_item_event_when_there_is_a_database_error() {
        // Given
        struct MockedInventoryRepository {}

        #[async_trait]
        impl InventoryRepository for MockedInventoryRepository {
            async fn character_inventory_update_remove(&self, inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>, _buy: bool) -> Result<(), Error> {
                Err(Error::Database(Box::new(PersistenceError::new("mocked error".to_string()))))
            }
        }
        let context = before_each(Arc::new(MockedInventoryRepository{}));
        let runtime = Runtime::new().unwrap();
        let mut character = create_character();
        let task_queue = Arc::new(TasksQueue::new());
        let map_instance = create_empty_map_instance(context.test_context.client_notification_sender(), task_queue.clone());
        let index = add_items_in_inventory(&mut character, "Jellopy", 10);
        let mut jellopy_item_model = character.get_item_from_inventory(index).unwrap().clone();
        let index = add_items_in_inventory(&mut character, "Knife", 1);
        let mut knife_item_model = character.get_item_from_inventory(index).unwrap().clone();
        let char_id = character.char_id;
        // When
        context.inventory_service.character_drop_items(&runtime, &mut character, CharacterRemoveItems {
            char_id,
            sell: false,
            items: vec![CharacterRemoveItem { char_id, index: 0, amount: 7, price: 0 }, CharacterRemoveItem { char_id, index: 1, amount: 1, price: 0 }],
        }, &map_instance);
        // Then
        assert_task_queue_is_empty!(task_queue);
    }
}