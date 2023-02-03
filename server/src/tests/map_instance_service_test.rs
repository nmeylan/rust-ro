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
    use std::mem;
    use crate::server::model::events::game_event::{CharacterKillMonster, GameEvent};
    use crate::server::model::map_item::{MapItem, MapItemType};
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
        let mob = create_mob(mob_item_id, "Poring");
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob.mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_die(&mut map_instance_state, mob_item_id);
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
        let mut mob = create_mob(mob_item_id, "Poring");
        let mob_id = mob.mob_id;
        mob.add_attack(150000, 20);
        mob.add_attack(150001, 40);
        mob.add_attack(150000, 30);
        map_instance_state.insert_item(MapItem::new(mob_item_id, mob_id, MapItemType::Mob));
        map_instance_state.mobs_mut().insert(mob_item_id, mob);
        // When
        context.map_instance_service.mob_die(&mut map_instance_state, mob_item_id);
        // Then
        task_queue_contains_event_at_tick(context.server_task_queue.clone(), GameEvent::CharacterKillMonster(CharacterKillMonster{ char_id: 150000, mob_id, map_instance_key: map_instance_state.key().clone() }), 0);
    }
}