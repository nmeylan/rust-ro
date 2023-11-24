#![allow(dead_code)]
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct StatusServiceTestContext {
    test_context: TestContext,
    status_service: StatusService,
}

fn before_each() -> StatusServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> StatusServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    StatusServiceTestContext {
        test_context:TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        status_service: StatusService::new(client_notification_sender, persistence_event_sender, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    
    use enums::class::JobName;
    use enums::weapon::WeaponType;
    use crate::enums::EnumWithStringValue;
    use crate::enums::EnumWithNumberValue;
    
    use crate::tests::common::character_helper::{create_character, equip_item_from_name};
    use super::*;

    #[test]
    fn test_right_hand_weapon_type_is_returning_the_right_hand_weapon_when_character_has_one() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item_from_name(&mut character, "Knife");
        // When
        let weapon_type = context.status_service.right_hand_weapon_type(&character.status);
        // Then
        assert_eq!(weapon_type, WeaponType::Dagger);
    }

    #[test]
    fn test_aspd() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, agi: u16, dex: u16, job: &'a str, expected_aspd: u16 }
        let stats = vec![
            Stats { weapon: "", agi: 1, dex: 1, job: "Novice", expected_aspd: 150 },
            Stats { weapon: "Knife", agi: 1, dex: 1, job: "Novice", expected_aspd: 135 },
            Stats { weapon: "Knife", agi: 15, dex: 15, job: "Novice", expected_aspd: 140 },
            Stats { weapon: "Knife", agi: 1, dex: 1, job: "Swordsman", expected_aspd: 150 },
            Stats { weapon: "Sword", agi: 1, dex: 1, job: "Swordsman", expected_aspd: 145 },
            Stats { weapon: "Bow", agi: 1, dex: 1, job: "Archer", expected_aspd: 130 },
            Stats { weapon: "", agi: 1, dex: 1, job: "Merchant", expected_aspd: 160 },
        ];
        for stat in stats {
            let mut character = create_character();
            character.status.agi = stat.agi;
            character.status.dex = stat.dex;
            character.status.job = JobName::from_string(stat.job).value() as u32;
            if !stat.weapon.is_empty() {
                equip_item_from_name(&mut character, stat.weapon);
            }
            // When
            let aspd = context.status_service.aspd(&character.status).round() as u16;
            // Then
            assert_eq!(aspd, stat.expected_aspd, "Expected aspd to be {} but was {} with stats {:?}", stat.expected_aspd, aspd, stat);
        }
    }

    #[test]
    fn test_client_side_aspd() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item_from_name(&mut character, "Knife");
        // When
        let aspd = context.status_service.aspd(&character.status);
        let client_side_aspd = context.status_service.client_aspd(aspd);
        // Then
        assert_eq!(client_side_aspd, 648);
    }

    #[test]
    fn test_attack_motion_delay() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item_from_name(&mut character, "Knife");
        // When
        let attack_motion = context.status_service.attack_motion(&character.status);
        // Then
        assert_eq!(attack_motion, 1296);
    }

    #[test]
    fn test_status_left_side_atk() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, str: u16, dex: u16, luk: u16, expected_status_atk: i32 }
        let stats = vec![
            Stats { weapon: "Knife", str: 1, dex: 1, luk: 1, expected_status_atk: 18 },
            Stats { weapon: "Knife", str: 5, dex: 1, luk: 1, expected_status_atk: 22 },
            Stats { weapon: "Knife", str: 5, dex: 1, luk: 5, expected_status_atk: 23 },
            Stats { weapon: "Knife", str: 5, dex: 10, luk: 5, expected_status_atk: 25 },
            Stats { weapon: "Bow", str: 1, dex: 1, luk: 1, expected_status_atk: 16 },
            Stats { weapon: "Bow", str: 1, dex: 5, luk: 1, expected_status_atk: 20 },
            Stats { weapon: "Bow", str: 1, dex: 5, luk: 5, expected_status_atk: 21 },
            Stats { weapon: "Bow", str: 10, dex: 5, luk: 5, expected_status_atk: 23 },
        ];
        for stat in stats {
            let mut character = create_character();
            character.status.str = stat.str;
            character.status.dex = stat.dex;
            character.status.luk = stat.luk;
            if !stat.weapon.is_empty() {
                equip_item_from_name(&mut character, stat.weapon);
            }
            // When
            let status_atk = context.status_service.status_atk_left_side(&character.status);
            // Then
            assert_eq!(status_atk, stat.expected_status_atk, "Expected status atk1 to be {} but was {} with stats {:?}", stat.expected_status_atk, status_atk, stat);
        }
    }

    #[test]
    fn test_attack_per_seconds() {
        // Given
        let context = before_each();
        let _character = create_character();
        for (aspd, expectation) in [(150.0_f32, "1.00"), (170.0_f32, "1.67"), (190.0_f32, "5.00"), (199.0_f32, "50.00")].iter() {
            // When
            let attack_motion = context.status_service.attack_per_seconds(*aspd);
            // Then
            assert_eq!(format!("{attack_motion:.2}"), **expectation, "Expected attack motion to be {expectation} with aspd {aspd} but was {attack_motion}");
        }
    }
    #[test]
    fn test_mob_vit_def() {
        // Given
        let context = before_each();
        let _character = create_character();
        for vit in [0, 1, 2, 45, 88].iter() {
            // When
            let actual_vit = context.status_service.mob_vit_def(*vit);
            // Then
            assert!(actual_vit >= *vit, "Expected actual_vit {} to be greater or equal to {}", actual_vit, vit);
        }
    }
}