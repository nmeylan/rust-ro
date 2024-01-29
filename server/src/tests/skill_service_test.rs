#![allow(dead_code)]


use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::battle_service::{BattleResultMode, BattleService};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct SkillServiceTestContext {
    test_context: TestContext,
    skill_service: SkillService,
    skill_min_service: SkillService,
    skill_max_service: SkillService,
    status_service: StatusService,
}

fn before_each() -> SkillServiceTestContext {
    before_each_with_latch(0)
}

fn before_each_with_latch(latch_size: usize) -> SkillServiceTestContext {
    common::before_all();
    let (client_notification_sender, client_notification_receiver) = create_mpsc::<Notification>();
    let (persistence_event_sender, persistence_event_receiver) = create_mpsc::<PersistenceEvent>();
    let count_down_latch = CountDownLatch::new(latch_size);
    SkillServiceTestContext {
        test_context: TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        skill_service: SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance(), BattleResultMode::Normal), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance()),
        skill_min_service: SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance(), BattleResultMode::TestMin), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance()),
        skill_max_service: SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance(), BattleResultMode::TestMax), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance()),
        status_service: StatusService::new(GlobalConfigService::instance()),
    }
}


#[cfg(test)]
#[cfg(not(feature = "integration_tests"))]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::time::Duration;
    use enums::class::JobName;

    use enums::{EnumWithNumberValue, EnumWithStringValue};
    use enums::skill::UseSkillFailure;
    use crate::tests::common::assert_helper::*;
    use models::position::Position;
    use enums::skill_enums::SkillEnum;
    use models::status::{KnownSkill, Status};
    use packets::packets::{Packet, PacketZcAckTouseskill, PacketZcActionFailure, PacketZcUseskillAck2};

    use skills::{Skill, SkillBase};
    use crate::{assert_sent_packet_in_current_packetver, status_snapshot, status_snapshot_mob};
    use crate::GlobalConfigService;
    use crate::server::model::map_item::{MapItemSnapshot, ToMapItem, ToMapItemSnapshot};
    use crate::tests::common;
    use crate::tests::common::character_helper::{add_item_in_inventory, create_character, equip_item_from_id, equip_item_from_name, takeoff_weapon};
    use crate::tests::common::mob_helper::create_mob;
    use crate::tests::skill_service_test::before_each;

    #[test]
    fn start_use_skill_should_validate_sp_requirement() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let packetver = GlobalConfigService::instance().packetver();
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        let target = MapItemSnapshot { map_item: mob.to_map_item(), position: Position { x: character.x + 1, y: character.y + 1, dir: 0 } };
        let known_skill = KnownSkill { value: SkillEnum::SmBash, level: 10 };
        character.status.known_skills.push(known_skill);
        character.status.sp = 50;
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcUseskillAck2);
        assert_eq!(packet.skid as u32, known_skill.value.id());
        assert_eq!(packet.aid, character.char_id);
        assert_eq!(packet.target_id, mob_item_id);

        // Given
        character.status.sp = 10;
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcAckTouseskill::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcAckTouseskill::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcAckTouseskill);
        assert_eq!(packet.cause, UseSkillFailure::SpInsufficient.value() as u8);
    }

    #[test]
    fn start_use_skill_should_validate_hp_requirement() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let packetver = GlobalConfigService::instance().packetver();
        let known_skill = KnownSkill { value: SkillEnum::MoBalkyoung, level: 1 };
        character.status.known_skills.push(known_skill);
        character.status.hp = 50;
        character.status.sp = 50;
        let target = character.to_map_item_snapshot();
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(character_status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcUseskillAck2);
        assert_eq!(packet.skid as u32, known_skill.value.id());
        assert_eq!(packet.aid, character.char_id);
        assert_eq!(packet.target_id, character.char_id);

        // Given
        character.status.hp = 10;
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(character_status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcAckTouseskill::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcAckTouseskill::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcAckTouseskill);
        assert_eq!(packet.cause, UseSkillFailure::HpInsufficient.value() as u8);
    }

    #[test]
    fn start_use_skill_should_validate_ammo_requirement() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let packetver = GlobalConfigService::instance().packetver();
        let known_skill = KnownSkill { value: SkillEnum::AcDouble, level: 1 };
        character.status.known_skills.push(known_skill);
        character.status.sp = 50;
        let item_inventory = equip_item_from_name(&mut character, "Arrow");
        equip_item_from_name(&mut character, "Bow");
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        let target = MapItemSnapshot { map_item: mob.to_map_item(), position: Position { x: character.x + 1, y: character.y + 1, dir: 0 } };
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcUseskillAck2);
        assert_eq!(packet.skid as u32, known_skill.value.id());
        assert_eq!(packet.aid, character.char_id);
        assert_eq!(packet.target_id, mob_item_id);

        // Given
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        character.takeoff_equip_item(item_inventory);
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcActionFailure::packet_id(packetver), 1)]));
    }

    #[test]
    fn start_use_skill_should_validate_skill_level_requirement() {}

    #[test]
    fn start_use_skill_should_validate_skill_assignment_requirement() {}

    #[test]
    fn start_use_skill_should_validate_zeny_requirement() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let packetver = GlobalConfigService::instance().packetver();
        let known_skill = KnownSkill { value: SkillEnum::McMammonite, level: 10 };
        character.status.known_skills.push(known_skill);
        character.status.sp = 50;
        character.status.zeny = 5000;
        equip_item_from_name(&mut character, "Axe");
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        let target = MapItemSnapshot { map_item: mob.to_map_item(), position: Position { x: character.x + 1, y: character.y + 1, dir: 0 } };
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcUseskillAck2);
        assert_eq!(packet.skid as u32, known_skill.value.id());
        assert_eq!(packet.aid, character.char_id);
        assert_eq!(packet.target_id, mob_item_id);

        // Given
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        character.status.zeny = 500;
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcAckTouseskill::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcAckTouseskill::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcAckTouseskill);
        assert_eq!(packet.cause, UseSkillFailure::Money.value() as u8);
    }

    #[test]
    fn start_use_skill_should_validate_item_requirement() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let packetver = GlobalConfigService::instance().packetver();
        let known_skill = KnownSkill { value: SkillEnum::MgStonecurse, level: 1 };
        character.status.known_skills.push(known_skill);
        character.status.sp = 500;
        let item_in_inventory_index = add_item_in_inventory(&mut character, "Red_Gemstone");
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        let target = MapItemSnapshot { map_item: mob.to_map_item(), position: Position { x: character.x + 1, y: character.y + 1, dir: 0 } };
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcUseskillAck2);
        assert_eq!(packet.skid as u32, known_skill.value.id());
        assert_eq!(packet.aid, character.char_id);
        assert_eq!(packet.target_id, mob_item_id);

        // Given
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        character.del_item_from_inventory(item_in_inventory_index, 1);
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcAckTouseskill::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcAckTouseskill::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcAckTouseskill);
        assert_eq!(packet.cause, UseSkillFailure::RedGemstone.value() as u8);
    }

    #[test]
    fn start_use_skill_should_validate_target_requirement() {}

    #[test]
    fn start_use_skill_should_validate_weapon_requirement() {
        // Given
        let mut context = before_each();
        let mut character = create_character();
        let packetver = GlobalConfigService::instance().packetver();
        let known_skill = KnownSkill { value: SkillEnum::AcDouble, level: 1 };
        character.status.known_skills.push(known_skill);
        character.status.sp = 50;
        let _arrow_inventory_item_index = equip_item_from_name(&mut character, "Arrow");
        let bow_inventory_item_index = equip_item_from_name(&mut character, "Bow");
        let mob_item_id = 82322;
        let mob = create_mob(mob_item_id, "PORING");
        let target = MapItemSnapshot { map_item: mob.to_map_item(), position: Position { x: character.x + 1, y: character.y + 1, dir: 0 } };
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcUseskillAck2);
        assert_eq!(packet.skid as u32, known_skill.value.id());
        assert_eq!(packet.aid, character.char_id);
        assert_eq!(packet.target_id, mob_item_id);

        // Given
        context.test_context.reset_increment_latch();
        context.test_context.clear_sent_packet();
        character.takeoff_equip_item(bow_inventory_item_index);
        let character_status = status_snapshot!(context, character);
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), character_status, Some(&mob.status), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcAckTouseskill::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcAckTouseskill::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcAckTouseskill);
        assert_eq!(packet.cause, UseSkillFailure::ThisWeapon.value() as u8);
    }

    #[test]
    fn start_use_skill_should_validate_range_requirement() {}

    #[test]
    fn start_use_skill_should_consume_sp_on_success() {}

    #[test]
    fn start_use_skill_should_consume_zeny_on_success() {}

    #[test]
    fn start_use_skill_should_consume_item_on_success() {
        // freezing trap, acid demonstration, stone curse
    }

    #[test]
    fn offensive_skill_calculate_damage() {
        // Given
        let context = before_each();
        let mut character = create_character();
        let _packetver = GlobalConfigService::instance().packetver();
        let fixture_file = "src/tests/common/fixtures/data/battle-all-skills-weapon-no-passives.json";
        let scenario = common::fixtures::battle_fixture::BattleFixture::load(fixture_file);
        let mut i = -1;
        #[derive(Clone)]
        struct TestResult {
            id: String,
            job: String,
            skill: String,
            weapon: String,
            passed: bool,
            comment: String,
            actual_min: Option<u32>,
            actual_max: Option<u32>,
            expected_min: Option<u32>,
            expected_max: Option<u32>,
        }
        let mut results: Vec<TestResult> = Vec::with_capacity(scenario.len());
        // When
        for scenarii in scenario.iter() {
            if scenarii.skill_to_use().skid() == 0 {
                // TODO basic attack
                continue;
            }
            i += 1;
            let mut character_status = Status::default();
            let job = JobName::from_string(scenarii.job().as_str());
            character_status.job = job.value() as u32;
            character_status.str = scenarii.base_str();
            character_status.agi = scenarii.base_agi();
            character_status.vit = scenarii.base_vit();
            character_status.dex = scenarii.base_dex();
            character_status.int = scenarii.base_int();
            character_status.luk = scenarii.base_luk();
            character_status.base_level = scenarii.base_level();
            character_status.job_level = scenarii.job_level();
            character_status.hp = scenarii.max_hp() as u32;
            character_status.sp = scenarii.max_sp() as u32;
            character.status = character_status;
            let item_id = scenarii.equipments().weapon().as_ref().unwrap().item_id();
            if (item_id >= 0) {
                equip_item_from_id(&mut character, item_id as u32);
            } else {
                takeoff_weapon(&mut character);
            }
            if let Some(ammo) = scenarii.ammo() {
                equip_item_from_name(&mut character, ammo.as_str());
            }
            let target = create_mob(1, scenarii.target().to_uppercase().as_str());
            if let Some(skill) = skills::skill_enums::to_object(SkillEnum::from_id(scenarii.skill_to_use().skid()), scenarii.skill_to_use().level()) {
                let skill_config = GlobalConfigService::instance().get_skill_config(skill.id()).clone();
                if let Some(offensive_skill) = skill.as_offensive_skill() {
                    let min = context.skill_min_service.calculate_damage(status_snapshot!(context, character), &target.status, offensive_skill);
                    let max = context.skill_max_service.calculate_damage(status_snapshot!(context, character), &target.status, offensive_skill);

                    let assert_min = scenarii.min_dmg().max(1) - 1 <= min && min <= scenarii.min_dmg() + 1;
                    let assert_max = scenarii.max_dmg().max(1) - 1 <= max && max <= scenarii.max_dmg() + 1;
                    results.push(TestResult{
                        id: scenarii.id().clone(),
                        job: job.as_str().to_string(),
                        skill: skill_config.name,
                        weapon: if item_id >= 0 {GlobalConfigService::instance().get_item(item_id).clone().name_aegis} else {"Unarmed".to_string()},
                        passed: assert_min && assert_max,
                        comment: " ".to_string(),
                        actual_min: Some(min),
                        actual_max: Some(max),
                        expected_min:  Some(scenarii.min_dmg()),
                        expected_max: Some(scenarii.max_dmg()),
                    })
                } else {
                    results.push(TestResult{
                        id: scenarii.id().clone(),
                        job: job.as_str().to_string(),
                        skill: skill_config.name,
                        weapon: if item_id >= 0 {GlobalConfigService::instance().get_item(item_id).clone().name_aegis} else {"Unarmed".to_string()},
                        passed: false,
                        comment:  format!("Skill {} is not an offensive skill", GlobalConfigService::instance().get_skill_config(skill.id()).name),
                        actual_min: None,
                        actual_max: None,
                        expected_min: None,
                        expected_max: None,
                    })
                }
            } else {
                results.push(TestResult{
                    id: scenarii.id().clone(),
                    job: job.as_str().to_string(),
                    skill: GlobalConfigService::instance().get_skill_config(scenarii.skill_to_use().skid()).clone().name,
                    weapon: if item_id >= 0 {GlobalConfigService::instance().get_item(item_id).clone().name_aegis} else {"Unarmed".to_string()},
                    passed: false,
                    comment:  format!("Skill {} was not found", GlobalConfigService::instance().get_skill_config(scenarii.skill_to_use().skid()).name),
                    actual_min: None,
                    actual_max: None,
                    expected_min: None,
                    expected_max: None,
                });
            }
            // assert!(scenarii.min_dmg() - 1 <= min && min <= scenarii.min_dmg() + 1, "Expected min damage to be {} but was {} with skill {} and stats {:?}", scenarii.min_dmg(), min, SkillEnum::from_id(scenarii.skill_to_use().skid()).to_name(), scenarii);
            // assert!(scenarii.max_dmg() - 1 <= max && max <= scenarii.max_dmg() + 1, "Expected max damage to be {} but was {} with skill {} and stats {:?}", scenarii.max_dmg(), max, SkillEnum::from_id(scenarii.skill_to_use().skid()).to_name(), scenarii);
        }
        let path = Path::new("../doc/notes/skills/offensive-skills-progress.md");
        let mut result_file = File::create(path).unwrap();
        result_file.write_all(format!("{}/{} tests passed, fixture file was [{}](/server/{})\n\n", results.iter().filter(|r| r.passed).count(), results.len(), fixture_file, fixture_file).as_bytes()).unwrap();
        let mut result_per_jobs: BTreeMap<String, Vec<TestResult>> = BTreeMap::new();
        for result in results.iter() {
           if !result_per_jobs.contains_key(result.job.as_str()) {
               result_per_jobs.insert(result.job.as_str().to_string(), vec![]);
           }
            let job_result = result_per_jobs.get_mut(result.job.as_str()).unwrap();
            if let Some(existing_result) = job_result.iter_mut().find(|r| r.skill == result.skill) {
                existing_result.passed = existing_result.passed && result.passed;
            } else {
                job_result.push(result.clone());
            }
        }
        for (job, results) in result_per_jobs.iter() {
            result_file.write_all(format!("# {}\n", job).as_bytes()).unwrap();
            result_file.write_all(b"|Skill|Passed|\n").unwrap();
            result_file.write_all(b"|-|-|\n").unwrap();
            for result in results.iter() {
                result_file.write_all(format!("|{}|{}|\n", result.skill, result.passed).as_bytes()).unwrap();
            }
        }

        result_file.write_all(b"# All results\n").unwrap();
        result_file.write_all(b"|Id|Job|Skill|Weapon|Passed|Comment|Min dmg (actual/expected)|Max dmg(actual/expected)|\n").unwrap();
        result_file.write_all(b"|-|-|-|-|-|-|-|-|\n").unwrap();
        for result in results.iter() {
            result_file.write_all(format!("|{}|{}|{}|{}|**{}**|{}|{}/{}|{}/{}|\n", result.id, result.job, result.skill, result.weapon,
                                          if result.passed {"passed"} else {"failed"}, result.comment,
                                          result.actual_min.map(|r| r.to_string()).unwrap_or(String::new()),
                                          result.expected_min.map(|r| r.to_string()).unwrap_or(String::new()),
                                          result.actual_max.map(|r| r.to_string()).unwrap_or(String::new()),
                                          result.expected_max.map(|r| r.to_string()).unwrap_or(String::new()),
            ).as_bytes()).unwrap();
        }
    }
    #[test]
    fn playground() {
        let id = "721s92";
        // Given
        let context = before_each();
        let mut character = create_character();
        let _packetver = GlobalConfigService::instance().packetver();
        let fixture_file = "src/tests/common/fixtures/data/battle-all-skills-weapon-no-passives.json";
        let scenario = common::fixtures::battle_fixture::BattleFixture::load(fixture_file);

        // When
        for scenarii in scenario.iter() {
            if !scenarii.id().eq(id) {
                continue;
            }
            let mut character_status = Status::default();
            let job = JobName::from_string(scenarii.job().as_str());
            character_status.job = job.value() as u32;
            character_status.str = scenarii.base_str();
            character_status.agi = scenarii.base_agi();
            character_status.vit = scenarii.base_vit();
            character_status.dex = scenarii.base_dex();
            character_status.int = scenarii.base_int();
            character_status.luk = scenarii.base_luk();
            character_status.base_level = scenarii.base_level();
            character_status.job_level = scenarii.job_level();
            character_status.hp = scenarii.max_hp() as u32;
            character_status.sp = scenarii.max_sp() as u32;
            character.status = character_status;
            let item_id = scenarii.equipments().weapon().as_ref().unwrap().item_id();
            if (item_id >= 0) {
                equip_item_from_id(&mut character, item_id as u32);
            } else {
                takeoff_weapon(&mut character);
            }
            if let Some(ammo) = scenarii.ammo() {
                equip_item_from_name(&mut character, ammo.as_str());
            }
            let target = create_mob(1, scenarii.target().to_uppercase().as_str());
            let skill = skills::skill_enums::to_object(SkillEnum::from_id(scenarii.skill_to_use().skid()), scenarii.skill_to_use().level()).unwrap();
            let skill_config = GlobalConfigService::instance().get_skill_config(skill.id()).clone();
            let offensive_skill = skill.as_offensive_skill().unwrap();

            let min = context.skill_min_service.calculate_damage(status_snapshot!(context, character), &target.status, offensive_skill);
            let max = context.skill_max_service.calculate_damage(status_snapshot!(context, character), &target.status, offensive_skill);

            let assert_min = scenarii.min_dmg().max(1) - 1 <= min && min <= scenarii.min_dmg() + 1;
            let assert_max = scenarii.max_dmg().max(1) - 1 <= max && max <= scenarii.max_dmg() + 1;
            println!("Passed: {}, min: {}/{}, max: {}/{}", assert_min && assert_max, min, scenarii.min_dmg(), max, scenarii.max_dmg());
        }
    }
}