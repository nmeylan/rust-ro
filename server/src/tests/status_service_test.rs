#![allow(dead_code)]

use crate::create_script_vm;
use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::sync_helper::CountDownLatch;
use crate::tests::common::{create_mpsc, TestContext};

pub struct StatusServiceTestContext {
    test_context: TestContext,
    pub status_service: StatusService,
}

pub fn before_each() -> StatusServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> StatusServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    StatusServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        status_service: StatusService::new(GlobalConfigService::instance(), create_script_vm("../native_functions_list.txt")),
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    #[macro_use]
    use crate::format_result;
    use crate::tests::common::fixtures::battle_fixture::Equipment;
    use crate::tests::common::fixtures::TestResult;
    use crate::{eq_with_variance, status_snapshot};
    use models::enums::bonus::BonusType;
    use models::enums::class::JobName;
    use models::enums::element::Element;
    use models::enums::skill_enums::SkillEnum;
    use models::enums::weapon::WeaponType;
    use models::enums::EnumWithNumberValue;
    use models::enums::{EnumWithMaskValueU64, EnumWithStringValue};
    use models::item::{WearGear, WearWeapon};
    use models::status::StatusSnapshot;
    use models::status_bonus::{StatusBonusFlag, TemporaryStatusBonus};
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};
    use std::mem;
    use std::path::Path;

    use super::*;
    use crate::tests::common::character_helper::{create_character, equip_item_from_id_with_cards, equip_item_from_name, equip_item_from_name_with_cards, equip_item_with_cards_and_refinement};
    use crate::tests::common::fixtures::battle_fixture::BattleFixture;

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
    fn test_status_right_side_atk() {
        // Given
        let context = before_each();
        #[derive(Debug)]
        struct Stats<'a> {
            weapon: &'a str,
            refine: u8,
            expected_status_atk: i32,
            expected_overupgrade_bonus: u8,
        }
        let stats = vec![
            Stats { weapon: "Knife", refine: 0, expected_status_atk: 0, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Knife", refine: 5, expected_status_atk: 5 * 2, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Knife", refine: 7, expected_status_atk: 7 * 2, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Knife", refine: 8, expected_status_atk: 8 * 2, expected_overupgrade_bonus: 3 },
            Stats { weapon: "Knife", refine: 9, expected_status_atk: 9 * 2, expected_overupgrade_bonus: 6 },
            Stats { weapon: "Knife", refine: 10, expected_status_atk: 10 * 2, expected_overupgrade_bonus: 9 },
            Stats { weapon: "Dagger", refine: 0, expected_status_atk: 0, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Dagger", refine: 5, expected_status_atk: 5 * 3, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Dagger", refine: 7, expected_status_atk: 7 * 3, expected_overupgrade_bonus: 5 },
            Stats { weapon: "Dagger", refine: 8, expected_status_atk: 8 * 3, expected_overupgrade_bonus: 10 },
            Stats { weapon: "Dagger", refine: 9, expected_status_atk: 9 * 3, expected_overupgrade_bonus: 15 },
            Stats { weapon: "Dagger", refine: 10, expected_status_atk: 10 * 3, expected_overupgrade_bonus: 20 },
            Stats { weapon: "Damascus", refine: 0, expected_status_atk: 0, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Damascus", refine: 5, expected_status_atk: 5 * 5, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Damascus", refine: 6, expected_status_atk: 6 * 5, expected_overupgrade_bonus: 8 },
            Stats { weapon: "Damascus", refine: 7, expected_status_atk: 7 * 5, expected_overupgrade_bonus: 16 },
            Stats { weapon: "Damascus", refine: 8, expected_status_atk: 8 * 5, expected_overupgrade_bonus: 24 },
            Stats { weapon: "Damascus", refine: 9, expected_status_atk: 9 * 5, expected_overupgrade_bonus: 32 },
            Stats { weapon: "Damascus", refine: 10, expected_status_atk: 10 * 5, expected_overupgrade_bonus: 40 },
            Stats { weapon: "Combat_Knife", refine: 0, expected_status_atk: 0, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Combat_Knife", refine: 4, expected_status_atk: 4 * 7, expected_overupgrade_bonus: 0 },
            Stats { weapon: "Combat_Knife", refine: 5, expected_status_atk: 5 * 7, expected_overupgrade_bonus: 14 },
            Stats { weapon: "Combat_Knife", refine: 6, expected_status_atk: 6 * 7, expected_overupgrade_bonus: 28 },
            Stats { weapon: "Combat_Knife", refine: 7, expected_status_atk: 7 * 7, expected_overupgrade_bonus: 42 },
            Stats { weapon: "Combat_Knife", refine: 8, expected_status_atk: 8 * 7, expected_overupgrade_bonus: 56 },
            Stats { weapon: "Combat_Knife", refine: 9, expected_status_atk: 9 * 7, expected_overupgrade_bonus: 70 },
            Stats { weapon: "Combat_Knife", refine: 10, expected_status_atk: 10 * 7, expected_overupgrade_bonus: 84 },
        ];

        for stat in stats {
            let mut character = create_character();
            equip_item_with_cards_and_refinement(&mut character, GlobalConfigService::instance().get_item_by_name(stat.weapon), vec![], stat.refine);
            // When
            let status_snapshot = status_snapshot!(context, character);
            // Then
            assert_eq!(status_snapshot.atk_right_side(), stat.expected_status_atk, "Expected status atk right to be {} but was {} with stats {:?}", stat.expected_status_atk, status_snapshot.atk_right_side(), stat);
            assert_eq!(status_snapshot.overupgrade_right_hand_atk_bonus(), stat.expected_overupgrade_bonus, "Expected status atk right to be {} but was {} with stats {:?}", stat.expected_overupgrade_bonus, status_snapshot.overupgrade_right_hand_atk_bonus(), stat);
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
    fn test_temporary_bonuses_are_included() {
        // Given
        let context = before_each();
        let mut character = create_character();
        // When
        character.status.temporary_bonuses.add(TemporaryStatusBonus::with_duration(BonusType::Agi(10), StatusBonusFlag::Default.as_flag(), 0, 10000, SkillEnum::AlIncagi.id() as u16));
        character.status.temporary_bonuses.add(TemporaryStatusBonus::with_duration(BonusType::SpeedPercentage(25), StatusBonusFlag::Default.as_flag(), 0, 10000, SkillEnum::AlIncagi.id() as u16));
        let status_snapshot = status_snapshot!(context, character);
        // Then
        assert_eq!(status_snapshot.bonus_agi(), 10);
        assert_eq!(status_snapshot.speed(), 112);
    }

    #[test]
    fn test_cast_time_reduction() {
        // Given
        let context = before_each();
        struct Scenarii {
            dex: u16,
            bonuses: Vec<BonusType>,
            base_cast_time: u16,
            expected_cast_time: u16,
        }
        let scenario = vec![
            Scenarii { dex: 1, bonuses: vec![], base_cast_time: 1000, expected_cast_time: 994, },
            Scenarii { dex: 150, bonuses: vec![], base_cast_time: 1000, expected_cast_time: 0, },
            Scenarii { dex: 99, bonuses: vec![], base_cast_time: 1000, expected_cast_time: 340, },
            Scenarii { dex: 99, bonuses: vec![BonusType::CastTimePercentage(-15)], base_cast_time: 1000, expected_cast_time: 289, },
        ];
        // When
        for scenarii in scenario {
            let mut character = create_character();
            character.status.dex = scenarii.dex;
            for bonus in scenarii.bonuses {
                character.status.temporary_bonuses.add(TemporaryStatusBonus::with_duration(bonus, StatusBonusFlag::Default.as_flag(), 0, 10000, SkillEnum::PrSuffragium.id() as u16));
            }
            let status_snapshot = status_snapshot!(context, character);
            assert_eq!((context.status_service.cast_time_reduction(status_snapshot) * scenarii.base_cast_time as f32).ceil() as u16, scenarii.expected_cast_time)
        }
        // Then
    }

    #[test]
    fn test_all_stats_when_job_level_change() {
        let fixture_file = "src/tests/common/fixtures/data/stats-for-each-job-level.json";
        let result_file_path = "../doc/progress/stats-for-each-job-level_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each job level", None, false);
    }

    #[test]
    fn test_all_stats_when_equip_items() {
        let fixture_file = "src/tests/common/fixtures/data/stats-for-items.json";
        let result_file_path = "../doc/progress/stats-for-each-items_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each items", None, false);
    }

    #[test]
    fn test_each_stats() {
        let fixture_file = "src/tests/common/fixtures/data/stats-for-each-stats.json";
        let result_file_path = "../doc/progress/each-bonus_progress.md";
        stats_tests(fixture_file, result_file_path, "Each item bonus", None, false);
    }

    #[test]
    fn test_all_stats_when_card() {
        let fixture_file = "src/tests/common/fixtures/data/stats-for-cards.json";
        let result_file_path = "../doc/progress/stats-for-each-card_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each cards", None, false);
    }


    #[test]
    fn playground() {
        let id = "0g3rud";
        let fixture_file = "src/tests/common/fixtures/data/stats-for-cards.json";
        let result_file_path = "../doc/progress/stats-for-each-items_progress.md";
        stats_tests(fixture_file, result_file_path, "Stats for each job level", Some(id), false);
    }


    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_increase_base_on_high_upgrade_cards() {
        // apocalypse Card
        // dimik Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_increase_base_on_low_upgrade_cards() {
        // gold acidus Card
        // gibbet
    }

    #[test]
    fn test_status_elemental_armor_cards() {
        // Given
        let context = before_each();

        struct Scenarii<'a> {
            card: &'a str,
            expected_element: Element,
        }
        ;
        let scenario = vec![
            Scenarii { card: "Ghostring_Card", expected_element: Element::Ghost },
            Scenarii { card: "Dokebi_Card", expected_element: Element::Wind },
            Scenarii { card: "Sand_Man_Card", expected_element: Element::Earth },
            Scenarii { card: "Angeling_Card", expected_element: Element::Holy },
            Scenarii { card: "Sword_Fish_Card", expected_element: Element::Water },
            Scenarii { card: "Pasana_Card", expected_element: Element::Fire },
            Scenarii { card: "Argiope_Card", expected_element: Element::Poison },
            Scenarii { card: "Bathory_Card", expected_element: Element::Dark },
            Scenarii { card: "Evil_Druid_Card", expected_element: Element::Undead },
        ];
        // When
        for scenarii in scenario {
            let mut character = create_character();
            equip_item_from_name_with_cards(&mut character, "Coat_", vec![GlobalConfigService::instance().get_item_id_from_name(scenarii.card) as i16]);
            let status_snapshot = status_snapshot!(context, character);
            // Then
            assert_eq!(*status_snapshot.element(), scenarii.expected_element);
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_stat_increase_base_on_another_stat_cards() {
        // venatu Card
    }

    // #[test]
    #[bench]
    // fn fullstuff_bench() {
    fn fullstuff_bench(bencher: &mut test::Bencher) {
        let id = "ve5tmv";
        let context = before_each();
        let fixture_file = "src/tests/common/fixtures/data/fullstuff.json";
        let scenario = BattleFixture::load(fixture_file);
        let scenarii = scenario.iter().find(|s| s.id() == id).unwrap();
        let mut character = create_character();
        scenarii.all_equipments().iter().for_each(|e| {
            equip_item_from_id_with_cards(&mut character, e.item_id() as u32, e.cards().iter().map(|c| c.item_id()).collect::<Vec<i16>>());
        });
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

        bencher.iter(|| {
            context.status_service.to_snapshot(&character_status);
        });
    }


    pub(crate) fn stats_tests(fixture_file: &str, result_file_path: &str, title: &str, test_id: Option<&str>, assert_passed: bool) {
        // Given
        let context = before_each();
        let _packetver = GlobalConfigService::instance().packetver();
        let scenario = crate::tests::common::fixtures::battle_fixture::BattleFixture::load(fixture_file);
        let mut i = -1;

        let mut results: Vec<TestResult> = Vec::with_capacity(scenario.len());
        // When
        for mut scenarii in scenario {
            if let Some(test_id) = test_id {
                if !scenarii.id().eq(test_id) {
                    continue;
                }
            }
            // println!("{}",scenarii.id());
            i += 1;
            let mut character = create_character();
            scenarii.all_equipments().iter().for_each(|e| {
                equip_item_from_id_with_cards(&mut character, e.item_id() as u32, e.cards().iter().map(|c| c.item_id()).collect::<Vec<i16>>());
            });

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
                actual_status: status_snapshot,
                desc: scenarii.desc().clone(),
                expected: mem::take(&mut scenarii),
                status: mem::take(character_status),
                actual_combat_result: None,
            };
            let mut passed = false;
            results.push(result);
        }
        if test_id.is_some() {
            return;
        }

        let path = Path::new(result_file_path);
        let mut result_file = File::create(path).unwrap();
        result_file.write_all(b"                              \n").unwrap();
        result_file.write_all(format!("fixture file was [{}](/server/{})\n\n", fixture_file, fixture_file).as_bytes()).unwrap();
        result_file.write_all(format!("# {}\n", title).as_bytes()).unwrap();
        result_file.write_all(b"|id|character|stats computed|stats from fixtures|passed|str|agi|vit|dex|int|luk|aspd|atk left|atk right|matk min|matk max|def|mdef|hit|flee|crit|hp|sp|Armor element|\n").unwrap();
        result_file.write_all(b"|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|-|\n").unwrap();
        let mut passed_count = 0;
        let mut markdown_rows_passed = vec![];
        let mut markdown_rows_failed = vec![];
        for result in results.iter_mut() {
            let mut bonuses_desc = vec![];
            let mut fixtures_bonuses_desc = vec![];
            let mut actual_bonuses = vec![];
            let mut expected_bonuses = vec![];

            let mut equipped_gears = result.status.equipped_weapons().iter().collect::<Vec<&WearWeapon>>();
            equipped_gears.sort_by(|a, b| a.item_id.cmp(&b.item_id));
            equipped_gears.iter().for_each(|weapon| {
                let item = GlobalConfigService::instance().get_item(weapon.item_id() as i32);
                if weapon.card0() > 0 {
                    let card = GlobalConfigService::instance().get_item(weapon.card0() as i32);
                    let mut bonuses = vec![];
                    context.status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        actual_bonuses.push(b.clone());
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                    let mut bonuses = vec![];
                    context.status_service.collect_bonuses(&result.status, &mut bonuses, card);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", card.name_aegis.clone(), bonuses.iter().map(|b| {
                        actual_bonuses.push(b.clone());
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                } else {
                    let mut bonuses = vec![];
                    context.status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        actual_bonuses.push(b.clone());
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                }
            });
            let mut equipped_gears = result.status.equipped_gears().iter().collect::<Vec<&WearGear>>();
            equipped_gears.sort_by(|a, b| a.item_id.cmp(&b.item_id));
            equipped_gears.iter().for_each(|equipment| {
                let item = GlobalConfigService::instance().get_item(equipment.item_id() as i32);
                if equipment.card0() > 0 {
                    let card = GlobalConfigService::instance().get_item(equipment.card0() as i32);
                    let mut bonuses = vec![];
                    context.status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        actual_bonuses.push(b.clone());
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                    let mut bonuses = vec![];
                    context.status_service.collect_bonuses(&result.status, &mut bonuses, card);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", card.name_aegis.clone(), bonuses.iter().map(|b| {
                        actual_bonuses.push(b.clone());
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                } else {
                    let mut bonuses = vec![];
                    context.status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        actual_bonuses.push(b.clone());
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                }
            });


            let mut fixture_equipments = result.expected.all_equipments().iter().cloned().collect::<Vec<&Equipment>>();
            fixture_equipments.sort_by(|a, b| a.item_id().cmp(&b.item_id()));
            fixture_equipments.iter().for_each(|e| {
                fixtures_bonuses_desc.push(format!("{}<ul>{}</ul>", e.name(), e.bonuses().iter().map(|b| {
                    expected_bonuses.push(b.0.clone());
                    format!("<li>*{:?}*</li>", b.0)
                })
                    .collect::<Vec<String>>()
                    .join("")));
                e.cards().iter().for_each(|c| {
                    fixtures_bonuses_desc.push(format!("{}<ul>{}</ul>", c.name(), c.bonuses().iter().map(|b| {
                        expected_bonuses.push(b.0.clone());
                        format!("<li>*{:?}*</li>", b.0)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                })
            });
            let bonuses_desc = bonuses_desc.iter().map(|d| format!("<li>{}</li>", d))
                .collect::<Vec<String>>()
                .join("");
            let fixtures_bonuses_desc = fixtures_bonuses_desc.iter().map(|d| format!("<li>{}</li>", d))
                .collect::<Vec<String>>()
                .join("");
            let job = format!("{}({}/{})", result.job, result.status.base_level, result.job_level);

            let str_passed = result.actual_status.str() as i16 == result.expected.bonus_str() + result.expected.base_str() as i16;
            let agi_passed = result.actual_status.agi() as i16 == result.expected.bonus_agi() + result.expected.base_agi() as i16;
            let vit_passed = result.actual_status.vit() as i16 == result.expected.bonus_vit() + result.expected.base_vit() as i16;
            let dex_passed = result.actual_status.dex() as i16 == result.expected.bonus_dex() + result.expected.base_dex() as i16;
            let int_passed = result.actual_status.int() as i16 == result.expected.bonus_int() + result.expected.base_int() as i16;
            let luk_passed = result.actual_status.luk() as i16 == result.expected.bonus_luk() + result.expected.base_luk() as i16;
            let aspd_passed = result.actual_status.aspd() >= result.expected.aspd() - 0.5 || result.actual_status.aspd() <= result.expected.aspd() + 0.5;
            let atk_left_passed = result.actual_status.atk_left_side() as u16 == result.expected.atk_left();
            let atk_right_passed = result.actual_status.atk_right_side() as u16 == result.expected.atk_right();
            let def_passed = result.actual_status.def() == result.expected.def();
            let mdef_passed = result.actual_status.mdef() == result.expected.mdef();
            let hit_passed = result.actual_status.hit() == result.expected.hit();
            let matk_min_passed = result.actual_status.matk_min() == result.expected.matk_min();
            let matk_max_passed = result.actual_status.matk_max() == result.expected.matk_max();
            let flee_passed = result.actual_status.flee() == result.expected.flee();
            let crit_passed = result.actual_status.crit() == result.expected.crit();
            let hp_passed = eq_with_variance!(1, result.actual_status.max_hp(), result.expected.max_hp());
            let sp_passed = eq_with_variance!(1, result.actual_status.max_sp(), result.expected.max_sp());
            let armor_element_passed = result.actual_status.element() == result.expected.element();

            let mut found_count = 0;
            for bonus in expected_bonuses.iter() {
                if let Some(index) = actual_bonuses.iter().position(|actual_bonus| actual_bonus.clone() == bonus.clone()) {
                    found_count += 1;
                    actual_bonuses.swap_remove(index);
                }
            }
            let stat_passed = found_count == expected_bonuses.len();

            result.passed = str_passed && agi_passed && vit_passed && dex_passed && int_passed && luk_passed && aspd_passed && atk_left_passed && atk_right_passed
                && matk_max_passed && matk_min_passed && def_passed && mdef_passed && hit_passed && flee_passed && crit_passed
                && hp_passed && sp_passed && armor_element_passed && stat_passed;

            if result.passed {
                passed_count += 1;
            }

            let text = format!("|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|\n",
                               result.id, job, format_result!(stat_passed, format!("<ul>{}</ul>",bonuses_desc)), format_result!(stat_passed, format!("<ul>{}</ul>",fixtures_bonuses_desc)), format_result!(result.passed, result.passed),
                               format_result!(str_passed, result.actual_status.base_str(), result.actual_status.bonus_str(), result.expected.base_str(), result.expected.bonus_str()),
                               format_result!(agi_passed, result.actual_status.base_agi(), result.actual_status.bonus_agi(), result.expected.base_agi(), result.expected.bonus_agi()),
                               format_result!(vit_passed, result.actual_status.base_vit(), result.actual_status.bonus_vit(), result.expected.base_vit(), result.expected.bonus_vit()),
                               format_result!(dex_passed, result.actual_status.base_dex(), result.actual_status.bonus_dex(), result.expected.base_dex(), result.expected.bonus_dex()),
                               format_result!(int_passed, result.actual_status.base_int(), result.actual_status.bonus_int(), result.expected.base_int(), result.expected.bonus_int()),
                               format_result!(luk_passed, result.actual_status.base_luk(), result.actual_status.bonus_luk(), result.expected.base_luk(), result.expected.bonus_luk()),
                               format_result!(aspd_passed, result.actual_status.aspd(), result.expected.aspd_displayed()),
                               format_result!(atk_left_passed, result.actual_status.atk_left_side(),result.expected.atk_left()),
                               format_result!(atk_right_passed, result.actual_status.atk_right_side(),result.expected.atk_right()),
                               format_result!(matk_min_passed,result.actual_status.matk_min(), result.expected.matk_min()),
                               format_result!(matk_max_passed,result.actual_status.matk_max(), result.expected.matk_max()),
                               format_result!(def_passed, result.actual_status.def(), result.expected.def()),
                               format_result!(mdef_passed, result.actual_status.mdef(), result.expected.mdef()),
                               format_result!(hit_passed, result.actual_status.hit(), result.expected.hit()),
                               format_result!(flee_passed, result.actual_status.flee(), result.expected.flee()),
                               format_result!(crit_passed, result.actual_status.crit(), result.expected.crit()),
                               format_result!(hp_passed, result.actual_status.max_hp(), result.expected.max_hp()),
                               format_result!(sp_passed, result.actual_status.max_sp(), result.expected.max_sp()),
                               format_result!(armor_element_passed, result.actual_status.element(), result.expected.element()),
            );
            if result.passed {
                markdown_rows_passed.push(text);
            } else {
                markdown_rows_failed.push(text);
            }
        }
        markdown_rows_failed.iter().for_each(|r| { result_file.write(r.as_bytes()).unwrap(); });
        markdown_rows_passed.iter().for_each(|r| { result_file.write(r.as_bytes()).unwrap(); });
        result_file.seek(SeekFrom::Start(0));
        result_file.write_all(format!("{}/{} tests passed\n", passed_count, results.len()).as_bytes()).unwrap();
        if assert_passed {
            assert_eq!(markdown_rows_failed.len(), 0);
        }
    }
}

