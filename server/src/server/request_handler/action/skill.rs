use packets::packets::PacketCzUseSkill;

use crate::Server;
use crate::server::model::events::game_event::{CharacterUseSkill, GameEvent};
use crate::server::model::request::Request;

pub fn handle_use_skill(server: &Server, context: Request) {
    let packet_cz_use_skill = cast!(context.packet(), PacketCzUseSkill);

    server.add_to_next_tick(GameEvent::CharacterUseSkill(CharacterUseSkill {
        char_id: context.session().char_id(),
        target_id: packet_cz_use_skill.target_id,
        skill_id: packet_cz_use_skill.skid as u32,
        skill_level: packet_cz_use_skill.selected_level as u8,
    }));
}
