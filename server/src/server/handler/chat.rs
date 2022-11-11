use std::sync::Arc;
use crate::Server;
use crate::server::core::request::Request;
use crate::server::handler::atcommand::handle_atcommand;
use packets::packets::PacketCzPlayerChat;

pub fn handle_chat(server: &Server, context: Request) {
    let packet_player_char = cast!(context.packet(), PacketCzPlayerChat);
    let char_id = context.session().char_id();
    let character = server.get_character_unsafe(char_id);
    if packet_player_char.msg.starts_with(format!("{} : @", character.name).as_str()) { // TODO make symbol configurable
        drop(character);
        handle_atcommand(server, context, &packet_player_char);
    }
}