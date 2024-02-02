#![allow(dead_code)]

use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct BattleServiceTestContext {
    test_context: TestContext,
    battle_service: BattleService,
    battle_min_service: BattleService,
    battle_max_service: BattleService,
    status_service: StatusService,
}

fn before_each() -> BattleServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> BattleServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    BattleServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        battle_service: BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance(), BattleResultMode::Normal),
        battle_min_service: BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance(), BattleResultMode::TestMin),
        battle_max_service: BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance(), BattleResultMode::TestMax),
        status_service: StatusService::new(GlobalConfigService::instance()),
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use models::enums::size::Size;
    use models::status::Status;
    use crate::{assert_eq_with_variance, status_snapshot, status_snapshot_mob};
    use crate::server::model::map_item::{ToMapItemSnapshot};
    use crate::server::service::battle_service::BattleService;
    use crate::tests::battle_service_test::before_each;
    use crate::tests::common::character_helper::{create_character, equip_item_from_name};
    use crate::tests::common::mob_helper::create_mob;
    use crate::util::tick::get_tick;

    #[test]
    fn test_damage_character_attack_mob_melee() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> {
            weapon: &'a str,
            agi: u16,
            dex: u16,
            str: u16,
            luk: u16,
            mob: &'a str,
            min_damage: u32,
            max_damage: u32,
            average_damage: u32,
        }
        let stats = vec![
            Stats { weapon: "Knife", agi: 1, dex: 5, str: 5, luk: 1, mob: "LUNATIC", min_damage: 8, max_damage: 19, average_damage: 13 },
        ];
        for stat in stats {
            let mut character = create_character();
            let mob = create_mob(1, stat.mob);
            character.status.str = stat.str;
            character.status.dex = stat.dex;
            character.status.luk = stat.luk;
            if !stat.weapon.is_empty() {
                equip_item_from_name(&mut character, stat.weapon);
            }
            // When
            let min = context.battle_min_service.physical_damage_character_attack_monster(status_snapshot!(context, character), &mob.status, 1.0, false);
            let max = context.battle_max_service.physical_damage_character_attack_monster(status_snapshot!(context, character), &mob.status, 1.0, false);
            // Then
            assert!(stat.min_damage - 1 <= min && min <= stat.min_damage + 1, "Expected min damage to be {} but was {} with stats {:?}", stat.min_damage, min, stat);
            assert!(stat.max_damage - 1 <= max && max <= stat.max_damage + 1, "Expected max damage to be {} but was {} with stats {:?}", stat.max_damage, max, stat);
        }
    }

    #[test]
    fn test_damage_weapon_attack_should_depend_on_dex() {
        // Given
        let context = before_each();#[derive(Debug)]
        struct Stats<'a> {
            weapon: &'a str,
            dex: u16,
            average_damage: u32,
            min_damage: u32,
            max_damage: u32,
        }
        let stats = vec![
            Stats { weapon: "Knife", dex: 1, average_damage: 6, min_damage: 0, max_damage: 12 },
            Stats { weapon: "Knife", dex: 99, average_damage: 12, min_damage: 12, max_damage: 12 },
            // Stats { weapon: "Bow", dex: 50, average_damage: 94, min_damage: 76, max_damage: 113 },
        ];
        let target_status: Status = Default::default(); // default is medium size
        for stat in stats {
            let mut character = create_character();
            character.status.dex = stat.dex;
            // When
            equip_item_from_name(&mut character, stat.weapon);
            let min = context.battle_min_service.weapon_atk(status_snapshot!(context, character), &context.status_service.to_snapshot(&target_status), false);
            let max = context.battle_max_service.weapon_atk(status_snapshot!(context, character), &context.status_service.to_snapshot(&target_status), false);
            // Then
            assert_eq_with_variance!(2, min, stat.min_damage, "Expected min damage to be {} but was {} with stats {:?}", stat.min_damage, min, stat);
            assert_eq_with_variance!(2, max, stat.max_damage, "Expected max damage to be {} but was {} with stats {:?}", stat.max_damage, max, stat);
        }
    }

    #[test]
    fn test_attack_when_repeat_attack_is_true_should_not_clear_attack() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        character.set_attack(mob_item_id, true, 0);
        let second_attack_tick = get_tick() + 2000;
        let character_status =status_snapshot!(context, character);
        // When
        let attack_1 = context.battle_service.basic_attack(&mut character, mob.to_map_item_snapshot(), character_status, &mob.status, get_tick());
        let attack_2 = context.battle_service.basic_attack(&mut character, mob.to_map_item_snapshot(), character_status, &mob.status, second_attack_tick);
        // Then
        assert!(attack_1.is_some());
        assert!(attack_2.is_some());
        assert_eq!(character.attack.unwrap().last_attack_tick, second_attack_tick);
        assert_eq!(character.attack.unwrap().last_attack_motion, context.status_service.attack_motion(character_status));
    }

    #[test]
    fn test_attack_when_repeat_attack_is_false_should_clear_attack() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        character.set_attack(mob_item_id, false, 0);
        // When
        let character_status =status_snapshot!(context, character);
        let attack_1 = context.battle_service.basic_attack(&mut character, mob.to_map_item_snapshot(), character_status, &mob.status, get_tick());
        let attack_2 = context.battle_service.basic_attack(&mut character, mob.to_map_item_snapshot(), character_status, &mob.status, get_tick() + 2000);
        // Then
        assert!(attack_1.is_some());
        assert!(attack_2.is_none());
        assert!(character.attack.is_none());
    }

    #[test]
    fn test_attack_should_not_attack_if_there_is_delay() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        character.set_attack(mob_item_id, true, 0);
        // When
        let character_status =status_snapshot!(context, character);
        let attack_1 = context.battle_service.basic_attack(&mut character, mob.to_map_item_snapshot(), character_status, &mob.status, get_tick());
        let attack_2 = context.battle_service.basic_attack(&mut character, mob.to_map_item_snapshot(), character_status, &mob.status, get_tick());
        // Then
        assert!(attack_1.is_some());
        assert!(attack_2.is_none());
    }

    #[test]
    fn test_size_modifier() {
        // Given
        let context = before_each();
        struct Scenarii<'a> {
            weapon: Option<&'a str>,
            target_size: Size,
            expected_modifier: f32,
        }
        let _source_status: Status = Default::default();
        let mut target_status: Status = Default::default();
        let scenario = vec![
            Scenarii { weapon: None, target_size: Size::Small, expected_modifier: 1.0 },
            Scenarii { weapon: None, target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: None, target_size: Size::Large, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Knife"), target_size: Size::Small, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Knife"), target_size: Size::Medium, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Knife"), target_size: Size::Large, expected_modifier: 0.5 },
            Scenarii { weapon: Some("Sword"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Sword"), target_size: Size::Medium, expected_modifier: 1.0},
            Scenarii { weapon: Some("Sword"), target_size: Size::Large, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Slayer"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Slayer"), target_size: Size::Medium, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Slayer"), target_size: Size::Large, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Spear"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Spear"), target_size: Size::Medium, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Spear"), target_size: Size::Large, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Axe"), target_size: Size::Small, expected_modifier: 0.5 },
            Scenarii { weapon: Some("Axe"), target_size: Size::Medium, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Axe"), target_size: Size::Large, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Mace"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Mace"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Mace"), target_size: Size::Large, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Rod"), target_size: Size::Small, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Rod"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Rod"), target_size: Size::Large, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Bow"), target_size: Size::Small, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Bow"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Bow"), target_size: Size::Large, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Claw"), target_size: Size::Small, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Claw"), target_size: Size::Medium, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Claw"), target_size: Size::Large, expected_modifier: 0.5 },
            Scenarii { weapon: Some("Lute"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Lute"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Lute"), target_size: Size::Large, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Whip"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Whip"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Whip"), target_size: Size::Large, expected_modifier: 0.5 },
            Scenarii { weapon: Some("Book"), target_size: Size::Small, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Book"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Book"), target_size: Size::Large, expected_modifier: 0.5 },
            Scenarii { weapon: Some("Katar"), target_size: Size::Small, expected_modifier: 0.75 },
            Scenarii { weapon: Some("Katar"), target_size: Size::Medium, expected_modifier: 1.0 },
            Scenarii { weapon: Some("Katar"), target_size: Size::Large, expected_modifier: 0.75 },
        ];
        // When
        for scenarii in scenario {
            let mut character = create_character();
            if let Some(weapon) = scenarii.weapon {
                equip_item_from_name(&mut character, weapon);
            };
            target_status.size = scenarii.target_size;
            let size_modifier = BattleService::size_modifier(status_snapshot!(context, character), &context.status_service.to_snapshot(&target_status));
            assert_eq!(size_modifier, scenarii.expected_modifier, "Expected size modifier to be {} but was {} when weapon is {:?} and target size {:?}", scenarii.expected_modifier, size_modifier, scenarii.weapon, target_status.size)
        }
    }
}