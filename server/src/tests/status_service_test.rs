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
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        status_service: StatusService::new(GlobalConfigService::instance()),
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use models::enums::class::JobName;
    use models::enums::weapon::WeaponType;
    use models::status::{Status, StatusSnapshot};
    use models::enums::EnumWithStringValue;
    use models::enums::EnumWithNumberValue;
    use crate::status_snapshot;

    use crate::tests::common::character_helper::{create_character, equip_item_from_name};
    use super::*;

    #[test]
    fn test_right_hand_weapon_type_is_returning_the_right_hand_weapon_when_character_has_one() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _inventory_index = equip_item_from_name(&mut character, "Knife");
        let status: &StatusSnapshot = status_snapshot!(context, character);
        // When
        let weapon_type = status.right_hand_weapon_type();
        // Then
        assert_eq!(*weapon_type, WeaponType::Dagger);
    }

    #[test]
    fn test_aspd() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> {
            weapon: &'a str,
            agi: u16,
            dex: u16,
            job: &'a str,
            expected_aspd: u16,
        }
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
            let status: &StatusSnapshot = status_snapshot!(context, character);
            // When
            let aspd = status.aspd().round() as u16;
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
        let status: &StatusSnapshot = status_snapshot!(context, character);
        // When
        let aspd = status.aspd();
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
        let status: &StatusSnapshot = status_snapshot!(context, character);
        // When
        let attack_motion = context.status_service.attack_motion(status);
        // Then
        assert_eq!(attack_motion, 1296);
    }

    #[test]
    fn test_status_left_side_atk() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> {
            weapon: &'a str,
            str: u16,
            dex: u16,
            luk: u16,
            expected_status_atk: i32,
        }
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
            let status_atk = context.status_service.status_atk_left_side(status_snapshot!(context, character));
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

    #[test]
    fn test_all_stats_when_job_level_change() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _packetver = GlobalConfigService::instance().packetver();
        let fixture_file = "src/tests/common/fixtures/data/stats-for-each-job-level.json";
        let scenario = common::fixtures::battle_fixture::BattleFixture::load(fixture_file);
        let mut i = -1;
        #[derive(Clone)]
        struct TestResult {
            id: String,
            job: String,
            job_level: usize,
            passed: bool,
            actual_str: u16,
            actual_bonus_str: u16,
            actual_agi: u16,
            actual_bonus_agi: u16,
            actual_vit: u16,
            actual_bonus_vit: u16,
            actual_int: u16,
            actual_bonus_int: u16,
            actual_dex: u16,
            actual_bonus_dex: u16,
            actual_luk: u16,
            actual_bonus_luk: u16,
            actual_aspd: f32,
            actual_atk_left: u16,
            actual_atk_right: u16,
            actual_matk_min: u16,
            actual_matk_max: u16,
            actual_def: u16,
            actual_mdef: u16,
            actual_hit: u16,
            actual_flee: u16,
            actual_hp: u16,
            actual_sp: u16,
            actual_crit: f32,
            expected_str: u16,
            expected_bonus_str: u16,
            expected_agi: u16,
            expected_bonus_agi: u16,
            expected_vit: u16,
            expected_bonus_vit: u16,
            expected_int: u16,
            expected_bonus_int: u16,
            expected_dex: u16,
            expected_bonus_dex: u16,
            expected_luk: u16,
            expected_bonus_luk: u16,
            expected_aspd: f32,
            expected_atk_left: u16,
            expected_atk_right: u16,
            expected_matk_min: u16,
            expected_matk_max: u16,
            expected_def: u16,
            expected_mdef: u16,
            expected_hit: u16,
            expected_flee: u16,
            expected_hp: u16,
            expected_sp: u16,
            expected_crit: f32,
        }
        let mut results: Vec<TestResult> = Vec::with_capacity(scenario.len());
        // When
        for scenarii in scenario.iter() {
            i += 1;
            let mut character_status = Status::default();
            let job = JobName::from_string(scenarii.job().as_str());
            character_status.job = job.value() as u32;
            character_status.job_level = scenarii.job_level();
            character_status.str = scenarii.base_str();
            character_status.agi = scenarii.base_agi();
            character_status.vit = scenarii.base_vit();
            character_status.dex = scenarii.base_dex();
            character_status.int = scenarii.base_int();
            character_status.luk = scenarii.base_luk();
            character_status.base_level = scenarii.base_level();
            let status_snapshot = context.status_service.to_snapshot(&character_status);
            let result = TestResult {
                id: scenarii.id().clone(),
                job: job.as_str().to_string(),
                job_level: scenarii.job_level() as usize,
                passed: false,
                actual_str: status_snapshot.base_str(),
                actual_bonus_str: status_snapshot.bonus_str(),
                actual_agi: status_snapshot.base_agi(),
                actual_bonus_agi: status_snapshot.bonus_agi(),
                actual_vit: status_snapshot.base_vit(),
                actual_bonus_vit: status_snapshot.bonus_vit(),
                actual_int: status_snapshot.base_int(),
                actual_bonus_int: status_snapshot.bonus_int(),
                actual_dex: status_snapshot.base_dex(),
                actual_bonus_dex: status_snapshot.bonus_dex(),
                actual_luk: status_snapshot.base_luk(),
                actual_bonus_luk: status_snapshot.bonus_luk(),
                actual_aspd: status_snapshot.aspd() as f32,
                actual_atk_left: context.status_service.status_atk_left_side(&status_snapshot) as u16,
                actual_atk_right: context.status_service.status_atk_right_side(&status_snapshot) as u16,
                actual_matk_min: status_snapshot.matk_min(),
                actual_matk_max: status_snapshot.matk_max(),
                actual_def: status_snapshot.def(),
                actual_mdef: status_snapshot.mdef(),
                actual_hit: status_snapshot.hit(),
                actual_flee: status_snapshot.flee(),
                actual_hp: status_snapshot.max_hp() as u16,
                actual_sp: status_snapshot.max_sp() as u16,
                actual_crit: status_snapshot.crit(),
                expected_str: scenarii.base_str(),
                expected_bonus_str: scenarii.bonus_str() as u16,
                expected_agi: scenarii.base_agi(),
                expected_bonus_agi: scenarii.bonus_agi() as u16,
                expected_vit: scenarii.base_vit(),
                expected_bonus_vit: scenarii.bonus_vit() as u16,
                expected_int: scenarii.base_int(),
                expected_bonus_int: scenarii.bonus_int() as u16,
                expected_dex: scenarii.base_dex(),
                expected_bonus_dex: scenarii.bonus_dex() as u16,
                expected_luk: scenarii.base_luk(),
                expected_bonus_luk: scenarii.bonus_luk() as u16,
                expected_aspd: scenarii.aspd_displayed(),
                expected_atk_left: scenarii.atk_left(),
                expected_atk_right: scenarii.atk_right(),
                expected_matk_min: scenarii.matk_min(),
                expected_matk_max: scenarii.matk_max(),
                expected_def: scenarii.def(),
                expected_mdef: scenarii.mdef(),
                expected_hit: scenarii.hit(),
                expected_flee: scenarii.flee(),
                expected_hp: scenarii.max_hp(),
                expected_sp: scenarii.max_sp(),
                expected_crit: scenarii.crit(),
            };
            let mut passed = false;
            results.push(result);

        }
        let path = Path::new("../doc/notes/stats/stats-for-each-job-level.md");
        let mut result_file = File::create(path).unwrap();
        result_file.write_all(format!("{}/{} tests passed, fixture file was [{}](/server/{})\n\n", results.iter().filter(|r| r.passed).count(), results.len(), fixture_file, fixture_file).as_bytes()).unwrap();
        result_file.write_all(b"# All results\n").unwrap();
        result_file.write_all(b"|id|job|jobLv|passed|str|agi|vit|dex|int|luk|aspd|atk left|atk right|matk min|matk max|def|mdef|hit|flee|crit|hp|sp|\n").unwrap();
        result_file.write_all(b"|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|\n").unwrap();
        for result in results.iter() {
            result_file.write_all(format!("|{}|{}|{}|**{}**|{}+{}/{}+{}|{}+{}/{}+{}|{}+{}/{}+{}|{}+{}/{}+{}|{}+{}/{}+{}|{}+{}/{}+{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|{}/{}|\n",
                                          result.id, result.job, result.job_level,result.passed,
                                          result.actual_str, result.actual_bonus_str, result.expected_str, result.expected_bonus_str,
                                          result.actual_agi, result.actual_bonus_agi, result.expected_agi, result.expected_bonus_agi,
                                          result.actual_vit, result.actual_bonus_vit, result.expected_vit, result.expected_bonus_vit,
                                          result.actual_dex, result.actual_bonus_dex, result.expected_dex, result.expected_bonus_dex,
                                          result.actual_int, result.actual_bonus_int, result.expected_int, result.expected_bonus_int,
                                          result.actual_luk, result.actual_bonus_luk, result.expected_luk, result.expected_bonus_luk,
                                          result.actual_aspd, result.expected_aspd,
                                          result.actual_atk_left,result.expected_atk_left,
                                          result.actual_atk_right,result.expected_atk_right,
                                          result.actual_matk_min, result.expected_matk_min,
                                          result.actual_matk_max, result.expected_matk_max,
                                          result.actual_def, result.expected_def,
                                          result.actual_mdef, result.expected_mdef,
                                          result.actual_hit, result.expected_hit,
                                          result.actual_flee, result.expected_flee,
                                          result.actual_crit, result.expected_crit,
                                          result.actual_hp, result.expected_hp,
                                          result.actual_sp, result.expected_sp,
            ).as_bytes()).unwrap();
        }
    }
}