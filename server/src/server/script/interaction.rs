use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use packets::packets::{PacketZcCloseDialog, PacketZcMenuList, PacketZcNotifyPlayerchat, PacketZcNpcChat, PacketZcOpenEditdlg, PacketZcOpenEditdlgstr, PacketZcSayDialog, PacketZcShowImage2, PacketZcWaitDialog};
use crate::packets::packets::Packet;
use crate::server::model::events::game_event::{CharacterRemoveItem, CharacterRemoveItems, GameEvent};
use crate::server::script::{PlayerInteractionScriptHandler, VM_THREAD_CONSTANT_INDEX_CHAR_ID, VM_THREAD_CONSTANT_INDEX_NPC_ID};
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::string::StringUtil;

impl PlayerInteractionScriptHandler {
    pub fn handle_interaction(&self, native: &Native, params: &Vec<Value>, execution_thread: &Thread, call_frame: &CallFrame) -> bool {
        if native.name.eq("mes") {
            let mut packet_dialog = PacketZcSayDialog::new(GlobalConfigService::instance().packetver());
            packet_dialog.msg = params.iter().map(|text| text.string_value().unwrap().clone()).collect::<Vec<String>>().join("\n");
            packet_dialog.naid = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_NPC_ID);
            packet_dialog.packet_length = (PacketZcSayDialog::base_len(self.server.packetver()) as i16 + packet_dialog.msg.len() as i16) + 1_i16;
            packet_dialog.fill_raw();
            self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_dialog);
            return true;
        } else if native.name.eq("close") {
            let mut packet_dialog = PacketZcCloseDialog::new(GlobalConfigService::instance().packetver());
            packet_dialog.naid = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_NPC_ID);
            packet_dialog.fill_raw();
            self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_dialog);
            return true;
        } else if native.name.eq("next") {
            let mut packet_dialog = PacketZcWaitDialog::new(GlobalConfigService::instance().packetver());
            packet_dialog.naid = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_NPC_ID);
            packet_dialog.fill_raw();
            self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_dialog);
            self.block_recv();
            return true;
        } else if native.name.eq("input") {
            let variable_name = params[0].string_value().unwrap();
            if variable_name.ends_with('$') {
                let mut packet_zc_open_editdlgstr = PacketZcOpenEditdlgstr::new(GlobalConfigService::instance().packetver());
                packet_zc_open_editdlgstr.naid = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_NPC_ID);
                packet_zc_open_editdlgstr.fill_raw();
                self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_zc_open_editdlgstr);
            } else {
                let mut packet_zc_open_editdlg = PacketZcOpenEditdlg::new(GlobalConfigService::instance().packetver());
                packet_zc_open_editdlg.naid = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_NPC_ID);
                packet_zc_open_editdlg.fill_raw();
                self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_zc_open_editdlg);
            }
            let input_value = self.block_recv();
            if let Some(input_value) = input_value {
                if variable_name.ends_with('$') {
                    if let Ok(message) = String::from_utf8(input_value) {
                        execution_thread.push_constant_on_stack(value::Value::new_string(message));
                    }
                } else {
                    let input_value = i32::from_le_bytes([input_value[0], input_value[1], input_value[2], input_value[3]]);
                    execution_thread.push_constant_on_stack(value::Value::new_number(input_value));
                }
            } else {
                execution_thread.abort();
            }
            return true;
        } else if native.name.eq("dispbottom") {
            let message = params[0].string_value().unwrap();
            let green = "0x00FF00".to_string();
            let color = if params.len() > 1 {
                params[1].string_value().unwrap_or(&green).clone()
            } else {
                green
            };
            let color_rgb = if color.starts_with("0x") {
                u32::from_str_radix(format!("{}{}{}", &color[6..8], &color[4..6], &color[2..4]).as_str(), 16).unwrap_or(65280)
            } else {
                65280
            };
            let mut packet_zc_npc_chat = PacketZcNpcChat::new(GlobalConfigService::instance().packetver());
            packet_zc_npc_chat.set_msg(message.to_string());
            packet_zc_npc_chat.set_color(color_rgb);
            packet_zc_npc_chat.set_account_id(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID));
            packet_zc_npc_chat.set_packet_length((PacketZcNpcChat::base_len(self.server.packetver()) + message.len() + 1) as i16);
            packet_zc_npc_chat.fill_raw();
            packet_zc_npc_chat.pretty_debug();
            self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_zc_npc_chat);
            return true;
        } else if native.name.eq("cutin") {
            let file_name = params[0].string_value().unwrap();
            let position = params[1].number_value().unwrap();
            let mut file_name_array: [char; 64] = [0 as char; 64];
            file_name.fill_char_array(file_name_array.as_mut());
            let mut packet_zc_show_image2 = PacketZcShowImage2::new(GlobalConfigService::instance().packetver());
            packet_zc_show_image2.set_image_name(file_name_array);
            packet_zc_show_image2.set_atype(position as u8);
            packet_zc_show_image2.fill_raw();
            self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_zc_show_image2);
            return true;
        } else if native.name.eq("purchaseitems") {
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems first argument should be array reference")).unwrap();
            let items_ids_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems second argument should be array reference")).unwrap();
            let items_amount_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let items_amounts: Vec<i16> = execution_thread.array_constants(items_amount_array).iter().map(|constant| *constant.value().number_value().as_ref().unwrap() as i16).collect::<Vec<i16>>();
            let mut items_ids_amount: Vec<(crate::server::script::Value, i16)> = vec![];
            execution_thread.array_constants(items_ids_array).iter().enumerate().for_each(|(i, constant)| {
                if constant.value().is_number() { // TODO handle string
                    items_ids_amount.push((crate::server::script::Value::Number(constant.value().number_value().unwrap()), items_amounts[i]))
                } else {
                    items_ids_amount.push((crate::server::script::Value::String(constant.value().string_value().unwrap().clone()), items_amounts[i]))
                }
            });

            self.server.script_service().schedule_get_items(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &self.runtime, items_ids_amount, true);
            return true;
        } else if native.name.eq("sellitems") {
            let char_id = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID);
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "sellitems first argument should be array reference")).unwrap();
            let items_ids_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "sellitems second argument should be array reference")).unwrap();
            let items_amount_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let items_amounts: Vec<i16> = execution_thread.array_constants(items_amount_array).iter().map(|constant| *constant.value().number_value().as_ref().unwrap() as i16).collect::<Vec<i16>>();

            let (owner_reference, reference) = params[2].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "sellitems third argument should be array reference")).unwrap();
            let items_price_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let items_prices: Vec<i16> = execution_thread.array_constants(items_price_array).iter().map(|constant| *constant.value().number_value().as_ref().unwrap() as i16).collect::<Vec<i16>>();
            let mut items_to_remove: Vec<CharacterRemoveItem> = vec![];
            execution_thread.array_constants(items_ids_array).iter().enumerate().for_each(|(i, constant)| {
                if constant.value().is_number() {
                    items_to_remove.push(CharacterRemoveItem { char_id, index: constant.value().number_value().unwrap() as usize, amount: items_amounts[i], price: items_prices[i] as i32 })
                }
            });
            self.server.add_to_next_tick(GameEvent::CharacterSellItems(CharacterRemoveItems {
                char_id,
                sell: true,
                items: items_to_remove,
            }));
            return true;
        } else if native.name.eq("checkweight2") {
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems first argument should be array reference")).unwrap();
            let items_ids_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems second argument should be array reference")).unwrap();
            let items_amount_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            self.runtime.block_on(async {
                let items_ids: Vec<i32> = execution_thread.array_constants(items_ids_array.clone()).iter().map(|constant| *constant.value().number_value().as_ref().unwrap()).collect::<Vec<i32>>();
                let items_amounts: Vec<i16> = execution_thread.array_constants(items_amount_array).iter().map(|constant| *constant.value().number_value().as_ref().unwrap() as i16).collect::<Vec<i16>>();
                let mut items_ids_amount: Vec<(i32, i16)> = vec![];
                execution_thread.array_constants(items_ids_array).iter().enumerate().for_each(|(i, constant)| {
                    if constant.value().is_number() { // TODO handle string
                        items_ids_amount.push((constant.value().number_value().unwrap(), items_amounts[i]))
                    }
                });
                let mut items_total_weight = 0;
                self.server.repository.get_weight(items_ids).await.unwrap().iter().for_each(|(id, weight)| {
                    items_total_weight += weight * (items_ids_amount.iter().find(|(iid, _amount)| *iid == *id).unwrap_or(&(*id, 0_i16)).1 as i32)
                });
                let character_ref = self.server.state().get_character_unsafe(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID));
                execution_thread.push_constant_on_stack(value::Value::new_number(if self.server.character_service().can_carry_weight(character_ref, items_total_weight as u32) { 1 } else { 0 }));
            });
            return true;
        } else if native.name.eq("select") {
            self.handle_menu(execution_thread, params);
        } else if native.name.eq("menu") {
            if let Some(_option) = self.handle_menu(execution_thread, params) {}
        } else if native.name.eq("message") {
            let _char_name = params[0].string_value().unwrap();
            let message = params[1].string_value().unwrap();
            let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new(GlobalConfigService::instance().packetver());
            packet_zc_notify_playerchat.set_msg(message.to_string());
            packet_zc_notify_playerchat.set_packet_length((PacketZcNotifyPlayerchat::base_len(self.server.packetver()) + message.len() + 1) as i16);
            packet_zc_notify_playerchat.fill_raw();
            self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_zc_notify_playerchat);
        } else if self.handle_shop(native, params, execution_thread, call_frame) {
            return true;
        }
        false
    }

    fn handle_menu(&self, execution_thread: &Thread, params: &Vec<value::Value>) -> Option<usize> {
        let menu_str = params.iter().map(|p| {
            if p.is_number() {
                format!("{}", p.number_value().unwrap())
            } else if p.is_string() {
                p.string_value().unwrap().clone()
            } else {
                String::new()
            }
        }).collect::<Vec<String>>().join(":");
        let mut packet_zc_menu_list = PacketZcMenuList::new(GlobalConfigService::instance().packetver());
        packet_zc_menu_list.naid = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_NPC_ID);
        packet_zc_menu_list.msg = menu_str;
        packet_zc_menu_list.packet_length = (PacketZcMenuList::base_len(self.server.packetver()) as i16 + packet_zc_menu_list.msg.len() as i16) + 1_i16;
        packet_zc_menu_list.fill_raw();
        self.send_packet_to_char(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), &mut packet_zc_menu_list);
        let selected_option = self.block_recv();
        if let Some(selected_option) = selected_option {
            let selected_option = u8::from_le_bytes([selected_option[0]]);
            if selected_option == 255 {
                execution_thread.abort();
                return None;
            }
            execution_thread.push_constant_on_stack(value::Value::Number(Some(selected_option as i32)));
            Some(selected_option as usize)
        } else {
            execution_thread.abort();
            None
        }
    }
}