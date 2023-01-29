use packets::packets::{PacketCzRequestAct};
use crate::server::Server;
use crate::server::model::request::Request;
use enums::EnumWithNumberValue;
use enums::action::ActionType;
use crate::server::model::events::game_event::{CharacterAttack, GameEvent};

pub fn handle_action(server: &Server, context: Request) {
    let packet_cz_request_act = cast!(context.packet(), PacketCzRequestAct);
    let session = context.session();
    let char_id = session.char_id();
    // let character = server.get_character_unsafe(char_id);
    // let map_ref = character.current_map.as_ref().unwrap().clone();
    // let mobs_guard = read_lock!(map_ref.mobs);
    // let mob_found = mobs_guard.get(&packet_cz_request_act2.target_gid);
    // if mob_found.is_some() {
    //     info!("Hit {}!", mob_found.unwrap().name);
    // }
    let action_type = ActionType::from_value(packet_cz_request_act.action as usize);
    match action_type {
        ActionType::Attack => {}
        ActionType::Itempickup => {}
        ActionType::Sit => {}
        ActionType::Stand => {}
        ActionType::AttackNomotion => {}
        ActionType::Splash => {}
        ActionType::Skill => {}
        ActionType::AttackRepeat => {
            server.add_to_next_tick(GameEvent::CharacterAttack(CharacterAttack{
                char_id,
                target_id: packet_cz_request_act.target_gid,
                repeat: true,
            }))
        }
        ActionType::AttackMultiple => {}
        ActionType::AttackMultipleNomotion => {}
        ActionType::AttackCritical => {}
        ActionType::AttackLucky => {}
        ActionType::Touchskill => {}
        ActionType::AttackMultipleCritical => {}
    }
}