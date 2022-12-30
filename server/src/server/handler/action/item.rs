use crate::server::core::request::Request;
use packets::packets::{PacketCzUseItem, PacketZcUseItemAck2};
use crate::packets::packets::Packet;
use crate::server::events::game_event::{CharacterUseItem, GameEvent};
use crate::server::Server;

pub fn handle_player_use_item(server: &Server, context: Request) {
    let packet_cz_use_item = cast!(context.packet(), PacketCzUseItem);
    if packet_cz_use_item.index > context.configuration().game.max_inventory {
        error!("packet_cz_use_item index is out of max inventory size");
        let mut packet_zc_use_item_ack2 = PacketZcUseItemAck2::new();
        packet_zc_use_item_ack2.set_aid(packet_cz_use_item.aid);
        packet_zc_use_item_ack2.set_index(packet_cz_use_item.index);
        packet_zc_use_item_ack2.set_result(false);
        packet_zc_use_item_ack2.fill_raw();
        socket_send!(context, packet_zc_use_item_ack2);
        return;
    }
    server.add_to_next_tick(GameEvent::CharacterUseItem(CharacterUseItem {
        char_id: context.session().char_id(),
        target_char_id: packet_cz_use_item.aid,
        index: packet_cz_use_item.index as usize,
    }));
}