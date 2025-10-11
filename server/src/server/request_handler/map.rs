use models::enums::EnumWithMaskValueU64;
use models::enums::map::MapPropertyFlags;
use packets::packets::{
    Packet, PacketCzReqname, PacketCzReqnameall2, PacketZcAckReqnameall2, PacketZcHatEffect, PacketZcNotifyMapproperty2,
};

use crate::server::Server;
use crate::server::model::events::game_event::GameEvent;
use crate::server::model::request::Request;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::packet::chain_packets;
use crate::util::string::StringUtil;

pub fn handle_map_item_name(server: &Server, context: Request) {
    let gid = if context.packet().as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
        let packet_cz_req_allname2 = cast!(context.packet(), PacketCzReqnameall2);
        packet_cz_req_allname2.gid
    } else if context.packet().as_any().downcast_ref::<PacketCzReqname>().is_some() {
        let packet_cz_req_name = cast!(context.packet(), PacketCzReqname);
        packet_cz_req_name.aid
    } else {
        0
    };
    let character = server.state().get_character_from_context_unsafe(&context);
    let maybe_map_item = server
        .state()
        .map_item(gid, character.current_map_name(), character.current_map_instance());
    if maybe_map_item.is_none() {
        error!("Can't find map item with id: {}", gid);
        return;
    }
    let map_item = maybe_map_item.unwrap();
    let mut packet_zc_ack_reqnameall2 = PacketZcAckReqnameall2::new(GlobalConfigService::instance().packetver());
    packet_zc_ack_reqnameall2.set_gid(gid);
    let mut name: [char; 24] = [0 as char; 24];
    // let aaaaa = format!("{} {}", map_item.x(), map_item.y());
    // aaaaa.fill_char_array(name.as_mut());
    #[cfg(feature = "debug_mob_movement")]
    {
        map_item.id().to_string().fill_char_array(name.as_mut());
    }
    #[cfg(not(feature = "debug_mob_movement"))]
    {
        let map_item_name = server
            .state()
            .map_item_name(&map_item, character.current_map_name(), character.current_map_instance())
            .unwrap_or_else(|| "unknown".to_string());
        map_item_name.fill_char_array(name.as_mut());
    }
    packet_zc_ack_reqnameall2.set_name(name);
    // TODO handle guild name, guild title
    packet_zc_ack_reqnameall2.fill_raw();
    socket_send!(context, packet_zc_ack_reqnameall2);
}

pub fn handle_char_loaded_client_side(server: &Server, context: Request) {
    info!("Reload char");
    let session = context.session();
    let session_id = session.account_id;

    let mut packet_zc_notify_mapproperty2 = PacketZcNotifyMapproperty2::new(GlobalConfigService::instance().packetver());
    let mut packet_zc_hat_effect = PacketZcHatEffect::new(GlobalConfigService::instance().packetver());
    packet_zc_notify_mapproperty2.set_atype(0x2); // TODO set this correctly see enum_macro map_type in hercules

    packet_zc_notify_mapproperty2.set_flags(MapPropertyFlags::IsUseCart.as_flag() as u32);
    packet_zc_notify_mapproperty2.fill_raw();
    packet_zc_hat_effect.set_aid(session_id);
    packet_zc_hat_effect.set_status(1);
    packet_zc_hat_effect.set_len(9); // len is: 9 (packet len) + number of effects
    packet_zc_hat_effect.fill_raw();
    let final_response_packet: Vec<u8> = chain_packets(vec![&packet_zc_hat_effect, &packet_zc_notify_mapproperty2]);
    socket_send_raw!(context, final_response_packet);
    server.add_to_tick(GameEvent::CharacterLoadedFromClientSide(session.char_id.unwrap()), 2);
}
