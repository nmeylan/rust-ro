use packets::packets::{PacketZcNotifyChat};
use crate::server::core::session::Session;

use packets::packets::Packet;

#[allow(dead_code)]
pub fn debug_in_game_chat(session: &Session, text: String) {
    let mut zc_notify_chat = PacketZcNotifyChat::new();
    zc_notify_chat.set_gid(session.account_id);
    zc_notify_chat.set_packet_length((text.len() + 8) as i16);
    zc_notify_chat.set_msg(text);
    zc_notify_chat.fill_raw();
    session.send_to_map_socket(zc_notify_chat.raw());
}