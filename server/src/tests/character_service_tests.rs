#![allow(dead_code)]

use std::sync::Arc;
use crate::repository::CharacterRepository;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::model::tasks_queue::TasksQueue;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::character::skill_tree_service::SkillTreeService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, test_script_vm, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct CharacterServiceTestContext {
    test_context: TestContext,
    character_service: CharacterService,
    server_task_queue: Arc<TasksQueue<GameEvent>>,
}

fn before_each(character_repository: Arc<dyn CharacterRepository + Sync>) -> CharacterServiceTestContext {
    before_each_with_latch(character_repository, 0)
}


fn before_each_with_latch(character_repository: Arc<dyn CharacterRepository + Sync>, latch_size: usize) -> CharacterServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    let server_task_queue = Arc::new(TasksQueue::new());
    StatusService::init(GlobalConfigService::instance(), test_script_vm());
    CharacterServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        character_service: CharacterService::new(client_notification_sender.clone(), persistence_event_sender, character_repository, GlobalConfigService::instance(),
                                                 SkillTreeService::new(client_notification_sender.clone(), GlobalConfigService::instance()),
                                                 StatusService::instance(),
                                                 server_task_queue.clone()),
        server_task_queue,
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::Relaxed;
    use std::time::Duration;
    use async_trait::async_trait;
    use sqlx::Error;
    use tokio::runtime::Runtime;
    use models::enums::class::JobName;
    use models::enums::look::LookType;
    use models::enums::status::StatusTypes;
    use packets::packets::{PacketZcSpriteChange2, PacketZcLongparChange, PacketZcParChange, PacketZcNotifyEffect, PacketZcStatusChangeAck, PacketZcSkillinfoList};
    use crate::tests::character_service_tests::GameEvent;
    use crate::{assert_sent_packet_in_current_packetver, assert_sent_persistence_event, assert_task_queue_contains_event, assert_task_queue_contains_event_at_tick};
    use crate::tests::common::assert_helper::{has_sent_persistence_event, has_sent_notification, NotificationExpectation, SentPacket, task_queue_contains_event_at_tick, task_queue_contains_event};
    use crate::tests::character_service_tests::before_each;
    use crate::tests::common::character_helper::{add_items_in_inventory, create_character};
    use crate::tests::common::mocked_repository;
    use models::enums::EnumWithStringValue;
    use models::enums::EnumWithNumberValue;
    use models::enums::skill_enums::SkillEnum;
    use crate::repository::CharacterRepository;
    use crate::server::model::events::map_event::MapEvent;
    use crate::server::model::events::game_event::{CharacterKillMonster, CharacterLook, CharacterUpdateStat, CharacterZeny};
    use crate::server::model::events::map_event::{MobDropItems};
    use crate::server::model::events::persistence_event::{IncreaseSkillLevel, PersistenceEvent, ResetSkills, SavePositionUpdate, StatusUpdate};
    use crate::server::model::map_instance::MapInstanceKey;
    use crate::server::model::movement::Movement;
    use models::position::Position;
    use models::status::KnownSkill;
    use crate::server::model::tasks_queue::TasksQueue;


    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::common::map_instance_helper::create_empty_map_instance;

    use crate::util::tick::get_tick;

    #[test]
    fn test_max_weight() {
        // Given
        let context = before_each(mocked_repository());
        struct WeightExpectation<'a> {
            job: &'a str,
            str: u16,
            expected_max_weight: u32,
        }
        // Note that client side display weight values / 10.
        let expectations = vec![
            WeightExpectation { job: "Novice", str: 1, expected_max_weight: 20300 },
            WeightExpectation { job: "Archer", str: 1, expected_max_weight: 26300 },
            WeightExpectation { job: "Blacksmith", str: 1, expected_max_weight: 30300 },
            WeightExpectation { job: "Swordsman", str: 1, expected_max_weight: 28300 },
            WeightExpectation { job: "Swordsman", str: 50, expected_max_weight: 43000 },
        ];
        for expectation in expectations.iter() {
            let mut character = create_character();
            character.status.str = expectation.str;
            character.status.job = JobName::from_string(expectation.job).value() as u32;
            // When
            let max_weight = context.character_service.max_weight(&character);
            // Then
            assert_eq!(max_weight, expectation.expected_max_weight, "Expected max weight to be {} but was {} for class {}", expectation.expected_max_weight, max_weight, expectation.job);
        }
    }

    #[test]
    fn test_can_carry_weight() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.str = 1;
        character.status.job = JobName::from_string("Novice").value() as u32;
        let phracon = GlobalConfigService::instance().get_item_by_name("Phracon");

        // When
        add_items_in_inventory(&mut character, "Phracon", 80); // Phracon weight is 200
        let can_carry = context.character_service.can_carry_weight(&character, phracon.weight as u32);
        // Then
        assert!(can_carry);

        // When
        add_items_in_inventory(&mut character, "Phracon", 11);
        let can_carry = context.character_service.can_carry_weight(&character, phracon.weight as u32);
        // Then
        assert!(!can_carry)
    }

    #[test]
    fn test_change_map_should_clear_movement() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 0);
        let mut character = create_character();
        character.movements = vec![Movement::new(100, 100, get_tick())];
        assert!(!character.movements.is_empty());
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        assert!(character.movements.is_empty());
    }

    #[test]
    fn test_change_map_should_update_position() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 0);
        let mut character = create_character();
        character.x = 99;
        character.y = 99;
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        assert_eq!(character.x(), 120);
        assert_eq!(character.y(), 120);
    }

    #[test]
    fn test_change_map_should_defer_position_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 0);
        let mut character = create_character();
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::SaveCharacterPosition(SavePositionUpdate { char_id: character.char_id, account_id: character.account_id, map_name: "geffen.gat".to_string(), x: 120, y: 120 }));
    }

    #[test]
    fn test_change_map_should_update_map() {
        // Given
        let context = before_each(mocked_repository());
        let map_instance_key = MapInstanceKey::new("geffen.gat".to_string(), 1);
        let mut character = create_character();
        character.x = 99;
        character.y = 99;
        // When
        context.character_service.change_map(&map_instance_key, Position { x: 120, y: 120, dir: 0 }, &mut character);
        // Then
        assert_eq!(character.map_instance_key.map_name(), &"geffen.gat".to_string());
        assert_eq!(character.map_instance_key.map_instance(), 1);
    }

    #[test]
    fn test_change_look_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let character_look = CharacterLook { char_id: character.char_id, look_type: LookType::Hair, look_value: 10 };
        // When
        context.character_service.change_look(character_look, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_eq!(10, character.get_look(LookType::Hair));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "hair".to_string(), value: 10, }));
    }

    #[test]
    fn test_change_look_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let character_look = CharacterLook { char_id: character.char_id, look_type: LookType::Hair, look_value: 10 };
        // When
        context.character_service.change_look(character_look, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_change_sprite_should_notify_area() {
        // Given
        let context = before_each(mocked_repository());
        let character = create_character();
        // When
        context.character_service.change_sprite(&character, LookType::Hair, 10, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_update_zeny_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_zeny = CharacterZeny { char_id: character.char_id, zeny: Some(100) };
        // When
        context.character_service.update_zeny(&runtime, character_zeny, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_eq!(character.status.zeny, 100);
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "zeny".to_string(), value: 100, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcLongparChange::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_update_zeny_should_fetch_zeny_when_zeny_amount_is_not_specified() {
        // Given
        struct MockedCharacterRepository {
            called_fetch_zeny: AtomicBool,
        }
        #[async_trait]
        impl CharacterRepository for MockedCharacterRepository {
            async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> {
                self.called_fetch_zeny.store(true, Relaxed);
                Ok(50)
            }
        }
        let mocked_character_repository = Arc::new(MockedCharacterRepository { called_fetch_zeny: AtomicBool::new(false) });
        let context = before_each(mocked_character_repository.clone());
        let mut character = create_character();
        let runtime = Runtime::new().unwrap();
        let character_zeny = CharacterZeny { char_id: character.char_id, zeny: None };
        // When
        context.character_service.update_zeny(&runtime, character_zeny, &mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_eq!(character.status.zeny, 50);
        assert!(mocked_character_repository.called_fetch_zeny.load(Relaxed));
    }

    #[test]
    fn test_update_base_level_should_be_bound_by_min_and_max_level() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_base_level(&mut character, Some(78), None);
        // Then
        assert_eq!(character.status.base_level, 78);
        // When
        context.character_service.update_base_level(&mut character, Some(788), None);
        // Then
        assert_eq!(character.status.base_level, 99);
        // When
        context.character_service.update_base_level(&mut character, None, Some(-6));
        // Then
        assert_eq!(character.status.base_level, 93);
        // When
        context.character_service.update_base_level(&mut character, None, Some(10));
        // Then
        assert_eq!(character.status.base_level, 99);
        // When
        context.character_service.update_base_level(&mut character, None, Some(-150));
        // Then
        assert_eq!(character.status.base_level, 1);
        // When
        context.character_service.update_base_level(&mut character, Some(66), Some(10));
        // Then
        assert_eq!(character.status.base_level, 66);
    }

    #[test]
    fn test_update_base_level_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_base_level(&mut character, Some(78), None);
        // Then
        assert_eq!(character.status.base_level, 78);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(5, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "base_level".to_string(), value: 78, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "status_point".to_string(), value: 849, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()))]));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcNotifyEffect::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_update_base_level_should_return_delta() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        let delta = context.character_service.update_base_level(&mut character, Some(78), None);
        // Then
        assert_eq!(delta, 77);
    }


    #[test]
    fn test_update_base_level_should_update_status_point_when_leveling_up_or_down() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            source_level: u16,
            target_level: u16,
            source_not_allocated_point: u32,
            target_not_allocated_point: u32,
        }

        let scenario = vec![
            Scenarii { source_level: 1, target_level: 10, source_not_allocated_point: 0, target_not_allocated_point: 32 },
            Scenarii { source_level: 10, target_level: 63, source_not_allocated_point: 0, target_not_allocated_point: 520 },
            Scenarii { source_level: 63, target_level: 74, source_not_allocated_point: 6, target_not_allocated_point: 184 },
            Scenarii { source_level: 74, target_level: 72, source_not_allocated_point: 184, target_not_allocated_point: 150 },
            Scenarii { source_level: 74, target_level: 60, source_not_allocated_point: 184, target_not_allocated_point: 0 },
            Scenarii { source_level: 74, target_level: 87, source_not_allocated_point: 184, target_not_allocated_point: 426 },
            Scenarii { source_level: 87, target_level: 92, source_not_allocated_point: 426, target_not_allocated_point: 528 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.base_level = scenarii.source_level as u32;
            character.status.status_point = scenarii.source_not_allocated_point;
            // When
            context.character_service.update_base_level(&mut character, Some(scenarii.target_level as u32), None);
            let status_point_count = character.status.status_point;
            // Then
            assert_eq!(status_point_count, scenarii.target_not_allocated_point, "Expected character at level {} when leveling up/down to {} to have {} status point but got {}", scenarii.source_level, scenarii.target_level, scenarii.target_not_allocated_point, status_point_count);
        }
    }

    #[test]
    fn test_update_base_level_should_reset_stats_when_leveling_down_and_has_allocated_more_status_point_than_supported_by_the_level() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            source_level: u16,
            target_level: u16,
            job: &'a str,
            source_str: u16,
            source_agi: u16,
            source_dex: u16,
            source_int: u16,
            source_luk: u16,
            source_vit: u16,
            target_str: u16,
            target_agi: u16,
            target_dex: u16,
            target_int: u16,
            target_luk: u16,
            target_vit: u16,
            source_allocated_status_point: u32,
            target_allocated_status_point: u32,
            source_available_status_point: u32,
            target_available_status_point: u32,
        }

        let scenario = vec![
            Scenarii { source_level: 63, target_level: 60, job: "Thief", source_str: 31, source_agi: 77, source_dex: 33, source_int: 1, source_luk: 1, source_vit: 1, target_str: 1, target_agi: 1, target_dex: 1, target_int: 1, target_luk: 1, target_vit: 1, source_allocated_status_point: 594, target_allocated_status_point: 0, source_available_status_point: 6, target_available_status_point: 555 },
            Scenarii { source_level: 92, target_level: 82, job: "Clown", source_str: 20, source_agi: 3, source_dex: 91, source_int: 4, source_luk: 1, source_vit: 1, target_str: 20, target_agi: 3, target_dex: 91, target_int: 4, target_luk: 1, target_vit: 1, source_allocated_status_point: 597, target_allocated_status_point: 597, source_available_status_point: 577, target_available_status_point: 378 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            character.status.base_level = scenarii.source_level as u32;
            character.status.str = scenarii.source_str;
            character.status.agi = scenarii.source_agi;
            character.status.dex = scenarii.source_dex;
            character.status.int = scenarii.source_int;
            character.status.luk = scenarii.source_luk;
            character.status.vit = scenarii.source_vit;
            character.status.status_point = scenarii.source_available_status_point;
            assert_eq!(context.character_service.get_spent_status_point(&character), scenarii.source_allocated_status_point);
            // When
            context.character_service.update_base_level(&mut character, Some(scenarii.target_level as u32), None);
            let status_point_count = character.status.status_point;
            // Then
            assert_eq!(context.character_service.get_spent_status_point(&character), scenarii.target_allocated_status_point, "Expected character of class {} at level {} when down leveling to {} to have {} allocated stats but got {}", scenarii.job, scenarii.source_level, scenarii.target_level, scenarii.target_allocated_status_point, context.character_service.get_spent_status_point(&character));
            assert_eq!(status_point_count, scenarii.target_available_status_point, "Expected character of class {} at level {} when down leveling to {} to have {} available status point but got {}", scenarii.job, scenarii.source_level, scenarii.target_level, scenarii.target_available_status_point, status_point_count);
            if scenarii.target_allocated_status_point == 0 {
                assert_eq!(character.status.str, 1);
                assert_eq!(character.status.dex, 1);
                assert_eq!(character.status.agi, 1);
                assert_eq!(character.status.vit, 1);
                assert_eq!(character.status.int, 1);
                assert_eq!(character.status.luk, 1);
            }
        }
    }

    #[test]
    fn test_update_job_level_should_be_bound_by_min_and_max_level() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            job: usize,
            job_level: u32,
            maybe_new_job_level: Option<u32>,
            maybe_job_level_delta: Option<i32>,
            expected: u32,
        }
        let scenario = vec![
            Scenarii { job: JobName::Paladin.value(), job_level: 1, maybe_new_job_level: Some(68), maybe_job_level_delta: None, expected: 68 },
            Scenarii { job: JobName::Paladin.value(), job_level: 68, maybe_new_job_level: Some(788), maybe_job_level_delta: None, expected: 70 },
            Scenarii { job: JobName::Paladin.value(), job_level: 70, maybe_new_job_level: None, maybe_job_level_delta: Some(-6), expected: 64 },
            Scenarii { job: JobName::Paladin.value(), job_level: 64, maybe_new_job_level: None, maybe_job_level_delta: Some(5), expected: 69 },
            Scenarii { job: JobName::Paladin.value(), job_level: 69, maybe_new_job_level: None, maybe_job_level_delta: Some(-150), expected: 1 },
            Scenarii { job: JobName::Paladin.value(), job_level: 1, maybe_new_job_level: Some(66), maybe_job_level_delta: Some(10), expected: 66 },
            Scenarii { job: JobName::Novice.value(), job_level: 1, maybe_new_job_level: Some(15), maybe_job_level_delta: None, expected: 10 },
            Scenarii { job: JobName::Archer.value(), job_level: 1, maybe_new_job_level: Some(60), maybe_job_level_delta: None, expected: 50 },
            Scenarii { job: JobName::Bard.value(), job_level: 1, maybe_new_job_level: Some(60), maybe_job_level_delta: None, expected: 50 },
            Scenarii { job: JobName::NoviceHigh.value(), job_level: 1, maybe_new_job_level: Some(60), maybe_job_level_delta: None, expected: 10 },
            Scenarii { job: JobName::ArcherHigh.value(), job_level: 1, maybe_new_job_level: Some(60), maybe_job_level_delta: None, expected: 50 },
            Scenarii { job: JobName::Clown.value(), job_level: 1, maybe_new_job_level: Some(80), maybe_job_level_delta: None, expected: 70 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job_level = scenarii.job_level;
            character.status.job = scenarii.job as u32;
            // When
            context.character_service.update_job_level(&mut character, scenarii.maybe_new_job_level, scenarii.maybe_job_level_delta);
            // Then
            assert_eq!(character.status.job_level, scenarii.expected, "Expected character of class {} to be level {} but was {}", scenarii.job, scenarii.expected, character.status.job_level);
        }
    }

    #[test]
    fn test_update_job_level_should_defer_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job = JobName::Paladin.value() as u32;
        // When
        context.character_service.update_job_level(&mut character, Some(68), None);
        // Then
        assert_eq!(character.status.job_level, 68);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(5, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "job_level".to_string(), value: 68, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "skill_point".to_string(), value: 67, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()))]));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcNotifyEffect::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_allocated_skills_point_should_exclude_platinium_skills() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            expected_allocated_points: u8,
            skills: Vec<KnownSkill>,
        }
        let scenario = vec![
            Scenarii {
                expected_allocated_points: 25,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_BASH"), level: 6 }, ],
            },
            Scenarii {
                expected_allocated_points: 85,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_FATALBLOW"), level: 1 },
                             KnownSkill { value: SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_RECOVERY"), level: 6 },
                             KnownSkill { value: SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_ENDURE"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                             KnownSkill { value: SkillEnum::from_name("KN_PIERCE"), level: 3 },
                             KnownSkill { value: SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                             KnownSkill { value: SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("KN_RIDING"), level: 1 },
                             KnownSkill { value: SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 },
                ],
            },
        ];

        for scenarii in scenario {
            // Given
            let mut character = create_character();
            character.status.known_skills = scenarii.skills;
            // When
            let actual = context.character_service.get_allocated_skills_point(&character);
            // Then
            assert_eq!(actual, scenarii.expected_allocated_points);
        }
    }

    #[test]
    fn test_update_job_level_should_update_skill_point_when_leveling_up_or_down() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            source_level: u32,
            job: usize,
            target_level: u32,
            current_skill_point: u32,
            expected_skill_point: u32,
            skills: Vec<KnownSkill>,
        }
        let scenario = vec![
            Scenarii { source_level: 1, job: JobName::Novice.value(), target_level: 2, current_skill_point: 0, expected_skill_point: 1, skills: vec![] },
            Scenarii { source_level: 1, job: JobName::Novice.value(), target_level: 10, current_skill_point: 0, expected_skill_point: 9, skills: vec![] },
            Scenarii { source_level: 1, job: JobName::Novice.value(), target_level: 4, current_skill_point: 0, expected_skill_point: 3, skills: vec![] },
            Scenarii { source_level: 4, job: JobName::Novice.value(), target_level: 10, current_skill_point: 3, expected_skill_point: 9, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 3 }] },
            Scenarii { source_level: 40, job: JobName::Knight.value(), target_level: 41, current_skill_point: 80, expected_skill_point: 81, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 }] },
            Scenarii { source_level: 10, job: JobName::Novice.value(), target_level: 4, current_skill_point: 0, expected_skill_point: 3, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 }] },
            Scenarii { source_level: 10, job: JobName::Novice.value(), target_level: 4, current_skill_point: 2, expected_skill_point: 3, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 }] },
            Scenarii { source_level: 10, job: JobName::Novice.value(), target_level: 1, current_skill_point: 10, expected_skill_point: 0, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 }] },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = scenarii.job as u32;
            character.status.job_level = scenarii.source_level;
            character.status.skill_point = scenarii.current_skill_point;
            character.status.known_skills = scenarii.skills;
            // When
            context.character_service.update_job_level(&mut character, Some(scenarii.target_level), None);
            // Then
            assert_eq!(character.status.skill_point, scenarii.expected_skill_point, "Expected character after job level change from {} to {}, to have {} skill points but got {}", scenarii.source_level, scenarii.target_level, scenarii.expected_skill_point, character.status.skill_point);
        }
    }

    #[test]
    fn test_get_skill_point_count_for_level() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            job: usize,
            job_level: u32,
            expected_skill_point_count: u8,
        }
        let scenario = vec![
            Scenarii { job: JobName::Novice.value(), job_level: 5, expected_skill_point_count: 4 },
            Scenarii { job: JobName::Novice.value(), job_level: 10, expected_skill_point_count: 9 },
            Scenarii { job: JobName::Archer.value(), job_level: 41, expected_skill_point_count: 9 + 40 },
            Scenarii { job: JobName::Archer.value(), job_level: 50, expected_skill_point_count: 9 + 49 },
            Scenarii { job: JobName::Hunter.value(), job_level: 50, expected_skill_point_count: 9 + 49 + 49 },
            Scenarii { job: JobName::Hunter.value(), job_level: 41, expected_skill_point_count: 9 + 49 + 40 },
            Scenarii { job: JobName::NoviceHigh.value(), job_level: 10, expected_skill_point_count: 9 },
            Scenarii { job: JobName::ArcherHigh.value(), job_level: 41, expected_skill_point_count: 9 + 40 },
            Scenarii { job: JobName::Sniper.value(), job_level: 70, expected_skill_point_count: 9 + 49 + 69 },
            Scenarii { job: JobName::Sniper.value(), job_level: 55, expected_skill_point_count: 9 + 49 + 54 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = scenarii.job as u32;
            character.status.job_level = scenarii.job_level;
            // When
            let skill_points = context.character_service.get_skill_point_count_for_level(&character);
            // Then
            assert_eq!(skill_points, scenarii.expected_skill_point_count, "Expected job {} at level {} to have {} skill points but got {}", JobName::from_value(scenarii.job).as_str(), scenarii.job_level, scenarii.expected_skill_point_count, skill_points);
        }
    }

    #[test]
    fn test_update_job_level_should_return_delta() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job = JobName::Paladin.value() as u32;
        // When
        let delta = context.character_service.update_job_level(&mut character, Some(68), None);
        // Then
        assert_eq!(delta, 67);
    }

    #[test]
    fn test_change_job_should_update_in_memory() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_job(&mut character, JobName::Assassin, false);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_eq!(character.status.job, JobName::Assassin.value() as u32);
    }

    #[test]
    fn test_change_job_should_reset_skills_and_reset_skill_level_when_should_reset_is_true() {
        // Given
        let mut context = before_each(mocked_repository());
        struct Scenarii {
            from_job: usize,
            to_job: usize,
            job_level: u32,
            expected_skill_point_count: u32,
            from_skill_point: u32,
        }
        let scenario = vec![
            Scenarii { from_job: JobName::Novice.value(), to_job: JobName::Archer.value(), job_level: 5, from_skill_point: 5, expected_skill_point_count: 9 + 4 },
            Scenarii { from_job: JobName::Novice.value(), to_job: JobName::Clown.value(), job_level: 5, from_skill_point: 5, expected_skill_point_count: 9 + 49 + 4 },
            Scenarii { from_job: JobName::Archer.value(), to_job: JobName::Clown.value(), job_level: 5, from_skill_point: 9 + 4, expected_skill_point_count: 9 + 49 + 4 },
            Scenarii { from_job: JobName::Clown.value(), to_job: JobName::Novice.value(), job_level: 5, from_skill_point: 9 + 49 + 4 , expected_skill_point_count: 4 },
            Scenarii { from_job: JobName::Clown.value(), to_job: JobName::Archer.value(), job_level: 5, from_skill_point: 9 + 49 + 4, expected_skill_point_count: 9 + 4 },
            Scenarii { from_job: JobName::Clown.value(), to_job: JobName::LordKnight.value(), job_level: 5, from_skill_point: 9 + 49 + 4, expected_skill_point_count: 9 + 49 + 4 },
            Scenarii { from_job: JobName::Clown.value(), to_job: JobName::Archer.value(), job_level: 40, from_skill_point: 9 + 49 + 39, expected_skill_point_count: 9 + 39 },
        ];
        // When
        for scenarii in scenario {
            context.test_context.reset_increment_latch();
            let mut character = create_character();
            character.status.job = scenarii.from_job as u32;
            character.status.job_level = scenarii.job_level;
            character.status.skill_point = scenarii.from_skill_point;
            // When
            context.character_service.change_job(&mut character, JobName::from_value(scenarii.to_job), true);
            // Then
            assert_eq!(character.status.skill_point, scenarii.expected_skill_point_count, "Expected from job {} at level {} to job {} to have {} skill points but got {}", JobName::from_value(scenarii.from_job).as_str(), JobName::from_value(scenarii.to_job).as_str(), scenarii.job_level, scenarii.expected_skill_point_count, character.status.skill_point);
            context.test_context.increment_latch().wait_expected_count_with_timeout(6, Duration::from_millis(200));
            assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "skill_point".to_string(), value: character.status.skill_point, }));
            assert_sent_persistence_event!(context, PersistenceEvent::ResetSkills(ResetSkills { char_id: character.char_id as i32, skills: vec![], }));
            // Platinium skill are not reset
            assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcSkillinfoList::packet_id(GlobalConfigService::instance().packetver()), 1)]));
        }
    }

    #[test]
    fn test_change_job_should_defer_db_update() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_job(&mut character, JobName::Assassin, false);
        // Then
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "class".to_string(), value: JobName::Assassin.value() as u32, }));
    }

    #[test]
    fn test_change_job_should_notify_sprite_change() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.change_job(&mut character, JobName::Assassin, false);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_id(PacketZcSpriteChange2::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_load_units_in_fov_should_add_new_item_in_character_map_view() {
        // Given
        let _context = before_each(mocked_repository());

        // When

        // Then
    }

    #[test]
    fn test_load_units_in_fov_should_remove_out_of_fov_item_from_character_map_view() {
        // Given
        let _context = before_each(mocked_repository());

        // When

        // Then
    }


    #[test]
    fn test_get_status_point_count_for_level() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            level: u16,
            job: &'a str,
            expected: u32,
        }
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", expected: 48 },
            Scenarii { level: 63, job: "Thief", expected: 600 },
            Scenarii { level: 99, job: "Assassin", expected: 1273 },
            Scenarii { level: 1, job: "Novice High", expected: 100 },
            Scenarii { level: 63, job: "Archer High", expected: 652 },
            Scenarii { level: 99, job: "Clown", expected: 1325 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            character.status.base_level = scenarii.level as u32;
            // When
            let status_point_count = context.character_service.get_status_point_count_for_level(&character);
            // Then
            assert_eq!(status_point_count, scenarii.expected, "Expected character of class {} at level {} to have {} status point but got {}", scenarii.job, scenarii.level, scenarii.expected, status_point_count);
        }
    }

    #[test]
    fn test_get_status_point_allocated() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            level: u16,
            job: &'a str,
            str: u16,
            agi: u16,
            dex: u16,
            int: u16,
            luk: u16,
            vit: u16,
            expected_allocated: u32,
        }
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", str: 19, agi: 3, dex: 1, int: 1, luk: 1, vit: 1, expected_allocated: 48 },
            Scenarii { level: 63, job: "Thief", str: 31, agi: 77, dex: 33, int: 1, luk: 1, vit: 1, expected_allocated: 594 },
            Scenarii { level: 99, job: "Assassin", str: 85, agi: 99, dex: 44, int: 1, luk: 1, vit: 1, expected_allocated: 1266 },
            Scenarii { level: 1, job: "Novice High", str: 33, agi: 1, dex: 1, int: 1, luk: 1, vit: 1, expected_allocated: 100 },
            Scenarii { level: 63, job: "Archer High", str: 0, agi: 48, dex: 33, int: 1, luk: 1, vit: 54, expected_allocated: 503 },
            Scenarii { level: 99, job: "Clown", str: 1, agi: 1, dex: 99, int: 75, luk: 66, vit: 1, expected_allocated: 1324 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            character.status.base_level = scenarii.level as u32;
            character.status.str = scenarii.str;
            character.status.agi = scenarii.agi;
            character.status.dex = scenarii.dex;
            character.status.int = scenarii.int;
            character.status.luk = scenarii.luk;
            character.status.vit = scenarii.vit;
            // When
            let status_point_count = context.character_service.get_spent_status_point(&character);
            // Then
            assert_eq!(status_point_count, scenarii.expected_allocated, "Expected character of class {} at level {} to have {} status point but got {}", scenarii.job, scenarii.level, scenarii.expected_allocated, status_point_count);
        }
    }


    #[test]
    fn test_calculate_status_point_delta_when_leveling_up_1_level_from_level_1() {
        // Given
        let context = before_each(mocked_repository());
        // When
        let result = context.character_service.calculate_status_point_delta(1, 2);
        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn test_calculate_status_point_delta_when_leveling_up_63_level_from_level_1() {
        // Given
        let context = before_each(mocked_repository());
        // When
        let result = context.character_service.calculate_status_point_delta(1, 63);
        // Then
        assert_eq!(result, 552);
    }

    #[test]
    fn test_calculate_status_point_delta_when_leveling_up_1_level_from_level_63() {
        // Given
        let context = before_each(mocked_repository());
        // When
        let result = context.character_service.calculate_status_point_delta(63, 64);
        // Then
        assert_eq!(result, 15);
    }

    #[test]
    fn test_calculate_status_point_delta_when_leveling_down_1_level_from_level_2() {
        // Given
        let context = before_each(mocked_repository());
        // When
        let result = context.character_service.calculate_status_point_delta(2, 1);
        // Then
        assert_eq!(result, -3);
    }

    #[test]
    fn test_calculate_status_point_delta_when_leveling_down_63_level_from_level_63() {
        // Given
        let context = before_each(mocked_repository());
        // When
        let result = context.character_service.calculate_status_point_delta(63, 1);
        // Then
        assert_eq!(result, -552);
    }

    #[test]
    fn test_reset_stats() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            level: u16,
            job: &'a str,
            str: u16,
            agi: u16,
            dex: u16,
            int: u16,
            luk: u16,
            vit: u16,
            status_point_expected: u32,
        }
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", str: 19, agi: 3, dex: 0, int: 0, luk: 0, vit: 0, status_point_expected: 48 },
            Scenarii { level: 63, job: "Thief", str: 31, agi: 77, dex: 33, int: 0, luk: 0, vit: 0, status_point_expected: 600 },
            Scenarii { level: 99, job: "Assassin", str: 85, agi: 99, dex: 44, int: 0, luk: 0, vit: 0, status_point_expected: 1273 },
            Scenarii { level: 1, job: "Novice High", str: 33, agi: 0, dex: 0, int: 0, luk: 0, vit: 0, status_point_expected: 100 },
            Scenarii { level: 63, job: "Archer High", str: 0, agi: 48, dex: 33, int: 0, luk: 0, vit: 54, status_point_expected: 652 },
            Scenarii { level: 99, job: "Clown", str: 0, agi: 0, dex: 99, int: 75, luk: 66, vit: 0, status_point_expected: 1325 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            character.status.base_level = scenarii.level as u32;
            character.status.str = scenarii.str;
            character.status.agi = scenarii.agi;
            character.status.dex = scenarii.dex;
            character.status.int = scenarii.int;
            character.status.luk = scenarii.luk;
            character.status.vit = scenarii.vit;
            // When
            context.character_service.reset_stats(&mut character);
            let status_point_count = character.status.status_point;
            // Then
            assert_eq!(character.status.str, 1);
            assert_eq!(character.status.dex, 1);
            assert_eq!(character.status.agi, 1);
            assert_eq!(character.status.vit, 1);
            assert_eq!(character.status.int, 1);
            assert_eq!(character.status.luk, 1);
            assert_eq!(status_point_count, scenarii.status_point_expected, "Expected character of class {} at level {} to have {} status point after stats reset but got {}", scenarii.job, scenarii.level, scenarii.status_point_expected, status_point_count);
        }
    }

    #[test]
    fn test_should_reset_stats_should_return_true_when_character_has_more_status_point_allocated_than_available_for_its_level() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.base_level = 10;
        character.status.str = 99;
        character.status.vit = 99;
        // When
        let result = context.character_service.should_reset_stats(&mut character);
        // Then
        assert!(result);
    }

    #[test]
    fn test_should_reset_stats_should_return_false_when_character_has_less_status_point_allocated_than_available_for_its_level() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.base_level = 10;
        character.status.str = 9;
        character.status.vit = 9;
        character.status.agi = 9;
        // When
        let result = context.character_service.should_reset_stats(&mut character);
        // Then
        assert!(!result);
    }


    #[test]
    fn test_should_reset_skills() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            job: usize,
            job_level: u32,
            skill_points: u8,
            skills: Vec<KnownSkill>,
            expected: bool,
        }

        let scenario = vec![
            Scenarii { job: JobName::Novice.value(), job_level: 10, skill_points: 0, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 }], expected: false },
            Scenarii { job: JobName::Novice.value(), job_level: 8, skill_points: 0, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 }], expected: true },
            Scenarii { job: JobName::Novice.value(), job_level: 8, skill_points: 1, skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 6 }], expected: false },
            Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 8,
                skill_points: 0,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 7 }, ],
                expected: false,
            },
            Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 8,
                skill_points: 2,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 5 }, ],
                expected: false,
            },
            Scenarii {
                job: JobName::Swordsman.value(),
                job_level: 8,
                skill_points: 2,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 7 }, ],
                expected: true,
            },
            Scenarii {
                job: JobName::Knight.value(),
                job_level: 8,
                skill_points: 0,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_RECOVERY"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_MAGNUM"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_ENDURE"), level: 10 }, ],
                expected: false,
            },
            Scenarii {
                job: JobName::Knight.value(),
                job_level: 8,
                skill_points: 5,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_RECOVERY"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_MAGNUM"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_ENDURE"), level: 10 }, ],
                expected: true,
            },
            Scenarii {
                job: JobName::Knight.value(),
                job_level: 1,
                skill_points: 0,
                skills: vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                             KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_RECOVERY"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_BASH"), level: 7 },
                             KnownSkill { value: SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                             KnownSkill { value: SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                             KnownSkill { value: SkillEnum::from_name("SM_ENDURE"), level: 5 }, ],
                expected: true,
            },
        ];

        for scenarii in scenario {
            // Given
            let mut character = create_character();
            character.status.job = scenarii.job as u32;
            character.status.job_level = scenarii.job_level;
            character.status.known_skills = scenarii.skills;
            character.status.skill_point = scenarii.skill_points as u32;
            // When
            let result = context.character_service.should_reset_skills(&mut character);
            // Then
            assert_eq!(result, scenarii.expected, "Expected should_reset_skills to be {} when job is {} and lvl {} with {} free skill points and {} allocated skill point", scenarii.expected, JobName::from_value(scenarii.job).as_str(), scenarii.job_level, scenarii.skill_points, context.character_service.get_allocated_skills_point(&character));
        }
    }

    #[test]
    fn test_update_status_point_should_defer_update_in_db_and_send_packet() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.update_status_point(&mut character, 10);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "status_point".to_string(), value: 10, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_id(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn test_reset_stat_should_defer_update_in_db_and_send_packet() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        // When
        context.character_service.reset_stats(&mut character);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(8, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "status_point".to_string(), value: character.status.status_point, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "str".to_string(), value: 1, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "agi".to_string(), value: 1, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "dex".to_string(), value: 1, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "vit".to_string(), value: 1, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "int".to_string(), value: 1, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "luk".to_string(), value: 1, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()), 1)]));
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterUpdateClientSideStats(character.char_id), 0);
    }

    #[test]
    fn test_reset_skills_defer_update_in_db_and_send_packet() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.known_skills = vec![KnownSkill { value: SkillEnum::from_name("NV_BASIC"), level: 9 },
                                      KnownSkill { value: SkillEnum::from_name("SM_SWORD"), level: 10 },
                                      KnownSkill { value: SkillEnum::from_name("SM_FATALBLOW"), level: 1 }, // Platinium
                                      KnownSkill { value: SkillEnum::from_name("SM_TWOHAND"), level: 10 },
                                      KnownSkill { value: SkillEnum::from_name("SM_RECOVERY"), level: 5 },
                                      KnownSkill { value: SkillEnum::from_name("SM_BASH"), level: 7 },
                                      KnownSkill { value: SkillEnum::from_name("SM_PROVOKE"), level: 5 },
                                      KnownSkill { value: SkillEnum::from_name("SM_MAGNUM"), level: 10 },
                                      KnownSkill { value: SkillEnum::from_name("SM_ENDURE"), level: 5 },
                                      KnownSkill { value: SkillEnum::from_name("KN_SPEARMASTERY"), level: 1 },
                                      KnownSkill { value: SkillEnum::from_name("KN_PIERCE"), level: 3 },
                                      KnownSkill { value: SkillEnum::from_name("KN_TWOHANDQUICKEN"), level: 8 },
                                      KnownSkill { value: SkillEnum::from_name("KN_AUTOCOUNTER"), level: 5 },
                                      KnownSkill { value: SkillEnum::from_name("KN_RIDING"), level: 1 },
                                      KnownSkill { value: SkillEnum::from_name("KN_CAVALIERMASTERY"), level: 5 }, ];
        let skills_to_reset: Vec<i32> = character.status.known_skills.iter().filter(|s| s.value != SkillEnum::from_name("SM_FATALBLOW")).map(|s| s.value.id() as i32).collect();
        // When
        context.character_service.reset_skills(&mut character, true);
        // Then
        assert_eq!(character.status.skill_point, 84);
        context.test_context.increment_latch().wait_expected_count_with_timeout(4, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "skill_point".to_string(), value: character.status.skill_point, }));
        assert_sent_persistence_event!(context, PersistenceEvent::ResetSkills(ResetSkills { char_id: character.char_id as i32, skills: skills_to_reset, }));
        // Platinium skill are not reset
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcSkillinfoList::packet_id(GlobalConfigService::instance().packetver()), 1)]));
    }

    #[test]
    fn test_allocate_status_point_allocate_status_point_to_stat() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.str = 1;
        character.status.int = 2;
        character.status.agi = 3;
        character.status.dex = 4;
        character.status.vit = 5;
        character.status.luk = 6;
        character.status.status_point = 1000;
        // When
        context.character_service.allocate_status_point(&mut character, StatusTypes::Str, 10);
        context.character_service.allocate_status_point(&mut character, StatusTypes::Int, 11);
        context.character_service.allocate_status_point(&mut character, StatusTypes::Agi, 12);
        context.character_service.allocate_status_point(&mut character, StatusTypes::Dex, 13);
        context.character_service.allocate_status_point(&mut character, StatusTypes::Vit, 14);
        context.character_service.allocate_status_point(&mut character, StatusTypes::Luk, 15);
        // Then
        assert_eq!(character.status.str, 11);
        assert_eq!(character.status.int, 13);
        assert_eq!(character.status.agi, 15);
        assert_eq!(character.status.dex, 17);
        assert_eq!(character.status.vit, 19);
        assert_eq!(character.status.luk, 21);
    }

    #[test]
    fn test_allocate_status_point_should_decrease_available_status_point() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            available_status_point: u32,
            initial_stat_level: u16,
            value_to_add: u16,
            expected_available_status_point: u32,
        }
        let scenario = vec![
            Scenarii { available_status_point: 3, initial_stat_level: 60, value_to_add: 10, expected_available_status_point: 3 },
            Scenarii { available_status_point: 14, initial_stat_level: 60, value_to_add: 2, expected_available_status_point: 0 },
            Scenarii { available_status_point: 7, initial_stat_level: 60, value_to_add: 1, expected_available_status_point: 0 },
            Scenarii { available_status_point: 14, initial_stat_level: 60, value_to_add: 3, expected_available_status_point: 14 },
            Scenarii { available_status_point: 23, initial_stat_level: 60, value_to_add: 3, expected_available_status_point: 1 },
            Scenarii { available_status_point: 22, initial_stat_level: 60, value_to_add: 10, expected_available_status_point: 22 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.str = scenarii.initial_stat_level;
            character.status.status_point = scenarii.available_status_point;
            // When
            context.character_service.allocate_status_point(&mut character, StatusTypes::Str, scenarii.value_to_add);
            // Then
            assert_eq!(character.status.status_point, scenarii.expected_available_status_point);
        }
    }

    #[test]
    fn test_allocate_status_point_should_ensure_enough_status_point_are_available() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii {
            available_status_point: u32,
            initial_stat_level: u16,
            value_to_add: u16,
            expected_can_raise_stat: bool,
        }
        let scenario = vec![
            Scenarii { available_status_point: 0, initial_stat_level: 60, value_to_add: 10, expected_can_raise_stat: false },
            Scenarii { available_status_point: 14, initial_stat_level: 60, value_to_add: 2, expected_can_raise_stat: true },
            Scenarii { available_status_point: 7, initial_stat_level: 60, value_to_add: 1, expected_can_raise_stat: true },
            Scenarii { available_status_point: 14, initial_stat_level: 60, value_to_add: 3, expected_can_raise_stat: false },
            Scenarii { available_status_point: 22, initial_stat_level: 60, value_to_add: 3, expected_can_raise_stat: true },
            Scenarii { available_status_point: 22, initial_stat_level: 60, value_to_add: 10, expected_can_raise_stat: false },
        ];

        for scenarii in scenario {
            let mut character = create_character();
            character.status.str = scenarii.initial_stat_level;
            character.status.status_point = scenarii.available_status_point;
            // When
            context.character_service.allocate_status_point(&mut character, StatusTypes::Str, scenarii.value_to_add);
            // Then
            if scenarii.expected_can_raise_stat {
                assert_eq!(character.status.str, scenarii.initial_stat_level + scenarii.value_to_add);
            } else {
                assert_eq!(character.status.str, scenarii.initial_stat_level);
            }
        }
    }

    #[test]
    fn test_allocate_status_point_should_trigger_calculate_stat() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.str = 98;
        character.status.status_point = 1000;
        // When
        context.character_service.allocate_status_point(&mut character, StatusTypes::Str, 1);
        // Then
        assert_task_queue_contains_event_at_tick!(context.server_task_queue, GameEvent::CharacterUpdateClientSideStats(character.char_id), 0);
    }

    #[test]
    fn test_allocate_status_point_cannot_exceed_max_stat_level_configured() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.str = 99;
        character.status.status_point = 1000;
        // When
        context.character_service.allocate_status_point(&mut character, StatusTypes::Str, 1);
        // Then
        assert_eq!(character.status.str, 99);
    }

    #[test]
    fn test_allocate_status_point_should_defer_stat_update_and_status_point_update_in_db() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.str = 98;
        character.status.status_point = 1000;
        // When
        context.character_service.allocate_status_point(&mut character, StatusTypes::Str, 1);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "status_point".to_string(), value: character.status.status_point, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "str".to_string(), value: 99, }));
    }

    #[test]
    fn test_allocate_skill_point_should_put_point_on_skill_and_decrease_available_skill_point_trigger_skill_list_packet_send() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job_level = 2;
        character.status.skill_point = 1;
        // When
        context.character_service.allocate_skill_point(&mut character, SkillEnum::NvBasic);
        // Then
        assert_eq!(character.status.known_skills.iter().find(|s| matches!(s.value, SkillEnum::NvBasic)).unwrap().level, 1);
        assert_eq!(character.status.skill_point, 0);
        context.test_context.increment_latch().wait_expected_count_with_timeout(4, Duration::from_millis(200));
        assert_sent_persistence_event!(context, PersistenceEvent::IncreaseSkillLevel(IncreaseSkillLevel { char_id: character.char_id as i32, skill: SkillEnum::NvBasic, increment: 1, }));
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id as u32, db_column: "skill_point".to_string(), value: 0, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()), 1)]));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcSkillinfoList::packet_id(GlobalConfigService::instance().packetver()), 1)]));
    }

    #[test]
    fn test_allocate_skill_point_should_ensure_enough_skill_point_are_available() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job_level = 2;
        character.status.skill_point = 0;
        // When
        let result = context.character_service.allocate_skill_point(&mut character, SkillEnum::NvBasic);
        // Then
        assert!(!result);
    }

    #[test]
    fn test_allocate_skill_point_should_do_nothing_when_skill_is_not_available_in_tree() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job_level = 2;
        character.status.skill_point = 1;
        // When
        let result = context.character_service.allocate_skill_point(&mut character, SkillEnum::SmBash);
        // Then
        assert!(!result);
    }

    #[test]
    fn test_gain_exp_should_update_in_memory_defer_update_in_db_and_send_packet() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.base_exp = 10;
        character.status.base_level = 10;
        // When
        context.character_service.gain_base_exp(&mut character, 100);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_eq!(character.status.base_exp, 110);
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "base_exp".to_string(), value: character.status.base_exp, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()), 1)]));
    }

    #[test]
    #[ignore]
    fn test_gain_exp_should_apply_rate_modifier() {
        // Given
        let context = before_each(mocked_repository());
        let mut config = GlobalConfigService::instance().config().clone();
        config.game.base_exp_rate = 10.0;
        unsafe { GlobalConfigService::instance_mut().set_config(config); }
        let mut character = create_character();
        character.status.base_exp = 10;
        character.status.base_level = 10;
        // When
        context.character_service.gain_base_exp(&mut character, 10);
        // Then
        assert_eq!(character.status.base_exp, 110);
        let mut config = GlobalConfigService::instance().config().clone();
        config.game.base_exp_rate = 1.0;
        unsafe { GlobalConfigService::instance_mut().set_config(config); }
    }

    #[test]
    fn test_gain_exp_should_level_up_when_character_exp_is_above_next_level_exp_requirement() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            level: u32,
            job: &'a str,
            exp: u32,
            gain_exp: u32,
            expected_level: u32,
            expected_exp: u32,
        }
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", exp: 0, gain_exp: 4, expected_level: 1, expected_exp: 4 },
            Scenarii { level: 1, job: "Novice", exp: 0, gain_exp: 12, expected_level: 2, expected_exp: 3 },
            Scenarii { level: 21, job: "Novice", exp: 263, gain_exp: 9631, expected_level: 25, expected_exp: 1300 },
            Scenarii { level: 98, job: "Hunter", exp: 99999997, gain_exp: 9631, expected_level: 99, expected_exp: 0 },
            Scenarii { level: 99, job: "Hunter", exp: 0, gain_exp: 9631, expected_level: 99, expected_exp: 0 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.base_exp = scenarii.exp;
            character.status.base_level = scenarii.level;
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            // When
            context.character_service.gain_base_exp(&mut character, scenarii.gain_exp);
            // Then
            assert_eq!(character.status.base_level, scenarii.expected_level, "Expected {} from level {} to be level {} after gaining {} exp but got {}", scenarii.job, scenarii.level, scenarii.expected_level, scenarii.gain_exp, character.status.base_level);
            assert_eq!(character.status.base_exp, scenarii.expected_exp, "Expected {} at level {} to have {} exp after gaining {} exp but got {}", scenarii.job, character.status.base_level, scenarii.expected_exp, scenarii.gain_exp, character.status.base_exp);
        }
    }

    #[test]
    fn test_gain_job_exp_should_update_in_memory_defer_update_in_db_and_send_packet() {
        // Given
        let context = before_each(mocked_repository());
        let mut character = create_character();
        character.status.job_exp = 10;
        character.status.job_level = 9;
        // When
        context.character_service.gain_job_exp(&mut character, 60);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_eq!(character.status.job_exp, 70);
        assert_sent_persistence_event!(context, PersistenceEvent::UpdateCharacterStatusU32(StatusUpdate { char_id: character.char_id, db_column: "job_exp".to_string(), value: character.status.job_exp, }));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcParChange::packet_id(GlobalConfigService::instance().packetver()), 1)]));
    }

    #[test]
    #[ignore]
    fn test_gain_job_exp_should_apply_rate_modifier() {
        // Given
        let context = before_each(mocked_repository());
        let mut config = GlobalConfigService::instance().config().clone();
        config.game.job_exp_rate = 5.0;
        unsafe { GlobalConfigService::instance_mut().set_config(config); }
        let mut character = create_character();
        character.status.job_exp = 10;
        character.status.job_level = 8;
        // When
        context.character_service.gain_job_exp(&mut character, 10);
        // Then
        assert_eq!(character.status.job_exp, 60);
        let mut config = GlobalConfigService::instance().config().clone();
        config.game.job_exp_rate = 1.0;
        unsafe { GlobalConfigService::instance_mut().set_config(config); }
    }

    #[test]
    fn test_gain_job_exp_should_job_level_up_when_character_exp_is_above_next_job_level_exp_requirement() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            job_level: u32,
            job: &'a str,
            job_exp: u32,
            gain_exp: u32,
            expected_job_level: u32,
            expected_job_exp: u32,
        }
        let scenario = vec![
            Scenarii { job_level: 1, job: "Novice", job_exp: 0, gain_exp: 4, expected_job_level: 1, expected_job_exp: 4 },
            Scenarii { job_level: 1, job: "Novice", job_exp: 0, gain_exp: 12, expected_job_level: 2, expected_job_exp: 2 },
            Scenarii { job_level: 21, job: "Archer", job_exp: 1125, gain_exp: 54411, expected_job_level: 27, expected_job_exp: 802 },
            Scenarii { job_level: 49, job: "Hunter", job_exp: 2083380, gain_exp: 169, expected_job_level: 50, expected_job_exp: 0 },
            Scenarii { job_level: 50, job: "Hunter", job_exp: 0, gain_exp: 9631, expected_job_level: 50, expected_job_exp: 0 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job_exp = scenarii.job_exp;
            character.status.job_level = scenarii.job_level;
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            // When
            context.character_service.gain_job_exp(&mut character, scenarii.gain_exp);
            // Then
            assert_eq!(character.status.job_level, scenarii.expected_job_level, "Expected {} from job level {} to be job level {} after gaining {} exp but got {}", scenarii.job, scenarii.job_level, scenarii.expected_job_level, scenarii.gain_exp, character.status.job_level);
            assert_eq!(character.status.job_exp, scenarii.expected_job_exp, "Expected {} at job level {} to have {} job exp after gaining {} job exp but got {}", scenarii.job, character.status.job_level, scenarii.expected_job_exp, scenarii.gain_exp, character.status.job_exp);
        }
    }

    #[test]
    fn test_next_base_level_required_exp() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            level: u32,
            job: &'a str,
            required_exp: u32,
        }
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", required_exp: 9 },
            Scenarii { level: 54, job: "Archer", required_exp: 178184 },
            Scenarii { level: 95, job: "Hunter", required_exp: 35658000 },
            Scenarii { level: 1, job: "Novice High", required_exp: 10 },
            Scenarii { level: 54, job: "Archer High", required_exp: 329640 },
            Scenarii { level: 95, job: "Sniper", required_exp: 106974000 },
            Scenarii { level: 98, job: "Sniper", required_exp: 343210000 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.base_level = scenarii.level;
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            // When
            let required_exp = context.character_service.next_base_level_required_exp(&character.status);
            // Then
            assert_eq!(required_exp, scenarii.required_exp, "Expected {} at level {} to need {} exp to reach next level but got {}", scenarii.job, scenarii.level, scenarii.required_exp, required_exp);
        }
    }

    #[test]
    fn test_next_job_level_required_exp() {
        // Given
        let context = before_each(mocked_repository());
        struct Scenarii<'a> {
            level: u32,
            job: &'a str,
            required_exp: u32,
        }
        let scenario = vec![
            Scenarii { level: 1, job: "Novice", required_exp: 10 },
            Scenarii { level: 34, job: "Archer", required_exp: 52417 },
            Scenarii { level: 45, job: "Hunter", required_exp: 1260955 },
            Scenarii { level: 1, job: "Novice High", required_exp: 11 },
            Scenarii { level: 34, job: "Archer High", required_exp: 131043 },
            Scenarii { level: 45, job: "Sniper", required_exp: 3782865 },
            Scenarii { level: 68, job: "Sniper", required_exp: 167071930 },
            Scenarii { level: 68, job: "Ninja", required_exp: 5123654 },
            Scenarii { level: 68, job: "Gunslinger", required_exp: 5123654 },
            Scenarii { level: 45, job: "Taekwon", required_exp: 2521910 },
            Scenarii { level: 45, job: "Soul Linker", required_exp: 2521910 },
        ];
        for scenarii in scenario {
            let mut character = create_character();
            character.status.job_level = scenarii.level;
            character.status.job = JobName::from_string(scenarii.job).value() as u32;
            // When
            let required_exp = context.character_service.next_job_level_required_exp(&character.status);
            // Then
            assert_eq!(required_exp, scenarii.required_exp, "Expected {} at job level {} to need {} exp to reach next level but got {}", scenarii.job, scenarii.level, scenarii.required_exp, required_exp);
        }
    }


    #[test]
    fn character_increase_stat_point_should_send_ack_packet() {
        // Given
        let context = before_each(mocked_repository());
        let mut character_state = create_character();
        // When
        context.character_service.character_increase_stat(&mut character_state, CharacterUpdateStat {
            char_id: 150000,
            stat_id: StatusTypes::Str.value() as u16,
            change_amount: 1,
        });
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(3, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character_state.char_id, vec![SentPacket::with_id(PacketZcStatusChangeAck::packet_id(GlobalConfigService::instance().packetver()))]));
    }

    #[test]
    fn character_kill_monster_should_trigger_map_mob_drop_items_when_autoloot_disabled_and_reward_attacker_with_exp() {
        // Given
        let context = before_each(mocked_repository());
        let mut character_state = create_character();
        let task_queue = Arc::new(TasksQueue::new());
        let map_instance = create_empty_map_instance(context.test_context.client_notification_sender(), task_queue.clone());
        // When
        let char_id = character_state.char_id;
        character_state.status.base_exp = 10;
        character_state.status.base_level = 10;
        character_state.status.job_exp = 5;
        character_state.status.job_level = 9;
        context.character_service.character_kill_monster(&mut character_state, CharacterKillMonster {
            char_id,
            mob_id: 1001,
            mob_x: 54,
            mob_y: 54,
            map_instance_key: map_instance.key().clone(),
            mob_base_exp: 100,
            mob_job_exp: 60,
        }, &map_instance);
        // Then
        assert_task_queue_contains_event!(task_queue, MapEvent::MobDropItems(MobDropItems { owner_id: char_id, mob_id: 1001, mob_x: 54, mob_y: 54 }));
        assert_eq!(character_state.status.base_exp, 110);
        assert_eq!(character_state.status.job_exp, 65);
    }
}