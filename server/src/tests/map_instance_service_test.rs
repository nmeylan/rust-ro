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
use crate::tests::common::sync_helper::CountDownLatch;

struct MapInstanceServiceTestContext {
    test_context: TestContext,
    map_instance_service: MapInstanceService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

fn before_each() -> MapInstanceServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> MapInstanceServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let mob_service = MobService::new(client_notification_sender.clone(), GlobalConfigService::instance());
    let server_task_queue = Arc::new(TasksQueue::new());
    let count_down_latch = CountDownLatch::new(latch_size);
    MapInstanceServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        server_task_queue: server_task_queue.clone(),
        map_instance_service: MapInstanceService::new(client_notification_sender, GlobalConfigService::instance(), mob_service, server_task_queue),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::mem;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::runtime::Runtime;
    use packets::packets::PacketZcItemDisappear;
    use crate::{assert_eq_with_variance, assert_sent_packet_in_current_packetver, assert_task_queue_contains_event_at_tick};
    use crate::server::model::action::Damage;
    use crate::server::model::events::game_event::{CharacterKillMonster, GameEvent};
    use crate::server::model::events::map_event::{MapEvent, MobDropItems, MobLocation};
    use crate::server::model::item::DroppedItem;
    use crate::server::model::map_item::{MapItem, MapItemType};
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::state::mob::Mob;
    use crate::server::map_instance_loop::MAP_LOOP_TICK_RATE;
    use crate::server::model::position::Position;
    use crate::tests::common::assert_helper::{NotificationExpectation, SentPacket, task_queue_contains_event_at_tick, has_sent_notification};
    use crate::tests::common::map_instance_helper::create_empty_map_instance_state;
    use crate::tests::common::mob_helper::create_mob;
    use crate::tests::map_instance_service_test::before_each;
    use crate::util::tick::get_tick;
    use crate::util::tick::delayed_tick;

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
        task_queue_contains_event_at_tick(context.server_task_queue.clone(), GameEvent::CharacterKillMonster(CharacterKillMonster { char_id: 150000, mob_id, mob_x: x, mob_y: y, map_instance_key: map_instance_state.key().clone() }), 0);
    }

    #[test]
    fn test_mob_drop_item_when_mob_are_normal() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let poring = GlobalConfigService::instance().get_mob_by_name("PORING");
        let mob_drop_items = MobDropItems { owner_id: 150000, mob_id: poring.id as i16, mob_x: 10, mob_y: 10 };
        let mut expected_dropped_item_amount: HashMap<u32, (String, u32)> = Default::default();
        poring.drops.iter().for_each(|drop_rate| {
            // Sometime mob like "PORING" drop an item twice (like "apple") with different drop rates
            let entry = expected_dropped_item_amount.entry(GlobalConfigService::instance().get_item_id_from_name(drop_rate.item_name.as_str())).or_insert((drop_rate.item_name.clone(), 0));
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
    fn test_mob_being_attacked_should_damage_mob() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let map_instance_tasks_queue = Arc::new(<TasksQueue<MapEvent>>::new());
        let mob = create_mob(mob_item_id, "PORING");
        let max_hp = mob.status.max_hp;
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_being_attacked(&mut map_instance_state, Damage { target_id: mob_item_id, attacker_id: 150000, damage: 10, attacked_at: get_tick() }, map_instance_tasks_queue.clone(), get_tick());
        // Then
        assert_eq!(map_instance_state.get_mob(mob_item_id).unwrap().hp(), max_hp - 10);
    }

    #[test]
    fn test_mob_being_attacked_should_trigger_mob_die_when_mob_has_get_more_damage_than_its_hp() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let map_instance_tasks_queue = Arc::new(<TasksQueue<MapEvent>>::new());
        let mob = create_mob(mob_item_id, "PORING");
        let max_hp = mob.status.max_hp;
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_being_attacked(&mut map_instance_state, Damage { target_id: mob_item_id, attacker_id: 150000, damage: max_hp + 10, attacked_at: get_tick() }, map_instance_tasks_queue.clone(), get_tick());
        // Then
        assert!(map_instance_state.get_mob(mob_item_id).is_none());
    }

    #[test]
    fn test_mob_being_attacked_should_delay_mob_vanish_client_side_notification() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let mob_item_id = 82322;
        let map_instance_tasks_queue = Arc::new(<TasksQueue<MapEvent>>::new());
        let mob = create_mob(mob_item_id, "PORING");
        let original_mob = mob.clone();
        let max_hp = mob.status.max_hp;
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_being_attacked(&mut map_instance_state, Damage { target_id: mob_item_id, attacker_id: 150000, damage: max_hp + 10, attacked_at: get_tick() + 10 }, map_instance_tasks_queue.clone(), get_tick());
        // Then
        assert_task_queue_contains_event_at_tick!(map_instance_tasks_queue.clone(), MapEvent::MobDeathClientNotification(MobLocation { mob_id: original_mob.id, x: original_mob.x, y: original_mob.y }), delayed_tick(10, MAP_LOOP_TICK_RATE));
    }

    #[test]
    fn test_remove_dropped_item_from_map_should_trigger_server_map_item_remove() {
        // Given
        let context = before_each();
        let mut map_instance_state = create_empty_map_instance_state();
        let clover = DroppedItem { map_item_id: 1000, item_id: GlobalConfigService::instance().get_item_id_from_name("Clover") as i32, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, };
        map_instance_state.insert_dropped_item(clover);
        // When
        context.map_instance_service.remove_dropped_item_from_map(&mut map_instance_state, clover.map_item_id);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(50, 50, vec![SentPacket::with_id(PacketZcItemDisappear::packet_id())]));
        assert_task_queue_contains_event_at_tick!(context.server_task_queue.clone(), GameEvent::MapNotifyItemRemoved(clover.map_item_id), 0);
    }
}