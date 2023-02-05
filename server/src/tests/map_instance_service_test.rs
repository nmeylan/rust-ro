use std::sync::Arc;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::map_instance_service::MapInstanceService;
use crate::server::service::mob_service::MobService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};

struct MapInstanceServiceTestContext {
    test_context: TestContext,
    map_instance_service: MapInstanceService,
    server_task_queue: Arc<TasksQueue<GameEvent>>
}

fn before_each() -> MapInstanceServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let mob_service = MobService::new(client_notification_sender.clone(), GlobalConfigService::instance());
    let server_task_queue = Arc::new(TasksQueue::new());
    MapInstanceServiceTestContext {
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender: persistence_event_sender.clone(), client_notification_receiver, persistence_event_receiver },
        server_task_queue: server_task_queue.clone(),
        map_instance_service: MapInstanceService::new(client_notification_sender,  GlobalConfigService::instance(), mob_service, server_task_queue),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::mem;
    use tokio::runtime::Runtime;
    use crate::assert_eq_with_variance;
    use crate::server::model::events::game_event::{CharacterKillMonster, GameEvent};
    use crate::server::model::events::map_event::MobDropItems;
    use crate::server::model::item::DroppedItem;
    use crate::server::model::map_item::{MapItem, MapItemType};
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::state::mob::Mob;
    use crate::tests::common::assert_helper::task_queue_contains_event_at_tick;
    use crate::tests::common::map_instance_helper::create_empty_map_instance_state;
    use crate::tests::common::mob_helper::create_mob;
    use crate::tests::map_instance_service_test::before_each;

    #[test]
    fn test_mob_die_should_remove_map_item_from_map_instance_and_mob() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_die(&mut map_instance_state, mob_item_id, 0);
        // Then
        assert_eq!(mem::discriminant(&map_instance_state.get_mob(mob_item_id)), mem::discriminant(&None));
        assert_eq!(mem::discriminant(&map_instance_state.get_map_item(mob_item_id)), mem::discriminant(&None));
    }
    #[test]
    fn test_mob_die_should_emit_an_event_with_attacker_with_higher_damage_and_mob_id() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let mut mob = create_mob(mob_item_id, "PORING");
        let mob_id = mob.mob_id;
        let x = mob.x;
        let y = mob.y;
        mob.add_attack(150000, 20);
        mob.add_attack(150001, 40);
        mob.add_attack(150000, 30);
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_die(&mut map_instance_state, mob_item_id, 0);
        // Then
        task_queue_contains_event_at_tick(context.server_task_queue.clone(), GameEvent::CharacterKillMonster(CharacterKillMonster{ char_id: 150000, mob_id, mob_x: x, mob_y: y, map_instance_key: map_instance_state.key().clone() }), 0);
    }
    #[test]
    fn test_mob_drop_item_when_mob_are_normal() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let poring = GlobalConfigService::instance().get_mob_by_name("PORING");
        let mob_drop_items = MobDropItems { owner_id: 150000, mob_id: poring.id as i16, mob_x: 10, mob_y: 10 };
        let mut expected_dropped_item_amount: HashMap<u32, (String,u32)> = Default::default();
        poring.drops.iter().for_each(|drop_rate| {
            // Sometime mob like "PORING" drop an item twice (like "apple") with different drop rates
            let entry = expected_dropped_item_amount.entry(GlobalConfigService::instance().get_item_id_from_name(drop_rate.item_name.as_str())).or_insert( (drop_rate.item_name.clone(), 0));
            entry.1 += drop_rate.rate as u32;
        });

        let iterations = 1100;
        // When
        let mut drops_per_item: HashMap<u32, u32> = HashMap::new();
        for _ in 0..=iterations {
            for _ in 0..10000 {
                let drops = context.map_instance_service.mob_drop_items(&mut map_instance_state, mob_drop_items);
                for drop in drops {
                    let item_drop_count = drops_per_item.entry(drop.item_id as u32).or_insert(0);
                    *item_drop_count += 1;
                }
            }
        }
        // Then
        let mut average_drops_per_item: HashMap<u32, u32> = HashMap::new();
        for (item_id, (item_name, expected_drop_amount)) in expected_dropped_item_amount {
            let total_amount_dropped = drops_per_item.get(&item_id).unwrap();
            let average = (*total_amount_dropped as f32 / iterations as f32).round() as u32;
            assert_eq_with_variance!(2, average, expected_drop_amount, "Expected item {} to be dropped {} times but was dropped {} times", item_name, expected_drop_amount, average);
            println!("Dropped: {} {} times", item_name, average);
        }
    }
    #[test]
    fn test_mob_drop_items_should_add_item_to_map_items() {
        // Given
        let context = before_each();
        let poring = GlobalConfigService::instance().get_mob_by_name("PORING");
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_drop_items = MobDropItems { owner_id: 150000, mob_id: poring.id as i16, mob_x: 10, mob_y: 10 };
        // When
        let drops = context.map_instance_service.mob_drop_items(&mut map_instance_state, mob_drop_items);
        // Then
        for drop in drops {
            assert!(map_instance_state.get_map_item(drop.map_item_id).is_some());
            assert!(matches!(map_instance_state.get_map_item(drop.map_item_id).unwrap().object_type(), MapItemType::DroppedItem));
            assert!(map_instance_state.get_dropped_item(drop.map_item_id).is_some());
        }
    }

    #[test]
    fn test_mob_drop_item_when_mob_is_mvp() {
        // Given
        let context = before_each();

        // When

        // Then
    }
}