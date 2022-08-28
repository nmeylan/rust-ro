use std::sync::{Arc, RwLock};
use std::{mem, thread};

use rathena_script_lang_interpreter::lang::vm::Vm;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

use packets::packets::{Packet, PacketCzContactnpc, PacketCzChooseMenu, PacketCzInputEditdlg, PacketCzInputEditdlgstr, PacketCzAckSelectDealtype, PacketCzPcPurchaseItemlist};

use crate::{Script, Server};
use crate::server::core::request::Request;
use crate::server::script::script::PlayerScriptHandler;

pub fn handle_contact_npc(server: Arc<Server>, context: Request) {
    let packet_cz_contact_npc = cast!(context.packet(), PacketCzContactnpc);
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
    let session = context.session();
    let socket = context.socket();
    session.set_script_handler_channel_sender(tx);
    let client_notification_channel = context.client_notification_channel().clone();
    thread::Builder::new().name(format!("script-player-{}-thread", session.account_id)).spawn(move || {
        let script: &Script = cast!(map_item, Script);
        Vm::run_main_function(server_clone.vm.clone(), script.class_reference, script.instance_reference,
                              Box::new(&PlayerScriptHandler { client_notification_channel, npc_id, server: server_clone.clone(), player_action_receiver: RwLock::new(rx), runtime, session: session.clone() })).unwrap()
    }).unwrap();
}

pub fn handle_player_next(context: Request) {
    context.session().script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(vec![0]).unwrap();
}

pub fn handle_player_choose_menu(context: Request) {
    let packet_cz_choose_menu= cast!(context.packet(), PacketCzChooseMenu);
    context.session().script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(Vec::from(packet_cz_choose_menu.num_raw)).unwrap();
}
pub fn handle_player_input_number(context: Request) {
    let packet_cz_input_editlg= cast!(context.packet(), PacketCzInputEditdlg);
    context.session().script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(Vec::from(packet_cz_input_editlg.value_raw)).unwrap();
}
pub fn handle_player_input_string(context: Request) {
    let packet_cz_input_editlgstr= cast!(context.packet(), PacketCzInputEditdlgstr);
    context.session().script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(packet_cz_input_editlgstr.msg_raw.clone()).unwrap();
}

pub fn handle_player_select_deal_type(context: Request) {
    let packet_cz_ack_select_deal_type= cast!(context.packet(), PacketCzAckSelectDealtype);
    context.session().script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(vec![packet_cz_ack_select_deal_type.atype]).unwrap();
}

pub fn handle_player_purchase_items(context: Request) {
    let packet_cz_pc_purchase_item_list = cast!(context.packet(), PacketCzPcPurchaseItemlist);
    let mut bytes = Vec::<u8>::new();
    bytes.push(packet_cz_pc_purchase_item_list.item_list.len() as u8);
    for item_raw in packet_cz_pc_purchase_item_list.item_list_raw.iter() {
        bytes.extend(item_raw.clone());
    }
    context.session().script_handler_channel_sender.lock().unwrap().as_ref().unwrap().blocking_send(bytes).unwrap();
}