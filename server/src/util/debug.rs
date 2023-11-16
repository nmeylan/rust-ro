use std::sync::mpsc::SyncSender;
use packets::packets::{PacketZcNotifyChat};

use packets::packets::Packet;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::state::character::Character;

#[allow(dead_code)]
pub fn debug_in_game_chat(notification_sender: SyncSender<Notification>, character: &Character, text: String) {
    let mut zc_notify_chat = PacketZcNotifyChat::new(GlobalConfigService::instance().packetver());
    zc_notify_chat.set_gid(character.account_id);
    zc_notify_chat.set_packet_length((text.len() + 8) as i16);
    zc_notify_chat.set_msg(text);
    zc_notify_chat.fill_raw();
    notification_sender.send(
        Notification::Char(CharNotification::new(character.char_id, std::mem::take(zc_notify_chat.raw_mut())))
    );

}