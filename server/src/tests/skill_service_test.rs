#![allow(dead_code)]


use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::battle_service::BattleService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::skill_service::SkillService;
use crate::server::service::status_service::StatusService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct SkillServiceTestContext {
    test_context: TestContext,
    skill_service: SkillService,
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
        skill_service: SkillService::new(client_notification_sender.clone(), persistence_event_sender.clone(), BattleService::new(client_notification_sender.clone(), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance()), StatusService::new(GlobalConfigService::instance()), GlobalConfigService::instance()),
        status_service: StatusService::new(GlobalConfigService::instance()),
    }
}


#[cfg(test)]
mod tests {
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
    use crate::tests::common::character_helper::{add_item_in_inventory, create_character, equip_item_from_name};
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
        let scenario = common::fixtures::battle_fixture::BattleFixture::load("./src/tests/common/fixtures/data/battle_fixtures.json");
        let mut i = -1;
        // When
        for scenarii in scenario.iter() {
            i += 1;
            // if i != 2 { continue; }
            let mut average = Vec::with_capacity(1001);
            let mut min = u32::MAX;
            let mut max = u32::MIN;
            let mut character_status = Status::default();
            let job = JobName::from_string(scenarii.job().as_str());
            character_status.job = job.value() as u32;
            character_status.str = scenarii.str();
            character_status.agi = scenarii.agi();
            character_status.vit = scenarii.vit();
            character_status.dex = scenarii.dex();
            character_status.int = scenarii.int();
            character_status.luk = scenarii.luk();
            character_status.base_level = scenarii.base_level();
            character_status.job_level = scenarii.job_level();
            character_status.hp = 10000;
            character_status.sp = 10000;
            character.status = character_status;
            equip_item_from_name(&mut character, scenarii.weapon().as_ref().unwrap().as_str());
            if let Some(ammo) = scenarii.ammo() {
                equip_item_from_name(&mut character, ammo.as_str());
            }
            let target = create_mob(1, scenarii.target());
            let skill = skills::skill_enums::to_object(SkillEnum::from_id(scenarii.skill_to_use().skid()), scenarii.skill_to_use().level()).unwrap();
            for _ in 0..1000 {
                let damage = context.skill_service.calculate_damage(status_snapshot!(context, character), &target.status, skill.as_offensive_skill().unwrap());
                average.push(damage);
                min = min.min(damage);
                max = max.max(damage);
            }
            let _average = (average.iter().sum::<u32>() as f32 / average.len() as f32).round() as u32;
            // Then
            assert!(scenarii.expected().min_dmg() - 1 <= min && min <= scenarii.expected().min_dmg() + 1, "Expected min damage to be {} but was {} with skill {} and stats {:?}", scenarii.expected().min_dmg(), min, SkillEnum::from_id(scenarii.skill_to_use().skid()).to_name(), scenarii);
            assert!(scenarii.expected().max_dmg() - 1 <= max && max <= scenarii.expected().max_dmg() + 1, "Expected max damage to be {} but was {} with skill {} and stats {:?}", scenarii.expected().max_dmg(), max, SkillEnum::from_id(scenarii.skill_to_use().skid()).to_name(), scenarii);

        }
    }
}