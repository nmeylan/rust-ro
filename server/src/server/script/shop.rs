use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use packets::packets::{CzPurchaseItem, CzSellItem, PacketZcPcPurchaseItemlist, PacketZcPcPurchaseResult, PacketZcPcSellItemlist, PacketZcSelectDealtype, PurchaseItem, SellItem};
use crate::server::script::PlayerScriptHandler;
use enums::item::ItemType;
use crate::enums::EnumWithNumberValue;
use crate::repository::ItemRepository;
use crate::repository::model::item_model::ItemModel;
use crate::enums::EnumWithStringValue;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::script::{GlobalVariableEntry, GlobalVariableScope};
use crate::server::service::global_config_service::GlobalConfigService;

impl PlayerScriptHandler {
    pub fn handle_shop(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, _call_frame: &CallFrame) -> bool {
        if native.name.eq("callshop") {
            let mut packet_zc_select_deal_type = PacketZcSelectDealtype::new(GlobalConfigService::instance().packetver());
            packet_zc_select_deal_type.naid = self.npc_id;
            packet_zc_select_deal_type.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_select_deal_type);
            let input_value = self.block_recv().unwrap(); // Selection: buy or sell
            let input_value = input_value[0] as i32;
            execution_thread.push_constant_on_stack(Value::new_number(input_value));
            return true;
        } else if native.name.eq("sendcharinventory") {
            let char_id = self.session.char_id();
            let character = self.server.state().get_character_unsafe(char_id);
            let mut packet_zc_pc_sell_itemlist = PacketZcPcSellItemlist::new(self.configuration_service.packetver());
            let mut packets_sell_items = vec![];
            character.inventory.iter().enumerate()
                .filter(|(index, item)| item.is_some())
                .map(|(index, item)| (index, item.as_ref().unwrap()))
                .filter(|(_, item)| item.equip == 0)
                .for_each(|(index, item)| {
                    let mut sell_item = SellItem::new(self.configuration_service.packetver());
                    sell_item.set_index(index as i16);
                    let item = self.configuration_service.get_item(item.item_id);
                    let mut price = item.price_sell.unwrap_or(0);
                    if price == 0 {
                        price = item.price_buy.unwrap_or(0);
                    }
                    sell_item.set_price(price);
                    sell_item.set_overchargeprice(price); // TODO take into account overcharge skill
                    packets_sell_items.push(sell_item);
                });
            packet_zc_pc_sell_itemlist.set_packet_length((PacketZcPcSellItemlist::base_len(self.configuration_service.packetver()) + packets_sell_items.len() * SellItem::base_len(self.configuration_service.packetver())) as i16);
            packet_zc_pc_sell_itemlist.set_item_list(packets_sell_items.clone());
            packet_zc_pc_sell_itemlist.fill_raw();
            self.client_notification_channel.send(Notification::Char(CharNotification::new(char_id, packet_zc_pc_sell_itemlist.raw)));
            self.await_player_click_on_sell(&mut packets_sell_items);
            return true;

        } else if native.name.eq("senditemlist") {
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "senditemlist first argument should be array name")).unwrap();
            let array_items = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "senditemlist second argument should be array name")).unwrap();
            let array_prices = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let mut packet_zc_pc_purchase_itemlist = PacketZcPcPurchaseItemlist::new(self.configuration_service.packetver());
            // Retrieve items id and price from VM array
            let mut item_ids: Vec<i32> = vec![];
            let mut price_overrides: Vec<i32> = vec![];
            for i in 0..array_items.len() {
                let item_constant_ref = array_items.get(i).unwrap().unwrap();
                let price_constant_ref = array_prices.get(i).unwrap().unwrap();
                let array_element_item = execution_thread.vm.get_from_constant_pool(item_constant_ref).unwrap().value();
                let array_element_price = execution_thread.vm.get_from_constant_pool(price_constant_ref).unwrap().value();
                let item_id = array_element_item.number_value().unwrap();
                let price = array_element_price.number_value().unwrap();
                item_ids.push(item_id);
                price_overrides.push(price);
            }
            // Build array of PurchaseItem, retrieving some information from db (prices)
            let items = item_ids.iter().map(|id| self.configuration_service.get_item(*id)).collect::<Vec<&ItemModel>>();
            let mut items_list: Vec<PurchaseItem> = vec![];
            for (i, _) in price_overrides.iter().enumerate().take(array_items.len()) {
                let mut purchase_item = PurchaseItem::new(self.configuration_service.packetver());
                let item = items.get(i).unwrap();
                purchase_item.set_itid(item.id as u16);
                purchase_item.set_atype(ItemType::from_string(item.item_type.as_str()).value() as u8);
                let price = if price_overrides[i] != -1 { // when script contains price override
                    price_overrides[i]
                } else {
                    item.price_buy.unwrap()
                };
                purchase_item.set_price(price);
                purchase_item.set_discountprice(price); // TODO handle discount, one day
                items_list.push(purchase_item);
            }
            packet_zc_pc_purchase_itemlist.set_item_list(items_list.clone());
            packet_zc_pc_purchase_itemlist.set_packet_length((PacketZcPcPurchaseItemlist::base_len(self.server.packetver()) + PurchaseItem::base_len(self.server.packetver()) * array_items.len()) as i16);
            packet_zc_pc_purchase_itemlist.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_pc_purchase_itemlist);
            // Wait for player click on "buy"
            self.await_player_click_on_buy(&mut items_list);
            return true;
        } else if native.name.eq("closeshop") {
            let result = if !params.is_empty() {
                params[0].number_value().unwrap_or(0)
            } else {
                0
            };
            let mut packet_zc_pc_purchase_result = PacketZcPcPurchaseResult::new(GlobalConfigService::instance().packetver());
            packet_zc_pc_purchase_result.set_result(result as u8);
            packet_zc_pc_purchase_result.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_pc_purchase_result);
            return true;
        }
        false
    }

    /// We feed character global variable for the shop script.
    /// We could execute get item logic here but we may want more flexibility for shop script, so instead we just feed global variables
    fn await_player_click_on_buy(&self, items_list: &mut [PurchaseItem]) {
        let mut items = self.block_recv().unwrap();
        // Once we receive player purchased items
        let items_count = items.remove(0); // first bytes contains number of purchased items
        let char_id = self.session.char_id();
        let character = self.server.state().get_character_unsafe(char_id);
        let mut script_variable_store = character.script_variable_store.lock().unwrap();
        for i in 0..items_count {
            let purchase_item_bytes = items.drain(0..CzPurchaseItem::base_len(self.server.packetver()));
            let purchased_item = CzPurchaseItem::from(purchase_item_bytes.as_slice(), self.server.packetver());
            // Ensure that packet with purchased item contains item from the list of available items, ignore otherwise
            if let Some(valid_purchased_item) = items_list.iter().find(|item| item.itid == purchased_item.itid) {
                let item_price = valid_purchased_item.discountprice;
                script_variable_store.push(
                    GlobalVariableEntry { name: "bought_nameid".to_string(), value: crate::server::script::Value::Number(purchased_item.itid as i32), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
                script_variable_store.push(
                    GlobalVariableEntry { name: "bought_quantity".to_string(), value: crate::server::script::Value::Number(purchased_item.count as i32), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
                script_variable_store.push(
                    GlobalVariableEntry { name: "bought_price".to_string(), value: crate::server::script::Value::Number(item_price), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
            } else {
                warn!("Player {} attempted to purchased an item with id {} not available in the shop!!! ", char_id, purchased_item.itid);
            }
        }
    }

    /// We feed character global variable for the shop script.
    /// We could execute remove item from inventory logic here but we may want more flexibility for shop script, so instead we just feed global variables
    fn await_player_click_on_sell(&self, items_list: &mut [SellItem]) {
        let mut items = self.block_recv().unwrap();
        // Once we receive player sold items
        let items_count = items.remove(0); // first bytes contains number of purchased items
        let char_id = self.session.char_id();
        let character = self.server.state().get_character_unsafe(char_id);
        let mut script_variable_store = character.script_variable_store.lock().unwrap();
        for i in 0..items_count {
            let sell_item_bytes = items.drain(0..CzSellItem::base_len(self.server.packetver()));
            let sold_item = CzSellItem::from(sell_item_bytes.as_slice(), self.server.packetver());
            if let Some(valid_purchased_item) = items_list.iter().find(|item| item.index == sold_item.index) {
                let item_price = valid_purchased_item.overchargeprice;
                script_variable_store.push(
                    GlobalVariableEntry { name: "sold_item_index".to_string(), value: crate::server::script::Value::Number(sold_item.index as i32), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
                script_variable_store.push(
                    GlobalVariableEntry { name: "sold_quantity".to_string(), value: crate::server::script::Value::Number(sold_item.count as i32), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
                script_variable_store.push(
                    GlobalVariableEntry { name: "sold_price".to_string(), value: crate::server::script::Value::Number(item_price), scope: GlobalVariableScope::CharTemporary, index: Some(i as usize) }
                );
            }
        }
    }
}