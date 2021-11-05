use std::net::TcpStream;
use std::sync::{Arc};
use tokio::runtime::Runtime;
use packets::packets::{Packet, PacketZcAckReqnameall2, PacketCzReqnameall2};
use crate::server::server::Server;
use crate::util::string::StringUtil;
use std::io::Write;
use parking_lot::RwLock;

pub fn handle_map_item_name(server: Arc<Server>, packet: &mut dyn Packet, _runtime: &Runtime, tcp_stream: Arc<RwLock<TcpStream>>, _session_id: u32) {
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