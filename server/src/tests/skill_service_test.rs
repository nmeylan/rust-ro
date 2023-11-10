#![allow(dead_code)]


use crate::server::model::events::client_notification::Notification;
use crate::server::model::events::persistence_event::PersistenceEvent;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::skill_service::SkillService;
use crate::tests::common;
use crate::tests::common::{create_mpsc, TestContext};
use crate::tests::common::sync_helper::CountDownLatch;

struct SkillServiceTestContext {
    test_context: TestContext,
    skill_service: SkillService,
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
        test_context:TestContext::new(client_notification_sender.clone(), client_notification_receiver, persistence_event_sender.clone(), persistence_event_receiver, count_down_latch),
        skill_service: SkillService::new(client_notification_sender, persistence_event_sender, GlobalConfigService::instance()),
    }
}



#[cfg(test)]
mod tests {
    use std::time::Duration;
    use enums::EnumWithNumberValue;
    use enums::skill::UseSkillFailure;
    use crate::tests::common::assert_helper::*;
    use models::position::Position;
    use skills::skill_enums::SkillEnum;
    use packets::packets::{Packet, PacketZcAckTouseskill, PacketZcActionFailure, PacketZcUseskillAck2};
    use crate::{assert_sent_packet_in_current_packetver};
    use crate::GlobalConfigService;
    use crate::server::model::map_item::{MapItemSnapshot, ToMapItem, ToMapItemSnapshot};
    use crate::server::state::skill::KnownSkill;
    use crate::tests::common::character_helper::{add_item_in_inventory, create_character};
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
        let map_item_snapshot = MapItemSnapshot { map_item: mob.to_map_item(), position: Position { x: character.x + 1, y: character.y + 1, dir: 0 } };
        let known_skill = KnownSkill { value: SkillEnum::SmBash, level: 10 };
        character.known_skills.push(known_skill);
        character.status.sp = 50;
        // When
        context.skill_service.start_use_skill(&mut character, Some(map_item_snapshot), known_skill.value.id(), known_skill.level, 0);
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
        // When
        context.skill_service.start_use_skill(&mut character, Some(map_item_snapshot), known_skill.value.id(), known_skill.level, 0);
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
        character.known_skills.push(known_skill);
        character.status.hp = 50;
        character.status.sp = 50;
        let target = character.to_map_item_snapshot();
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), known_skill.value.id(), known_skill.level, 0);
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
        // When
        context.skill_service.start_use_skill(&mut character, Some(target), known_skill.value.id(), known_skill.level, 0);
        // Then
        context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
        assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcAckTouseskill::packet_id(packetver), 1)]));
        let packets = context.test_context.get_sent_packet(vec![PacketZcAckTouseskill::packet_id(packetver)], packetver);
        let packet = cast!(packets[0], PacketZcAckTouseskill);
        assert_eq!(packet.cause, UseSkillFailure::HpInsufficient.value() as u8);
    }
    // #[test]
    // fn start_use_skill_should_validate_ammo_requirement() {
    //     // Given
    //     let mut context = before_each();
    //     let mut character = create_character();
    //     let packetver = GlobalConfigService::instance().packetver();
    //     let known_skill = KnownSkill { value: SkillEnum::AcDouble, level: 1 };
    //     character.known_skills.push(known_skill);
    //     character.status.sp = 50;
    //     let item_inventory = add_item_in_inventory(&mut character, "Arrow");
    //     let target = character.to_map_item_snapshot();
    //     // When
    //     context.skill_service.start_use_skill(&mut character, Some(target), known_skill.value.id(), known_skill.level, 0);
    //     // Then
    //     context.test_context.increment_latch().wait_expected_count_with_timeout(2, Duration::from_millis(200));
    //     assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_fov(character.x, character.y, vec![SentPacket::with_count(PacketZcUseskillAck2::packet_id(packetver), 1)]));
    //     let packets = context.test_context.get_sent_packet(vec![PacketZcUseskillAck2::packet_id(packetver)], packetver);
    //     let packet = cast!(packets[0], PacketZcUseskillAck2);
    //     assert_eq!(packet.skid as u32, known_skill.value.id());
    //     assert_eq!(packet.aid, character.char_id);
    //     assert_eq!(packet.target_id, character.char_id);
    //
    //     // Given
    //     context.test_context.reset_increment_latch();
    //     context.test_context.clear_sent_packet();
    //     character.takeoff_equip_item(item_inventory);
    //     // When
    //     context.skill_service.start_use_skill(&mut character, Some(target), known_skill.value.id(), known_skill.level, 0);
    //     // Then
    //     context.test_context.increment_latch().wait_expected_count_with_timeout(1, Duration::from_millis(200));
    //     assert_sent_packet_in_current_packetver!(context, NotificationExpectation::of_char(character.char_id, vec![SentPacket::with_count(PacketZcActionFailure::packet_id(packetver), 1)]));
    // }

    #[test]
    fn start_use_skill_should_validate_skill_level_requirement() {

    }
    #[test]
    fn start_use_skill_should_validate_skill_assignment_requirement() {

    }
    #[test]
    fn start_use_skill_should_validate_zeny_requirement() {

    }
    #[test]
    fn start_use_skill_should_validate_item_requirement() {

    }
    #[test]
    fn start_use_skill_should_validate_target_requirement() {

    }
    #[test]
    fn start_use_skill_should_validate_weapon_requirement() {

    }
    #[test]
    fn start_use_skill_should_validate_range_requirement() {

    }

    #[test]
    fn start_use_skill_should_consume_sp_on_success() {

    }

    #[test]
    fn start_use_skill_should_consume_zeny_on_success() {

    }

    #[test]
    fn start_use_skill_should_consume_item_on_success() {

    }
}