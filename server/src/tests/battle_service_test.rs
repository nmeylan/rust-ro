use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::battle_service::BattleService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct BattleServiceTestContext {
    test_context: TestContext,
    battle_service: BattleService,
    status_service: StatusService,
}

fn before_each() -> BattleServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> BattleServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let status_service =  StatusService::new(client_notification_sender.clone(), persistence_event_sender.clone(), GlobalConfigService::instance());
    let count_down_latch = CountDownLatch::new(latch_size);
    BattleServiceTestContext {
        test_context:TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        battle_service: BattleService::new(client_notification_sender.clone(), status_service, GlobalConfigService::instance()),
        status_service: StatusService::new(client_notification_sender.clone(), persistence_event_sender.clone(), GlobalConfigService::instance())
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_eq_with_variance;
    use crate::server::model::map_item::ToMapItem;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::battle_service_test::before_each;
    use crate::tests::common::character_helper::{create_character, equip_item};
    use crate::tests::common::mob_helper::create_mob;
    use crate::util::tick::get_tick;

    #[test]
    fn test_damage_character_attack_mob_melee() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, agi: u16, dex: u16, str: u16, luk: u16, mob: &'a str, min_damage: u32, max_damage: u32, average_damage: u32 };
        let stats = vec![
            Stats { weapon: "Knife", agi: 1, dex: 5, str: 5, luk: 1, mob: "LUNATIC", min_damage: 8, max_damage: 19, average_damage: 13 },
        ];
        for stat in stats {
            let mut character = create_character();
            let mob =  GlobalConfigService::instance().get_mob_by_name(stat.mob);
            character.status.str = stat.str;
            character.status.dex = stat.dex;
            character.status.luk = stat.luk;
            if !stat.weapon.is_empty() {
                equip_item(&mut character, stat.weapon);
            }
            // When
            let mut average = Vec::with_capacity(1001);
            let mut min = u32::MAX;
            let mut max = u32::MIN;
            for _ in 0..1000 {
                let damage = context.battle_service.damage_character_attack_monster_melee(&character, mob);
                average.push(damage);
                min = min.min(damage);
                max = max.max(damage);
            }
            let average = (average.iter().sum::<u32>() as f32 / average.len() as f32).round() as u32;
            // Then
            assert!(stat.average_damage - 1 <= average && average <= stat.average_damage + 1, "Expected average damage to be {} but was {} with stats {:?}", stat.average_damage, average, stat);
            assert!(stat.min_damage - 1 <= min && min <= stat.min_damage + 1, "Expected min damage to be {} but was {} with stats {:?}", stat.min_damage, min, stat);
            assert!(stat.max_damage - 1 <= max && max <= stat.max_damage + 1, "Expected max damage to be {} but was {} with stats {:?}", stat.max_damage, max, stat);
        }
    }

    #[test]
    fn test_damage_weapon_attack_should_depend_on_dex() {
        // Given
        let context = before_each();#[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, dex: u16, average_damage: u32 };
        let stats = vec![
            Stats { weapon: "Knife", dex: 1, average_damage: 9 },
            Stats { weapon: "Knife", dex: 100, average_damage: 17 },
        ];
        for stat in stats {
            let mut character = create_character();
            character.status.dex = stat.dex;
            // When
            let mut average = Vec::with_capacity(1001);
            let mut min = u32::MAX;
            let mut max = u32::MIN;
            for _ in 0..1000 {
                let damage = context.battle_service.weapon_atk(&character, Some(GlobalConfigService::instance().get_item_by_name(stat.weapon)));
                average.push(damage);
                min = min.min(damage);
                max = max.max(damage);
            }
            let average = (average.iter().sum::<u32>() as f32 / average.len() as f32).round() as u32;
            // Then
            assert_eq_with_variance!(1, average, stat.average_damage, "Expected average damage to be {} but was {} with stats {:?}", stat.average_damage, average, stat);
        }
    }
    
    #[test]
    fn test_attack_when_repeat_attack_is_true_should_not_clear_attack() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let mob_item_id = 82322;
        let mut mob = create_mob(mob_item_id, "PORING");
        character.set_attack(mob_item_id, true, 0);
        let second_attack_tick = get_tick() + 2000;
        // When
        let attack_1 = context.battle_service.attack(&mut character, mob.to_map_item(), get_tick());
        let attack_2 = context.battle_service.attack(&mut character, mob.to_map_item(), second_attack_tick);
        // Then
        assert!(attack_1.is_some());
        assert!(attack_2.is_some());
        assert_eq!(character.attack.unwrap().last_attack_tick, second_attack_tick);
        assert_eq!(character.attack.unwrap().last_attack_motion, context.status_service.attack_motion(&character));
    }

    #[test]
    fn test_attack_when_repeat_attack_is_false_should_clear_attack() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let mob_item_id = 82322;
        let mut mob = create_mob(mob_item_id, "PORING");
        character.set_attack(mob_item_id, false, 0);
        // When
        let attack_1 = context.battle_service.attack(&mut character, mob.to_map_item(), get_tick());
        let attack_2 = context.battle_service.attack(&mut character, mob.to_map_item(), get_tick() + 2000);
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
        let mut mob = create_mob(mob_item_id, "PORING");
        character.set_attack(mob_item_id, true, 0);
        // When
        let attack_1 = context.battle_service.attack(&mut character, mob.to_map_item(), get_tick());
        let attack_2 = context.battle_service.attack(&mut character, mob.to_map_item(), get_tick());
        // Then
        assert!(attack_1.is_some());
        assert!(attack_2.is_none());
    }
}