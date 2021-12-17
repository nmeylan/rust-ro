use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use packets::packets::{Packet, PacketZcAckReqnameall2, PacketCzReqnameall2, PacketZcNotifyMapproperty2, PacketZcHatEffect};
use crate::server::server::Server;
use crate::util::string::StringUtil;
use std::io::Write;
use std::ops::Deref;
use crate::Map;
use crate::server::core::map::{MapPropertyFlags, RANDOM_CELL};
use crate::server::core::session::Session;
use crate::util::packet::chain_packets;

pub fn handle_map_item_name(server: Arc<Server>, packet: &mut dyn Packet, tcp_stream: Arc<RwLock<TcpStream>>) {
    let packet_cz_req_allname2 = cast!(packet, PacketCzReqnameall2);
    let map_items_guard = read_lock!(server.map_items);
    let map_item_found = map_items_guard.get(&packet_cz_req_allname2.gid);
    if map_item_found.is_none() {
        error!("Can't find map item with id: {}", packet_cz_req_allname2.gid);
        return;
    }
    let map_item = map_item_found.unwrap();
    let mut packet_zc_ack_reqnameall2 = PacketZcAckReqnameall2::new();
    packet_zc_ack_reqnameall2.set_gid(packet_cz_req_allname2.gid);
    let mut name: [char; 24] = [0 as char; 24];
    let aaaaa = format!("{} {}", map_item.x(), map_item.y());
    aaaaa.fill_char_array(name.as_mut());
    // map_item.name().fill_char_array(name.as_mut());
    packet_zc_ack_reqnameall2.set_name(name);
    // TODO handle guild name, guild title
    packet_zc_ack_reqnameall2.fill_raw();
    socket_send!(tcp_stream, packet_zc_ack_reqnameall2.raw());
}

pub fn handle_char_loaded_client_side(_server: Arc<Server>, tcp_stream: Arc<RwLock<TcpStream>>, session: Arc<Session>) {
    info!("Reload char");
    let session_id = session.account_id;

    let mut packet_zc_notify_mapproperty2 = PacketZcNotifyMapproperty2::new();
    let mut packet_zc_hat_effect = PacketZcHatEffect::new();
    packet_zc_notify_mapproperty2.set_atype(0x2); // TODO set this correctly see enum map_type in hercules
    let mut flags = MapPropertyFlags::new();
    flags.set_is_use_cart(true); // TODO add other flags correctly
    packet_zc_notify_mapproperty2.set_flags(flags.raw());
    packet_zc_notify_mapproperty2.fill_raw();
    packet_zc_hat_effect.set_aid(session_id);
    packet_zc_hat_effect.set_status(1);
    packet_zc_hat_effect.set_len(9 + 0); // len is: 9 (packet len) + number of effects
    packet_zc_hat_effect.fill_raw();
    let final_response_packet: Vec<u8> = chain_packets(vec![&packet_zc_hat_effect, &packet_zc_notify_mapproperty2]);
    socket_send!(tcp_stream, &final_response_packet);
}