use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::spawn;
use futures::task::SpawnExt;

use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use packets::packets::{Packet, PacketCzContactnpc, PacketCzReqNextScript};

use crate::{Script, Server};
use crate::server::core::script::PlayerScriptHandler;
use crate::server::core::session::Session;

pub fn handle_contact_npc(server: Arc<Server>, packet: &mut dyn Packet, tcp_stream: Arc<RwLock<TcpStream>>, session: Arc<Session>) {
    let packet_cz_contact_npc = cast!(packet, PacketCzContactnpc);
    let npc_id = packet_cz_contact_npc.naid;
    let map_items_guard = read_lock!(server.map_items);
    let map_item_found = map_items_guard.get(&npc_id);
    if map_item_found.is_none() {
        error!("Can't find map item with id: {}", npc_id);
        return;
    }
    let map_item = map_item_found.unwrap().clone();
    let server_clone = server.clone();
    let runtime = Runtime::new().unwrap();
    let (tx, rx) = mpsc::channel(1);
    session.set_script_handler_channel_sender(tx);
    thread::Builder::new().name(format!("script-player-{}-thread", session.account_id)).spawn(move || {
        let script: &Script = cast!(map_item, Script);
        Vm::run_main_function(server_clone.vm.clone(), script.class_reference, script.instance_reference,
                              Box::new(&PlayerScriptHandler { tcp_stream: tcp_stream.clone(), npc_id, server: server_clone.clone(), player_action_receiver: RwLock::new(rx), runtime, session: session.clone() })).unwrap()
    }).unwrap();
}

pub fn handle_player_next(server: Arc<Server>, packet: &mut dyn Packet, tcp_stream: Arc<RwLock<TcpStream>>, session: Arc<Session>) {
    let packet_cz_req_next = cast!(packet, PacketCzReqNextScript);
    session.script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(vec![0, 0, 0]);
}