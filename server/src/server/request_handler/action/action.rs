use packets::packets::{PacketCzItemPickup, PacketCzRequestAct};
use crate::server::Server;
use crate::server::model::request::Request;
use models::enums::EnumWithNumberValue;
use models::enums::action::ActionType;
use crate::server::model::events::game_event::{CharacterAttack, CharacterPickUpItem, GameEvent};

pub fn handle_pickup_item(server: &Server, context: Request) {
    let packet_cz_item_pickup = cast!(context.packet(), PacketCzItemPickup);
    let map_item_id = packet_cz_item_pickup.itaid;
    server.add_to_next_tick(GameEvent::CharacterPickUpItem(CharacterPickUpItem { char_id: context.session().char_id.unwrap(), map_item_id }));
}

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
        ActionType::Sit => {
            server.add_to_next_tick(GameEvent::CharacterSit(char_id))
        }
        ActionType::Stand => {
            server.add_to_next_tick(GameEvent::CharacterStand(char_id))
        }
        ActionType::AttackNomotion => {}
        ActionType::Splash => {}
        ActionType::Skill => {}
        ActionType::AttackRepeat => {
            server.add_to_next_tick(GameEvent::CharacterAttack(CharacterAttack {
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