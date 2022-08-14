use std::io::Write;

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use packets::packets::{PacketZcPcPurchaseItemlist, PacketZcSelectDealtype, PurchaseItem};
use crate::server::script::script::PlayerScriptHandler;
use crate::packets::packets::Packet;

impl PlayerScriptHandler {
    pub fn handle_shop(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, call_frame: &CallFrame) -> bool {
        if native.name.eq("callshop") {
            let mut packet_zc_select_deal_type = PacketZcSelectDealtype::new();
            packet_zc_select_deal_type.naid = self.npc_id;
            packet_zc_select_deal_type.fill_raw();
            socket_send!(self.tcp_stream, packet_zc_select_deal_type.raw());
            let input_value = self.block_recv().unwrap();
            let input_value = input_value[0] as i32;
            execution_thread.push_constant_on_stack(Value::new_number(input_value));
            return true;
        } else if native.name.eq("senditemlist") {
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "senditemlist first argument should be array name")).unwrap();
            let array_items = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "senditemlist second argument should be array name")).unwrap();
            let array_prices = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let mut packet_zc_pc_purchase_itemlist = PacketZcPcPurchaseItemlist::new();
            // TODO get item type from db
            let mut items_list: Vec<PurchaseItem> = vec![];
            for i in 0..array_items.len() {
                let mut purchase_item = PurchaseItem::new();
                let item_constant_ref = array_items.get(i).unwrap().unwrap();
                let array_element = execution_thread.vm.get_from_constant_pool(item_constant_ref).unwrap().value();
                let item_id = array_element.number_value().unwrap();
                purchase_item.set_itid(item_id as u16);
                items_list.push(purchase_item);
            }
            packet_zc_pc_purchase_itemlist.set_item_list(items_list);
            packet_zc_pc_purchase_itemlist.set_packet_length((PacketZcPcPurchaseItemlist::base_len(self.server.packetver()) - PurchaseItem::base_len(self.server.packetver()) + PurchaseItem::base_len(self.server.packetver()) * array_items.len()) as i16);
            packet_zc_pc_purchase_itemlist.fill_raw();
            socket_send!(self.tcp_stream, packet_zc_pc_purchase_itemlist.raw());
            let input_value = self.block_recv().unwrap();
            return true;
        }
        false
    }
}