use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use packets::packets::{Packet, PacketZcAckReqnameall2, PacketCzReqnameall2, PacketZcNotifyMapproperty2, PacketZcHatEffect, PacketCzReqname};
use crate::server::server::Server;
use crate::util::string::StringUtil;
use std::io::Write;
use crate::server::core::map::{MapPropertyFlags};
use crate::server::core::request::RequestContext;
use crate::server::core::session::Session;
use crate::util::packet::chain_packets;

pub fn handle_map_item_name(server: Arc<Server>, context: RequestContext) {
    let gid = if context.packet().as_any().downcast_ref::<PacketCzReqnameall2>().is_some() {
        let packet_cz_req_allname2 = cast!(context.packet(), PacketCzReqnameall2);
        packet_cz_req_allname2.gid
    } else if context.packet().as_any().downcast_ref::<PacketCzReqname>().is_some() {
        let packet_cz_req_name = cast!(context.packet(), PacketCzReqname);
        packet_cz_req_name.aid
    } else {
        0
    };
    let map_items_guard = read_lock!(server.map_items);
    let map_item_found = map_items_guard.get(&gid);
    if map_item_found.is_none() {
        error!("Can't find map item with id: {}", gid);
        return;
    }
    let map_item = map_item_found.unwrap();
    let mut packet_zc_ack_reqnameall2 = PacketZcAckReqnameall2::new();
    packet_zc_ack_reqnameall2.set_gid(gid);
    let mut name: [char; 24] = [0 as char; 24];
    // let aaaaa = format!("{} {}", map_item.x(), map_item.y());
    // aaaaa.fill_char_array(name.as_mut());
    info!("{:?}", map_item.name());
    map_item.name().fill_char_array(name.as_mut());
    packet_zc_ack_reqnameall2.set_name(name);
    // TODO handle guild name, guild title
    packet_zc_ack_reqnameall2.fill_raw();
    socket_send!(context.socket(), packet_zc_ack_reqnameall2.raw());
}

pub fn handle_char_loaded_client_side(_server: Arc<Server>, context: RequestContext) {
    info!("Reload char");
    let session = context.session();
    let session_id = session.account_id;

    let character_session = session.character();
    character_session.load_units_in_fov(&session);
    let mut packet_zc_notify_mapproperty2 = PacketZcNotifyMapproperty2::new();
    let mut packet_zc_hat_effect = PacketZcHatEffect::new();
    packet_zc_notify_mapproperty2.set_atype(0x2); // TODO set this correctly see enum map_type in hercules
    let mut flags = MapPropertyFlags::new();
    flags.set_is_use_cart(true); // TODO add other flags correctly
    packet_zc_notify_mapproperty2.set_flags(flags.raw());
    packet_zc_notify_mapproperty2.fill_raw();
    packet_zc_hat_effect.set_aid(session_id);
    packet_zc_hat_effect.set_status(1);
    packet_zc_hat_effect.set_len(9); // len is: 9 (packet len) + number of effects
    packet_zc_hat_effect.fill_raw();
    let final_response_packet: Vec<u8> = chain_packets(vec![&packet_zc_hat_effect, &packet_zc_notify_mapproperty2]);
    socket_send!(context.socket(), &final_response_packet);
}