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
    status_service: &'static StatusService,
}

fn before_each() -> BattleServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> BattleServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    StatusService::init(GlobalConfigService::instance(), "../native_functions_list.txt");
    BattleServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        battle_service: BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::Normal),
        battle_min_service: BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::TestMin),
        battle_max_service: BattleService::new(client_notification_sender.clone(), StatusService::instance(), GlobalConfigService::instance(), BattleResultMode::TestMax),
        status_service: StatusService::instance(),
    }
}

#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::fs::File;
    use std::io::{Seek, SeekFrom, Write};
    use std::mem;
    use std::path::Path;
    use models::enums::class::JobName;
    use models::enums::element::Element;
    use models::enums::{EnumWithStringValue, EnumWithNumberValue};
    use models::enums::size::Size;
    use models::enums::skill_enums::SkillEnum;
    use models::item::{WearGear, WearWeapon};
    use models::status::Status;
    use skills::base::bard_base::MelodyStrike;
    use skills::{OffensiveSkill, Skill};
    use crate::{assert_eq_with_variance, format_result, status_snapshot, status_snapshot_mob};
    use crate::server::model::map_item::{ToMapItemSnapshot};
    use crate::server::service::battle_service::BattleService;
    use crate::server::service::global_config_service::GlobalConfigService;
    use crate::server::service::status_service::StatusService;
    use crate::tests::battle_service_test::{BattleServiceTestContext, before_each};
    use crate::tests::common::character_helper::{create_character, equip_item_from_id_with_cards, equip_item_from_name, equip_item_from_id};
    use crate::tests::common::fixtures::{CombatTestResult, TestResult};
    use crate::tests::common::fixtures::battle_fixture::BattleFixture;
    use crate::tests::common::mob_helper::{create_mob, create_mob_by_id};
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
            min_damage: i32,
            max_damage: i32,
            average_damage: i32,
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
        let character_status = status_snapshot!(context, character);
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
        let character_status = status_snapshot!(context, character);
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
        let character_status = status_snapshot!(context, character);
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
            Scenarii { weapon: Some("Sword"), target_size: Size::Medium, expected_modifier: 1.0 },
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


    #[test]
    fn test_mob_vitdef() {
        // Given
        let context = before_each();
        let _character = create_character();
        for vit in [0, 1, 2, 45, 88].iter() {
            let mut status = Status::default();
            status.vit = *vit;
            // When
            let actual_vit = context.battle_service.mob_vitdef(&context.status_service.to_snapshot(&status));
            // Then
            assert!(actual_vit >= (*vit as f32), "Expected actual_vit {} to be greater or equal to {}", actual_vit, vit);
        }
    }

    #[test]
    fn test_vitdef() {
        // Given
        let context = before_each();
        struct Scenarii<'a> {
            item: &'a str,
            vit: u16,
            expected_vitdef_min: u16,
            expected_vitdef_max: u16,
        }
        let scenario = vec![
            Scenarii { item: "Knife", vit: 99, expected_vitdef_min: 78, expected_vitdef_max: 113 },
            Scenarii { item: "Knife", vit: 49, expected_vitdef_min: 38, expected_vitdef_max: 39 },
        ];
        // When
        for scenarii in scenario {
            let mut character = create_character();
            character.status.vit = scenarii.vit;
            let min = context.battle_min_service.player_vitdef(status_snapshot!(context, character));
            let max = context.battle_max_service.player_vitdef(status_snapshot!(context, character));
            // Then
            assert_eq!(min, scenarii.expected_vitdef_min);
            assert_eq!(max, scenarii.expected_vitdef_max);
        }
    }


    #[test]
    fn test_attack_element() {
        // TODO test endow and crafted weapon
        // Given
        let context = crate::tests::battle_service_test::before_each();
        struct Scenarii<'a> {
            weapon: &'a str,
            ammo: Option<&'a str>,
            expected_element: Element,
            skill: Option<Box<dyn OffensiveSkill>>,
        }
        let scenario = vec![
            Scenarii { weapon: "Sword", ammo: None, expected_element: Element::Neutral, skill: None },
            Scenarii { weapon: "Fire_Brand", ammo: None, expected_element: Element::Fire, skill: None },
            Scenarii { weapon: "Ice_Falchon", ammo: None, expected_element: Element::Water, skill: None },
            Scenarii { weapon: "Katar_Of_Piercing_Wind", ammo: None, expected_element: Element::Wind, skill: None },
            Scenarii { weapon: "Katar_Of_Piercing_Wind", ammo: Some("Fire_Arrow"), expected_element: Element::Wind, skill: None },
            Scenarii { weapon: "Katar_Of_Thornbush", ammo: None, expected_element: Element::Earth, skill: None },
            Scenarii { weapon: "Bow", ammo: Some("Fire_Arrow"), expected_element: Element::Fire, skill: None },
            Scenarii { weapon: "Bow", ammo: Some("Arrow_Of_Wind"), expected_element: Element::Wind, skill: None },
            Scenarii { weapon: "Lute", ammo: Some("Arrow_Of_Wind"), expected_element: Element::Neutral, skill: None },
            Scenarii { weapon: "Lute", ammo: Some("Arrow_Of_Wind"), expected_element: Element::Wind, skill: skills::skill_enums::to_offensive_skill(SkillEnum::BaMusicalstrike, 1) },
        ];
        // When
        for scenari in scenario {
            let mut character = create_character();
            equip_item_from_name(&mut character, scenari.weapon);
            scenari.ammo.map(|ammo| equip_item_from_name(&mut character, ammo));
            // Then
            let element = context.battle_service.attack_element(status_snapshot!(context, character), scenari.skill.as_deref());
            assert_eq!(element, scenari.expected_element, "expected {} but got {} for weapon {} and skill {:?}", scenari.expected_element, element, scenari.weapon, scenari.skill.map(|s| s.id()));
        }
    }

    #[test]
    fn test_attack_element_modifiers() {
        // Given
        let context = crate::tests::battle_service_test::before_each();
        let fixture_file = "src/tests/common/fixtures/data/attack-element-using-arrow.json";
        let result_file_path = "../doc/progress/battle-attack-element-using-arrow_progress.md";
        let scenario = crate::tests::common::fixtures::battle_fixture::BattleFixture::load(fixture_file);

        let test_id: Option<&str> = None;

        battle_test_cases(fixture_file, result_file_path, "Attack element using arrow", false, scenario, test_id, &context.status_service, &context.battle_min_service, &context.battle_max_service)
    }

    fn battle_test_cases(fixture_file: &str, result_file_path: &str, title: &str, assert_passed: bool, scenario: Vec<BattleFixture>, test_id: Option<&str>,
                         status_service: &StatusService, battle_min_service: &BattleService, battle_max_service: &BattleService) {
        let mut i = -1;
        let mut results: Vec<TestResult> = Vec::with_capacity(scenario.len());
        for mut scenarii in scenario {
            if let Some(test_id) = test_id {
                if !scenarii.id().eq(test_id) {
                    continue;
                }
            }
            i += 1;
            let mut character = create_character();
            scenarii.all_equipments().iter().for_each(|e| {
                equip_item_from_id_with_cards(&mut character, e.item_id() as u32, e.cards().iter().map(|c| c.item_id()).collect::<Vec<i16>>());
            });
            scenarii.ammo_id().as_ref().map(|ammo| {equip_item_from_id(&mut character, *ammo)});
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
            let status_snapshot = status_service.to_snapshot(&character_status);
            let target = create_mob_by_id(1, scenarii.target_id());
            let is_ranged = status_snapshot.right_hand_weapon().map(|w| w.weapon_type().is_ranged()).unwrap_or(false);
            let max_dmg = battle_max_service.physical_damage_character_attack_monster(&status_snapshot, &target.status, 0.0, is_ranged);
            let min_dmg = battle_min_service.physical_damage_character_attack_monster(&status_snapshot, &target.status, 0.0, is_ranged);


            let mut result = TestResult {
                id: scenarii.id().clone(),
                job: job.as_str().to_string(),
                job_level: scenarii.job_level() as usize,
                passed: false,
                actual_status: status_snapshot,
                desc: scenarii.desc().clone(),
                expected: mem::take(&mut scenarii),
                status: mem::take(character_status),
                actual_combat_result: Some(CombatTestResult { max_dmg, min_dmg }),
            };
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
        result_file.write_all(b"|id|character|stats|skill|target|passed|min dmg|max dmg|\n").unwrap();
        result_file.write_all(b"|-|-|-|-|-|-|-|-|\n").unwrap();
        let mut passed_count = 0;
        let mut markdown_rows_passed = vec![];
        let mut markdown_rows_failed = vec![];
        for result in results.iter_mut() {
            let mut bonuses_desc = vec![];
            let mut equipped_gears = result.status.equipped_weapons().iter().collect::<Vec<&WearWeapon>>();
            equipped_gears.sort_by(|a, b| a.item_id.cmp(&b.item_id));
            equipped_gears.iter().for_each(|weapon| {
                let item = GlobalConfigService::instance().get_item(weapon.item_id() as i32);
                if weapon.card0() > 0 {
                    let card = GlobalConfigService::instance().get_item(weapon.card0() as i32);
                    let mut bonuses = vec![];
                    status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                    let mut bonuses = vec![];
                    status_service.collect_bonuses(&result.status, &mut bonuses, card);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", card.name_aegis.clone(), bonuses.iter().map(|b| {
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                } else {
                    let mut bonuses = vec![];
                    status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                }
            });
            result.status.equipped_ammo().map(|ammo| {
                bonuses_desc.push(format!("{}", GlobalConfigService::instance().get_item(ammo.item_id).name_aegis));
            });

            let mut equipped_gears = result.status.equipped_gears().iter().collect::<Vec<&WearGear>>();
            equipped_gears.sort_by(|a, b| a.item_id.cmp(&b.item_id));
            equipped_gears.iter().for_each(|equipment| {
                let item = GlobalConfigService::instance().get_item(equipment.item_id() as i32);
                if equipment.card0() > 0 {
                    let card = GlobalConfigService::instance().get_item(equipment.card0() as i32);
                    let mut bonuses = vec![];
                    status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                    let mut bonuses = vec![];
                    status_service.collect_bonuses(&result.status, &mut bonuses, card);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", card.name_aegis.clone(), bonuses.iter().map(|b| {
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                } else {
                    let mut bonuses = vec![];
                    status_service.collect_bonuses(&result.status, &mut bonuses, item);
                    bonuses_desc.push(format!("{}<ul>{}</ul>", item.name_aegis.clone(), bonuses.iter().map(|b| {
                        format!("<li>*{:?}*</li>", b)
                    })
                        .collect::<Vec<String>>()
                        .join("")));
                }
            });
            let bonuses_desc = bonuses_desc.iter().map(|d| format!("<li>{}</li>", d))
                .collect::<Vec<String>>()
                .join("");
            let job = format!("{}({}/{})", result.job, result.status.base_level, result.job_level);
            let min_dmg_passed = result.expected.min_dmg().max(1) - 1 <= result.actual_combat_result.as_ref().unwrap().min_dmg && result.actual_combat_result.as_ref().unwrap().min_dmg <= result.expected.min_dmg() + 1;
            let max_dmg_passed = result.expected.max_dmg().max(1) - 1 <= result.actual_combat_result.as_ref().unwrap().max_dmg && result.actual_combat_result.as_ref().unwrap().max_dmg <= result.expected.max_dmg() + 1;
            result.passed = min_dmg_passed && max_dmg_passed;
            if result.passed {
                passed_count += 1;
            }
            let mut skill = String::from("Basic Attack");
            if result.expected.skill_to_use().skid() > 0 {
                skill = format!("{} (Lv. {})", SkillEnum::from_id(result.expected.skill_to_use().skid()).to_name(),
                                result.expected.skill_to_use().level());
            }
            let text = format!("|{}|{}|{}|{}|{}|{}|{}|{}|\n",
                               result.id, job, format!("<ul>{}</ul>", bonuses_desc),
                               skill, result.expected.target(),
                               format_result!(result.passed, result.passed),
                               format_result!(min_dmg_passed, result.actual_combat_result.as_ref().unwrap().min_dmg, result.expected.min_dmg()),
                               format_result!(max_dmg_passed, result.actual_combat_result.as_ref().unwrap().max_dmg, result.expected.max_dmg()),
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

    // https://irowiki.org/classic/Card_Reference
    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_defensive_resistence_cards() {
        // Wootan Fighter Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_defensive_immunity_cards() {
        // Ungoliant Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_elemental_damage_reduction_cards() {
        // jakk Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_racial_damage_reduction_cards() {
        // thara frog Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_size_damage_reduction_cards() {
        // Mysteltainn Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_mob_group_damage_reduction_cards() {
        // Alice Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_exp_increase_cards() {
        // Am Mut Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_garment_elemental_damage_increase_cards() {
        // Magmaring Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_weapon_elemental_damage_increase() {
        // fireblend
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_ammo_elemental_damage_increase() {
        // fire arrow with bow
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_critical_damage_increase_against_race_cards() {
        // assaulter Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_damage_increase_against_group_cards() {
        // abysmal Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_damage_increase_against_race_cards() {
        // hydra Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_damage_increase_against_element_cards() {
        // vadon Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_damage_increase_against_size_cards() {
        // minorous Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_chance_to_inflict_effect_cards() {
        // zenorc Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_status_armor_inflict_effect_cards() {
        // skogul Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_food_dropping_cards() {
        // anopheles Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_box_dropping_cards() {
        // sleeper Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_increase_skill_damage_cards() {
        // hill wind Card
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_drain_sp_cards() {
        // phendark Card
    }
}