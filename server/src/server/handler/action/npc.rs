use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use rathena_script_lang_interpreter::lang::vm::Vm;
use packets::packets::{Packet, PacketCzContactnpc};
use crate::{Script, Server};

pub fn handle_contact_npc(server: Arc<Server>, packet: &mut dyn Packet, tcp_stream: Arc<RwLock<TcpStream>>) {
    let packet_cz_contact_npc = cast!(packet, PacketCzContactnpc);
    let npc_id = packet_cz_contact_npc.naid;
    let map_items_guard = read_lock!(server.map_items);
    let map_item_found = map_items_guard.get(&npc_id);
    if map_item_found.is_none() {
        error!("Can't find map item with id: {}", npc_id);
        return;
    }
    let map_item = map_item_found.unwrap();
    let script: &Script = cast!(map_item, Script);

    Vm::run_main_function(server.vm.clone(), script.class_reference, script.instance_reference).unwrap();
}