use packets::packets::PacketCzShortcutKeyChange;

use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::hotkey::Hotkey;
use crate::server::model::request::Request;

pub fn handle_shortcut_change(server: &Server, context: Request) {
    let packet_cz_shortcut_key_change = cast!(context.packet(), PacketCzShortcutKeyChange);
    let hotkey = Hotkey {
        index: packet_cz_shortcut_key_change.index as i16,
        is_skill: packet_cz_shortcut_key_change.short_cut_key.is_skill as i16,
        itemskill_id: packet_cz_shortcut_key_change.short_cut_key.id as i32,
        skill_lvl: packet_cz_shortcut_key_change.short_cut_key.count,
    };
    let char_id = context.session().char_id.unwrap();
    if hotkey.itemskill_id == 0 {
        server.add_to_next_tick(GameEvent::CharacterHotkeyRemove(char_id, hotkey.index as usize))
    } else {
        server.add_to_next_tick(GameEvent::CharacterHotkeyAdd(char_id, hotkey))
    }
}
