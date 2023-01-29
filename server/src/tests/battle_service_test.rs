use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::battle_service::BattleService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};

struct BattleServiceTestContext {
    test_context: TestContext,
    battle_service: BattleService,
}

fn before_each() -> BattleServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let status_service =  StatusService::new(client_notification_sender.clone(), persistence_event_sender.clone(), GlobalConfigService::instance());
    BattleServiceTestContext {
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender, client_notification_receiver, persistence_event_receiver },
        battle_service: BattleService::new(client_notification_sender, status_service, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::tests::battle_service_test::before_each;
    use crate::tests::common::character_helper::{create_character, equip_item};

    #[test]
    fn test_damage_character_attack_mob_melee() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, agi: u16, dex: u16, str: u16, luk: u16, mob: &'a str, min_damage: u32, max_damage: u32, average_damage: u32 };
        let stats = vec![
            Stats { weapon: "Knife", agi: 1, dex: 5, str: 5, luk: 1, mob: "Lunatic", min_damage: 8, max_damage: 19, average_damage: 13 },
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
}