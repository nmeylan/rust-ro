use crate::packets::packets::{PacketZcNotifyChat, Packet};
use std::io::Write;
use crate::server::core::session::Session;

pub fn debug_in_game_chat(session: &Session, text: String) {
    let mut tcp_stream_guard = crate::write_lock!(session.map_server_socket.as_ref().unwrap());
    let mut zc_notify_chat = PacketZcNotifyChat::new();
    zc_notify_chat.set_gid(session.account_id);
    zc_notify_chat.set_packet_length((text.len() + 8) as i16);
    zc_notify_chat.set_msg(text);
    zc_notify_chat.fill_raw();
    tcp_stream_guard.write(&zc_notify_chat.raw());
    tcp_stream_guard.flush();
}