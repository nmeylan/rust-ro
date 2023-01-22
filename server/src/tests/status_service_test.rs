use std::sync::mpsc::SyncSender;
use std::sync::Once;
use crate::server::events::client_notification::Notification;
use crate::server::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TEST_CONTEXT, TestContext};
use crate::enums::{*};

struct StatusServiceTestContext {
    test_context: TestContext,
    status_service: StatusService,
}

fn before_each() -> StatusServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    StatusServiceTestContext {
        test_context: TestContext { client_notification_sender: client_notification_sender.clone(), persistence_event_sender: persistence_event_sender.clone(), client_notification_receiver, persistence_event_receiver },
        status_service: StatusService::new(client_notification_sender, persistence_event_sender, GlobalConfigService::instance()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use enums::class::JobName;
    use enums::weapon::WeaponType;
    use crate::server::core::map_instance::MapInstanceKey;
    use crate::server::state::character::Character;
    use crate::server::state::status::Status;
    use crate::tests::common::character_helper::{create_character, equip_item};
    use super::*;

    #[test]
    fn test_right_hand_weapon_type_is_returning_the_right_hand_weapon_when_character_has_one() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item(&mut character, "Knife");
        // When
        let weapon_type = context.status_service.right_hand_weapon_type(&character);
        // Then
        assert_eq!(weapon_type, WeaponType::Dagger);
    }

    #[test]
    fn test_aspd() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, agi: u16, dex: u16, job: &'a str, expected_aspd: u16 };
        let stats = vec![
            Stats { weapon: "", agi: 1, dex: 1, job: "Novice", expected_aspd: 150 },
            Stats { weapon: "Knife", agi: 1, dex: 1, job: "Novice", expected_aspd: 135 },
            Stats { weapon: "Knife", agi: 1, dex: 1, job: "Swordman", expected_aspd: 150 },
            Stats { weapon: "Sword", agi: 1, dex: 1, job: "Swordman", expected_aspd: 145 },
            Stats { weapon: "Bow", agi: 1, dex: 1, job: "Archer", expected_aspd: 130 },
            Stats { weapon: "Axe", agi: 1, dex: 1, job: "Archer", expected_aspd: 1 },
            Stats { weapon: "", agi: 1, dex: 1, job: "Merchant", expected_aspd: 160 },
        ];
        for stat in stats {
            let mut character = create_character();
            character.status.agi = stat.agi;
            character.status.dex = stat.dex;
            character.status.job = JobName::from_string(stat.job).value() as u32;
            if stat.weapon != "" {
                equip_item(&mut character, stat.weapon);
            }
            // When
            let aspd = context.status_service.aspd(&character).round() as u16;
            // Then
            assert_eq!(aspd, stat.expected_aspd, "Expected aspd to be {} but was {} with stats {:?}", stat.expected_aspd, aspd, stat);
        }
    }

    #[test]
    fn test_client_side_aspd() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item(&mut character, "Knife");
        // When
        let aspd = context.status_service.aspd(&character);
        let client_side_aspd = context.status_service.client_aspd(aspd);
        // Then
        assert_eq!(client_side_aspd, 498);
    }

    #[test]
    fn test_attack_motion_delay() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item(&mut character, "Knife");
        // When
        let attack_motion = context.status_service.attack_motion(&character);
        // Then
        assert_eq!(attack_motion, 996);
    }

    #[test]
    fn test_status_atk() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> { weapon: &'a str, str: u16, dex: u16, luk: u16, expected_status_atk: i32 };
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
            if stat.weapon != "" {
                equip_item(&mut character, stat.weapon);
            }
            // When
            let status_atk = context.status_service.status_atk_left_side(&character);
            // Then
            assert_eq!(status_atk, stat.expected_status_atk, "Expected status atk1 to be {} but was {} with stats {:?}", stat.expected_status_atk, status_atk, stat);
        }
    }
}