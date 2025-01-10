#![allow(dead_code)]
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::script::skill::ScriptSkillService;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::inventory_service::InventoryService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::item_service::ItemService;
use crate::server::service::script_service::ScriptService;
use crate::server::service::server_service::ServerService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::server::Server;
use crate::tests::common;
use crate::tests::common::mocked_repository::MockedRepository;
use crate::tests::common::sync_helper::CountDownLatch;
use crate::tests::common::{create_mpsc, test_script_vm, ServerBuilder, TestContext};
use std::sync::mpsc::SyncSender;
use std::sync::Arc;
use tokio::runtime::Runtime;

struct ServerServiceTestContext {
    test_context: TestContext,
    client_notification_sender: SyncSender<Notification>,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
    movement_task_queue: Arc<TasksQueue<GameEvent>>,
    status_service: &'static StatusService,
    server: Server,
    runtime: Arc<Runtime>,
}

impl ServerServiceTestContext {
    pub fn runtime(&self) -> &Runtime {
        self.runtime.as_ref()
    }
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
    StatusService::init(GlobalConfigService::instance(), test_script_vm());
    let repository = Arc::new(MockedRepository);
    let server_service = ServerService::new(client_notification_sender.clone(), GlobalConfigService::instance(), server_task_queue.clone(), movement_task_queue.clone(), test_script_vm(),
                                            InventoryService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance(), server_task_queue.clone()),
                                            BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal),
                                            SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal), StatusService::instance(), GlobalConfigService::instance()).force_no_delay(),
                                            StatusService::instance(),
                                            ScriptService::new(client_notification_sender.clone(), GlobalConfigService::instance(), repository.clone(), server_task_queue.clone(), test_script_vm()),
                                            CharacterService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance(),
                                                                  SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()), StatusService::instance(), server_task_queue.clone()),
                                            SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
                                            ItemService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), test_script_vm(), GlobalConfigService::instance()),
                                            ScriptSkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), repository.clone(), GlobalConfigService::instance())
    );

    let runtime = Arc::new(Runtime::new().unwrap());
    let server = ServerBuilder::new(GlobalConfigService::instance().config(), server_service, runtime.clone()).tasks_queue(server_task_queue.clone()).build();
    ServerServiceTestContext {
        client_notification_sender: client_notification_sender.clone(),
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        server_task_queue: server_task_queue.clone(),
        movement_task_queue: movement_task_queue.clone(),
        status_service: StatusService::instance(),
        runtime,
        server
    }
}


#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use crate::server::model::events::game_event::CharacterUseSkill;
    use crate::server::model::events::map_event::MapEvent;
    use crate::server::model::map_item::ToMapItem;
    use crate::server::model::tasks_queue::TasksQueue;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::Server;
    use crate::tests::common::assert_helper::assert_vecs_equal;
    use crate::tests::common::assert_helper::{has_sent_notification, task_queue_contains_event_at_tick, NotificationExpectation, SentPacket};
    use crate::tests::common::character_helper::create_character;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;
    use crate::tests::common::server_helper::create_empty_server_state;
    use crate::tests::server_service_test::before_each;
    use crate::util::tick::get_tick;
    use crate::{assert_sent_packet_in_current_packetver, assert_vec_equals, status_snapshot};
    use models::enums::bonus::BonusType;
    use models::enums::skill_enums::SkillEnum;
    use models::item::DroppedItem;
    use models::position::Position;
    use models::status::KnownSkill;
    use models::status_bonus::{StatusBonus, StatusBonuses};
    use packets::packets::{PacketZcMsgStateChange, PacketZcMsgStateChange2};
    use std::mem;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_when_item_in_fov() {
        // Given
        let context = before_each();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, 501);
        assert_eq!(item_from_inventory.amount, 2);
    }

    #[test]
    fn character_pickup_item_should_add_item_to_character_inventory_and_keep_is_identified_status_from_item_drop() {
        // Given
        let context = before_each();
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
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, red_potion_map_item_id, &map_instance, context.runtime());
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, clover_map_item_id, &map_instance, context.runtime());
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, knife_map_item_id, &map_instance, context.runtime());
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
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        map_instance.state_mut().insert_dropped_item(DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true });
        // When
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
    }


    #[test]
    fn character_pickup_item_should_prevent_pickup_when_item_is_still_locked_by_another_player() {
        // Given
        let context = before_each();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: Some(15001), dropped_at: get_tick() - 10, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_none());
    }

    #[test]
    fn character_pickup_item_should_pickup_when_item_is_no_longer_locked_by_another_player() {
        // Given
        let context = before_each();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: Some(15001), dropped_at: get_tick() - 10 - (GlobalConfigService::instance().config().game.mob_dropped_item_locked_to_owner_duration_in_secs as u128 * 1000), amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_some());
    }

    #[test]
    fn character_pickup_item_should_remove_map_item_from_map_instance() {
        // Given
        let context = before_each();
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
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0);
        assert!(item_from_inventory.is_some());
        task_queue_contains_event_at_tick::<MapEvent>(task_queue, MapEvent::RemoveDroppedItemFromMap(map_item_id), 0);
    }

    #[test]
    fn character_pickup_item_should_be_at_most_called_once() {
        // Given
        let context = before_each();
        let mut server_state = create_empty_server_state();
        let mut character_state = create_character();
        let map_instance = create_empty_map_instance(context.client_notification_sender.clone(), Arc::new(TasksQueue::new()));
        let map_item_id = 1000;
        let item = DroppedItem { map_item_id, item_id: 501, location: Position { x: 50, y: 50, dir: 0 }, sub_location: Position { x: 3, y: 3, dir: 0 }, owner_id: None, dropped_at: 0, amount: 2, is_identified: true };
        // Add dropped item in character fov
        character_state.map_view.insert(item.to_map_item());
        map_instance.state_mut().insert_dropped_item(item);
        // When
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        context.server.server_service().character_pickup_item(&mut server_state, &mut character_state, map_item_id, &map_instance, context.runtime());
        // Then
        let item_from_inventory = character_state.get_item_from_inventory(0).unwrap();
        assert_eq!(item_from_inventory.item_id, 501);
        assert_eq!(item_from_inventory.amount, 2);
    }

    #[test]
    fn character_use_support_skill_should_apply_bonuses_and_send_add_bonuses_packet() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let char_id = character.char_id;
        context.server.state_mut().insert_character(character);
        #[derive(Clone)]
        struct TestResult {
            skill: KnownSkill,
            expected_bonuses: StatusBonuses
        }
        let scenario = vec![
            TestResult { skill: KnownSkill { value: SkillEnum::AlBlessing, level: 10 }, expected_bonuses: StatusBonuses::new(vec![StatusBonus::new(BonusType::Dex(10)), StatusBonus::new(BonusType::Str(10)), StatusBonus::new(BonusType::Int(10))]) },
            TestResult { skill: KnownSkill { value: SkillEnum::AlIncagi, level: 10 }, expected_bonuses: StatusBonuses::new(vec![StatusBonus::new(BonusType::Agi(12)), StatusBonus::new(BonusType::SpeedPercentage(25))]) },
        ];
        // When

        let mut server_state_mut = context.server.state_mut();
        let mut tick = 0;
        for scenarii in scenario {
            context.test_context.reset_increment_latch();
            context.test_context.clear_sent_packet();
            let character = server_state_mut.characters_mut().get_mut(&char_id).unwrap();
            character.status.hp = 1000;
            character.status.sp = 1000;
            let source_status = status_snapshot!(context, character);
            let target_status = status_snapshot!(context, character);
            context.server.server_service().character_start_use_skill(context.server.state(), character, CharacterUseSkill {
                char_id,
                target_id: char_id,
                skill_id: scenarii.skill.value.id(),
                skill_level: scenarii.skill.level,
            }, tick);
            tick += 100;
            Server::game_loop_iteration(&context.server, tick);
            context.test_context.increment_latch().wait_expected_count_with_timeout(4, Duration::from_millis(400));
            assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcMsgStateChange2::packet_id(GlobalConfigService::instance().packetver()), 1)]));
        }
        // Then
        let character = mem::take(&mut server_state_mut.characters_mut().get_mut(&char_id)).unwrap();
        assert!(!character.status.temporary_bonuses.is_empty());
        assert_vec_equals!(character.status.temporary_bonuses.to_vec(), vec![StatusBonus::new(BonusType::Dex(110)), StatusBonus::new(BonusType::Str(10)), StatusBonus::new(BonusType::Int(10)), StatusBonus::new(BonusType::Agi(12)), StatusBonus::new(BonusType::SpeedPercentage(25))]);

        // Then after skills duration, temporary bonuses have expired, a packet is sent to client
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        Server::game_loop_iteration(&context.server, tick + 240 * 1000); // 240s is duration of inc agi and blessing

        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcMsgStateChange::packet_id(GlobalConfigService::instance().packetver()), 1)]));
    }

}
