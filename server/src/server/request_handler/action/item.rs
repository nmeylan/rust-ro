use crate::packets::packets::Packet;
use crate::server::model::events::game_event::{CharacterEquipItem, CharacterRemoveItem, CharacterTakeoffEquipItem, CharacterUseItem, GameEvent};
use crate::server::model::request::Request;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::Server;
use packets::packets::{PacketCzItemThrow, PacketCzReqItemcompositionList, PacketCzReqTakeoffEquip, PacketCzReqWearEquip, PacketCzUseItem, PacketZcUseItemAck2};

pub fn handle_player_use_item(server: &Server, context: Request) {
    let packet_cz_use_item = cast!(context.packet(), PacketCzUseItem);
    if packet_cz_use_item.index > context.configuration().game.max_inventory {
        error!("packet_cz_use_item index is out of max inventory size");
        let mut packet_zc_use_item_ack2 = PacketZcUseItemAck2::new(GlobalConfigService::instance().packetver());
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

pub fn handle_player_equip_item(server: &Server, context: Request) {
    let packet_cz_wear_equip = cast!(context.packet(), PacketCzReqWearEquip);
    server.add_to_next_tick(GameEvent::CharacterEquipItem(CharacterEquipItem {
        char_id: context.session().char_id(),
        index: packet_cz_wear_equip.index as usize,
    }));
}

pub fn handle_player_takeoff_equip_item(server: &Server, context: Request) {
    let packet_cz_req_takeoff_equip = cast!(context.packet(), PacketCzReqTakeoffEquip);
    server.add_to_next_tick(GameEvent::CharacterTakeoffEquipItem(CharacterTakeoffEquipItem {
        char_id: context.session().char_id(),
        index: packet_cz_req_takeoff_equip.index as usize,
    }));
}

pub fn handle_player_drop_item(server: &Server, context: Request) {
    let packet_cz_item_throw = cast!(context.packet(), PacketCzItemThrow);
    server.add_to_next_tick(GameEvent::CharacterDropItem(CharacterRemoveItem {
        char_id: context.session().char_id(),
        index: packet_cz_item_throw.index as usize,
        amount: packet_cz_item_throw.count,
        price: 0
    }));
}

pub fn handle_player_card_slot_card(server: &Server, context: Request) {
    let packet_cz_req_item_composition_list = cast!(context.packet(), PacketCzReqItemcompositionList);

}
