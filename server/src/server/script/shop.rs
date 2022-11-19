use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use packets::packets::{CzPurchaseItem, PacketZcPcPurchaseItemlist, PacketZcSelectDealtype, PurchaseItem};
use crate::server::script::PlayerScriptHandler;
use crate::server::enums::item::ItemType;
use crate::server::script::{GlobalVariableEntry, GlobalVariableScope};

impl PlayerScriptHandler {
    pub fn handle_shop(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, _call_frame: &CallFrame) -> bool {
        if native.name.eq("callshop") {
            let mut packet_zc_select_deal_type = PacketZcSelectDealtype::new();
            packet_zc_select_deal_type.naid = self.npc_id;
            packet_zc_select_deal_type.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_select_deal_type);
            let input_value = self.block_recv().unwrap(); // Selection: buy or sell
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
            // Retrieve items id and price from VM array
            let mut item_ids: Vec<u32> = vec![];
            let mut price_overrides: Vec<i32> = vec![];
            for i in 0..array_items.len() {
                let item_constant_ref = array_items.get(i).unwrap().unwrap();
                let price_constant_ref = array_prices.get(i).unwrap().unwrap();
                let array_element_item = execution_thread.vm.get_from_constant_pool(item_constant_ref).unwrap().value();
                let array_element_price = execution_thread.vm.get_from_constant_pool(price_constant_ref).unwrap().value();
                let item_id = array_element_item.number_value().unwrap();
                let price = array_element_price.number_value().unwrap();
                item_ids.push(item_id as u32);
                price_overrides.push(price);
            }
            // Build array of PurchaseItem, retrieving some information from db (prices)
            let items = self.runtime.block_on( async{self.server.repository.item_buy_sell_fetch_all_where_ids(item_ids).await }).unwrap();
            let mut items_list: Vec<PurchaseItem> = vec![];
            for (i, _) in price_overrides.iter().enumerate().take(array_items.len()) {
                let mut purchase_item = PurchaseItem::new();
                let item = items.get(i).unwrap();
                purchase_item.set_itid(item.id.unwrap() as u16);
                purchase_item.set_atype(ItemType::from_string(item.item_type.as_str()).value() as u8);
                let price = if price_overrides[i] != -1 { // when script contains price override
                    price_overrides[i]
                } else {
                    item.price_buy.unwrap() as i32
                };
                purchase_item.set_price(price);
                purchase_item.set_discountprice(price); // TODO handle discount, one day
                items_list.push(purchase_item);
            }
            packet_zc_pc_purchase_itemlist.set_item_list(items_list.clone());
            packet_zc_pc_purchase_itemlist.set_packet_length((PacketZcPcPurchaseItemlist::base_len(self.server.packetver()) - PurchaseItem::base_len(self.server.packetver()) + PurchaseItem::base_len(self.server.packetver()) * array_items.len()) as i16);
            packet_zc_pc_purchase_itemlist.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_pc_purchase_itemlist);
            // Wait for player click on "buy"
            let mut items = self.block_recv().unwrap();
            // Once we receive player purchased item
            let items_count = items.remove(0);
            let char_id = self.session.char_id();
            let character = self.server.get_character_unsafe(char_id);
            let mut script_variable_store = character.script_variable_store.lock().unwrap();
            for i in 0..items_count {
                let purchase_item_bytes = items.drain(0..CzPurchaseItem::base_len(self.server.packetver()));
                let purchased_item = CzPurchaseItem::from(purchase_item_bytes.as_slice(), self.server.packetver());
                let item_price = items_list.iter().find(|item| item.itid == purchased_item.itid).unwrap().discountprice;

                script_variable_store.push(
                    GlobalVariableEntry { name: "bought_nameid".to_string(), value: crate::server::script::Value::Number(purchased_item.itid as i32), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
                script_variable_store.push(
                    GlobalVariableEntry { name: "bought_quantity".to_string(), value: crate::server::script::Value::Number(purchased_item.count as i32), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
                script_variable_store.push(
                    GlobalVariableEntry { name: "bought_price".to_string(), value: crate::server::script::Value::Number(item_price), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
            }
            return true;
        }
        false
    }
}