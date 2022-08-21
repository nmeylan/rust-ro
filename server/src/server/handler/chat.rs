use std::sync::Arc;
use crate::Server;
use crate::server::core::request::RequestContext;
use crate::server::handler::atcommand::handle_atcommand;
use packets::packets::PacketCzPlayerChat;

pub fn handle_chat(server: Arc<Server>, context: RequestContext) {
    let packet_player_char = cast!(context.packet(), PacketCzPlayerChat);
    if packet_player_char.msg.starts_with(format!("{} : @", context.session().character.as_ref().unwrap().name).as_str()) { // TODO make symbol configurable
        handle_atcommand(server, context, &packet_player_char);
    }
}