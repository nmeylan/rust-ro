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
    use std::fmt::format;
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};
    use std::mem;
    use std::path::Path;
    use std::ptr::eq;
    use models::enums::bonus::BonusType;
    use models::enums::class::JobName;
    use models::enums::weapon::WeaponType;
    use models::status::{Status, StatusSnapshot};
    use models::enums::EnumWithStringValue;
    use models::enums::EnumWithNumberValue;
    use crate::{eq_with_variance, status_snapshot};

    use crate::tests::common::character_helper::{create_character, equip_item_from_id, equip_item_from_name};
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
            let status_snapshot = status_snapshot!(context, character);
            // When
            let status_atk = status_snapshot.atk_left_side();
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
        let fixture_file = "src/tests/common/fixtures/data/stats-for-each-job-level.json";
        let result_file_path = "../doc/progress/stats-for-each-job-level_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each job level", None, |result| format!("JobLvl {}", result.job_level));
    }

    #[test]
    fn test_all_stats_when_equip_items() {
        let context = before_each();
        let fixture_file = "src/tests/common/fixtures/data/stats-for-items.json";
        let result_file_path = "../doc/progress/stats-for-each-items_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each items", None, |result| {
            let mut desc: Vec<String> = vec![];
            if result.status.all_equipped_items().len() == 0 {
                return "Can't find item with id".to_string();
            }
            result.status.all_equipped_items().iter().for_each(|wearable| {
                let item = GlobalConfigService::instance().get_item(wearable.item_id());

                desc.push(format!("**{}**({})", item.name_aegis, item.bonuses.iter().map(|b| format!("<br>\t*{:?}*", b))
                    .collect::<Vec<String>>()
                    .join("")));
            });
            desc.iter().map(|d| format!("{}", d))
                .collect::<Vec<String>>()
                .join("<br>")
        });
        // For each item bonuses run a test
    }

    #[test]
    fn test_each_stats() {
        let context = before_each();
        let fixture_file = "src/tests/common/fixtures/data/stats-for-each-stats.json";
        let result_file_path = "../doc/progress/each-bonus_progress.md";
        stats_tests(fixture_file, result_file_path, "Each item bonus", None, |result| format!("{}", result.desc.as_ref().unwrap_or(&"NA".to_string())));
    }

    #[test]
    fn test_all_stats_when_card() {
    }

    #[test]
    fn test_all_elements_modifier() {
    }
    #[test]
    fn test_all_race_modifier() {
    }
    #[test]
    fn test_all_mob_class_modifier() {
    }
    #[test]
    fn test_all_mob_size_modifier() {
    }
    #[test]
    fn test_all_resistance() {
    }


    #[test]
    fn playground() {
        let id = "ybc2m0";
        let fixture_file = "src/tests/common/fixtures/data/stats-for-items.json";
        let result_file_path = "../doc/progress/stats-for-each-items_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each job level", Some(id), |result| format!("JobLvl {}", result.job_level));
    }

    fn stats_tests<F>(fixture_file: &str, result_file_path: &str, title: &str, test_id: Option<&str>, description_fn : F)
        where F: Fn(&TestResult) -> String {
        // Given
        let context = before_each();
        let _packetver = GlobalConfigService::instance().packetver();
        let scenario = common::fixtures::battle_fixture::BattleFixture::load(fixture_file);
        let mut i = -1;

        let mut results: Vec<TestResult> = Vec::with_capacity(scenario.len());
        // When
        for scenarii in scenario.iter() {
            if let Some(test_id) = test_id {
                if !scenarii.id().eq(test_id) {
                    continue;
                }
            }
            // println!("{}",scenarii.id());
            i += 1;
            let mut character = create_character();
            scenarii.all_equipments().iter().for_each(|e| { equip_item_from_id(&mut character, e.item_id() as u32); });

            let mut character_status = &mut character.status;
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
                actual_str: status_snapshot.base_str() as i16,
                actual_bonus_str: status_snapshot.bonus_str() as i16,
                actual_agi: status_snapshot.base_agi() as i16,
                actual_bonus_agi: status_snapshot.bonus_agi() as i16,
                actual_vit: status_snapshot.base_vit() as i16,
                actual_bonus_vit: status_snapshot.bonus_vit() as i16,
                actual_int: status_snapshot.base_int() as i16,
                actual_bonus_int: status_snapshot.bonus_int() as i16,
                actual_dex: status_snapshot.base_dex() as i16,
                actual_bonus_dex: status_snapshot.bonus_dex() as i16,
                actual_luk: status_snapshot.base_luk() as i16,
                actual_bonus_luk: status_snapshot.bonus_luk() as i16,
                actual_aspd: status_snapshot.aspd() as f32,
                actual_atk_left: status_snapshot.atk_left_side() as u16,
                actual_atk_right: status_snapshot.atk_right_side() as u16,
                actual_matk_min: status_snapshot.matk_min(),
                actual_matk_max: status_snapshot.matk_max(),
                actual_def: status_snapshot.def(),
                actual_mdef: status_snapshot.mdef(),
                actual_hit: status_snapshot.hit(),
                actual_flee: status_snapshot.flee(),
                actual_hp: status_snapshot.max_hp() as u16,
                actual_sp: status_snapshot.max_sp() as u16,
                actual_crit: status_snapshot.crit(),
                expected_str: scenarii.base_str() as i16,
                expected_bonus_str: scenarii.bonus_str(),
                expected_agi: scenarii.base_agi() as i16,
                expected_bonus_agi: scenarii.bonus_agi(),
                expected_vit: scenarii.base_vit() as i16,
                expected_bonus_vit: scenarii.bonus_vit(),
                expected_int: scenarii.base_int() as i16,
                expected_bonus_int: scenarii.bonus_int(),
                expected_dex: scenarii.base_dex() as i16,
                expected_bonus_dex: scenarii.bonus_dex(),
                expected_luk: scenarii.base_luk() as i16,
                expected_bonus_luk: scenarii.bonus_luk(),
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
                status: mem::take(character_status),
                desc: scenarii.desc().clone(),
            };
            let mut passed = false;
            results.push(result);
        }
        if test_id.is_some() {
            return;
        }
        macro_rules! format_result {
            ( $passed:expr, $arg1:expr ) => {
                if $passed {format!("**{}**", format!("{}", $arg1))} else {format!("*{}*", format!("{}", $arg1))}
          };
            ( $passed:expr, $arg1:expr, $arg2:expr  ) => {
                if $passed {format!("**{}**", format!("{}/{}", $arg1, $arg2))} else {format!("*{}*", format!("{}/{}", $arg1, $arg2))}
          };
            ( $passed:expr, $arg1:expr, $arg2:expr , $arg3:expr , $arg4:expr  ) => {
                if $passed {format!("**{}**", format!("{}+{}/{}+{}", $arg1, $arg2, $arg3, $arg4))} else {format!("*{}*", format!("{}+{}/{}+{}", $arg1, $arg2, $arg3, $arg4))}
          };
        }
        let path = Path::new(result_file_path);
        let mut result_file = File::create(path).unwrap();
        result_file.write_all(b"                              \n").unwrap();
        result_file.write_all(format!("fixture file was [{}](/server/{})\n\n", fixture_file, fixture_file).as_bytes()).unwrap();
        result_file.write_all(format!("# {}\n", title).as_bytes()).unwrap();
        result_file.write_all(b"|id|job|context|passed|str|agi|vit|dex|int|luk|aspd|atk left|atk right|matk min|matk max|def|mdef|hit|flee|crit|hp|sp|\n").unwrap();
        result_file.write_all(b"|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|\n").unwrap();
        let mut passed_count = 0;
        for result in results.iter_mut() {
            let str_passed = result.actual_str + result.actual_bonus_str == result.expected_str + result.expected_bonus_str;
            let agi_passed = result.actual_agi + result.actual_bonus_agi == result.expected_agi + result.expected_bonus_agi;
            let vit_passed = result.actual_vit + result.actual_bonus_vit == result.expected_vit + result.expected_bonus_vit;
            let dex_passed = result.actual_dex + result.actual_bonus_dex == result.expected_dex + result.expected_bonus_dex;
            let int_passed = result.actual_int + result.actual_bonus_int == result.expected_int + result.expected_bonus_int;
            let luk_passed = result.actual_luk + result.actual_bonus_luk == result.expected_luk + result.expected_bonus_luk;
            let aspd_passed = result.actual_aspd >= result.expected_aspd - 0.5 || result.actual_aspd <= result.expected_aspd + 0.5;
            let atk_left_passed = result.actual_atk_left == result.expected_atk_left;
            let atk_right_passed = result.actual_atk_right == result.expected_atk_right;
            let def_passed = result.actual_def == result.expected_def;
            let mdef_passed = result.actual_mdef == result.expected_mdef;
            let hit_passed = result.actual_hit == result.expected_hit;
            let matk_min_passed = result.actual_matk_min == result.expected_matk_min;
            let matk_max_passed = result.actual_matk_max == result.expected_matk_max;
            let flee_passed = result.actual_flee == result.expected_flee;
            let crit_passed = result.actual_crit == result.expected_crit;
            let hp_passed = eq_with_variance!(1, result.actual_hp, result.expected_hp);
            let sp_passed = eq_with_variance!(1, result.actual_sp, result.expected_sp);
            result.passed = str_passed && agi_passed && vit_passed && dex_passed && int_passed && luk_passed && aspd_passed && atk_left_passed && atk_right_passed
                && matk_max_passed && matk_min_passed && def_passed && mdef_passed && hit_passed && flee_passed && crit_passed
                && hp_passed && sp_passed;
            if result.passed {
                passed_count += 1;
            }
            result_file.write_all(format!("|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\n",
                                          result.id, result.job, description_fn(result), format_result!(result.passed, result.passed),
                                          format_result!(str_passed, result.actual_str, result.actual_bonus_str, result.expected_str, result.expected_bonus_str),
                                          format_result!(agi_passed, result.actual_agi, result.actual_bonus_agi, result.expected_agi, result.expected_bonus_agi),
                                          format_result!(vit_passed, result.actual_vit, result.actual_bonus_vit, result.expected_vit, result.expected_bonus_vit),
                                          format_result!(dex_passed, result.actual_dex, result.actual_bonus_dex, result.expected_dex, result.expected_bonus_dex),
                                          format_result!(int_passed, result.actual_int, result.actual_bonus_int, result.expected_int, result.expected_bonus_int),
                                          format_result!(luk_passed, result.actual_luk, result.actual_bonus_luk, result.expected_luk, result.expected_bonus_luk),
                                          format_result!(aspd_passed, result.actual_aspd, result.expected_aspd),
                                          format_result!(atk_left_passed, result.actual_atk_left,result.expected_atk_left),
                                          format_result!(atk_right_passed, result.actual_atk_right,result.expected_atk_right),
                                          format_result!(matk_min_passed,result.actual_matk_min, result.expected_matk_min),
                                          format_result!(matk_max_passed,result.actual_matk_max, result.expected_matk_max),
                                          format_result!(def_passed, result.actual_def, result.expected_def),
                                          format_result!(mdef_passed, result.actual_mdef, result.expected_mdef),
                                          format_result!(hit_passed, result.actual_hit, result.expected_hit),
                                          format_result!(flee_passed, result.actual_flee, result.expected_flee),
                                          format_result!(crit_passed, result.actual_crit, result.expected_crit),
                                          format_result!(hp_passed, result.actual_hp, result.expected_hp),
                                          format_result!(sp_passed, result.actual_sp, result.expected_sp),
            ).as_bytes()).unwrap();
        }
        result_file.seek(SeekFrom::Start(0));
        result_file.write_all(format!("{}/{} tests passed\n", passed_count, results.len()).as_bytes()).unwrap();
    }

    #[derive(Clone)]
    struct TestResult {
        id: String,
        job: String,
        job_level: usize,
        passed: bool,
        actual_str: i16,
        actual_bonus_str: i16,
        actual_agi: i16,
        actual_bonus_agi: i16,
        actual_vit: i16,
        actual_bonus_vit: i16,
        actual_int: i16,
        actual_bonus_int: i16,
        actual_dex: i16,
        actual_bonus_dex: i16,
        actual_luk: i16,
        actual_bonus_luk: i16,
        actual_aspd: f32,
        actual_atk_left: u16,
        actual_atk_right: u16,
        actual_matk_min: u16,
        actual_matk_max: u16,
        actual_def: i16,
        actual_mdef: i16,
        actual_hit: i16,
        actual_flee: i16,
        actual_hp: u16,
        actual_sp: u16,
        actual_crit: f32,
        expected_str: i16,
        expected_bonus_str: i16,
        expected_agi: i16,
        expected_bonus_agi: i16,
        expected_vit: i16,
        expected_bonus_vit: i16,
        expected_int: i16,
        expected_bonus_int: i16,
        expected_dex: i16,
        expected_bonus_dex: i16,
        expected_luk: i16,
        expected_bonus_luk: i16,
        expected_aspd: f32,
        expected_atk_left: u16,
        expected_atk_right: u16,
        expected_matk_min: u16,
        expected_matk_max: u16,
        expected_def: i16,
        expected_mdef: i16,
        expected_hit: i16,
        expected_flee: i16,
        expected_hp: u16,
        expected_sp: u16,
        expected_crit: f32,
        status: Status,
        desc: Option<String>
    }
}

