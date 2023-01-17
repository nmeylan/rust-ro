use std::mem;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::SyncSender;

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value;
use rathena_script_lang_interpreter::lang::value::Native;
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use sprintf::{Printf, vsprintf};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use enums::look::LookType;
use crate::enums::EnumWithNumberValue;

use packets::packets::{Packet, PacketZcCloseDialog, PacketZcMenuList, PacketZcNotifyPlayerchat, PacketZcNpcChat, PacketZcOpenEditdlg, PacketZcOpenEditdlgstr, PacketZcSayDialog, PacketZcShowImage2, PacketZcWaitDialog};

use crate::server::core::session::Session;
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::game_event::CharacterLook;
use crate::server::events::game_event::GameEvent::CharacterUpdateLook;
use crate::server::script::constant::{get_battle_flag, load_constant};
use crate::server::Server;


use skill::SkillService;
use crate::repository::ItemRepository;
use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::util::string::StringUtil;

mod global_variable_handler;
pub mod constant;
mod shop;
pub mod skill;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct GlobalVariableEntry {
    pub name: String,
    pub value: Value,
    pub scope: GlobalVariableScope,
    pub index: Option<usize>,
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum GlobalVariableScope {
    CharTemporary,
    #[allow(dead_code)]
    AccountTemporary,
}

#[derive(Default)]
pub struct ScriptGlobalVariableStore {
    pub variables: Vec<GlobalVariableEntry>,
}

impl ScriptGlobalVariableStore {
    pub fn push(&mut self, variable: GlobalVariableEntry) {
        self.remove_global_by_name_and_scope(&variable.name, &variable.scope, &variable.index);
        self.variables.push(variable);
    }

    pub fn find_global_by_name_and_scope(&self, name: &String, scope: &GlobalVariableScope) -> Option<GlobalVariableEntry> {
        self.variables.iter().find(|entry| entry.name == *name && entry.scope == *scope
            && mem::discriminant(&entry.index) == mem::discriminant(&None)).cloned()
    }

    pub fn remove_global_by_name_and_scope(&mut self, name: &String, scope: &GlobalVariableScope, index: &Option<usize>) {
        let position = self.variables.iter().position(|entry| entry.name == *name && entry.scope == *scope
            && ((index.is_some() && entry.index.is_some() && index.unwrap() == entry.index.unwrap()) || index.is_none() && entry.index.is_none()));
        if let Some(position) = position {
            self.variables.remove(position);
        }
    }
    pub fn remove_global_by_name_and_scope_and_index(&mut self, name: &String, scope: &GlobalVariableScope, index: usize) {
        let position = self.variables.iter().position(|entry| entry.name == *name && entry.scope == *scope
            && entry.index.is_some() && *entry.index.as_ref().unwrap() == index);
        if let Some(position) = position {
            self.variables.remove(position);
        }
    }

    pub fn find_global_array_entries(&self, name: &String, scope: GlobalVariableScope) -> Vec<GlobalVariableEntry> {
        self.variables.iter().filter(|entry| &entry.name == name && entry.scope == scope
            && entry.index.is_some()).cloned().collect::<Vec<GlobalVariableEntry>>()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Value {
    String(String),
    Number(i32),
}

pub struct ScriptHandler;

pub struct PlayerScriptHandler {
    pub client_notification_channel: SyncSender<Notification>,
    pub npc_id: u32,
    pub server: Arc<Server>,
    pub player_action_receiver: RwLock<Receiver<Vec<u8>>>,
    pub runtime: Runtime,
    pub session: Arc<Session>,
    pub configuration_service: &'static GlobalConfigService,
}

impl NativeMethodHandler for ScriptHandler {
    fn handle(&self, native: &Native, params: Vec<value::Value>, execution_thread: &Thread, _call_frame: &CallFrame) {
        if native.name.eq("print") {
            println!("{}", params.iter().map(|p| {
                match p {
                    value::Value::String(v) => v.as_ref().unwrap().clone(),
                    value::Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    value::Value::Reference(v) => format!("{:?}", v),
                    value::Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
        } else if native.name.eq("getglobalvariable") {
            let constant_name = params[0].string_value().unwrap();
            if let Some(value) = load_constant(constant_name) {
                execution_thread.push_constant_on_stack(value);
            } else {
                panic!("ScriptHandler - getglobalvariable no constant found with name {}", constant_name);
            }
        } else if native.name.eq("getbattleflag") {
            let constant_name = params[0].string_value().unwrap();
            let value = get_battle_flag(constant_name);
            execution_thread.push_constant_on_stack(value);
        } else {
            error!("ScriptHandler - Native function \"{}\" not handled yet!", native.name);
        }
    }
}

impl PlayerScriptHandler {
    pub(crate) fn block_recv(&self) -> Option<Vec<u8>> {
        // TODO handle timeout!
        self.player_action_receiver.write().unwrap().blocking_recv()
    }

    pub(crate) fn send_packet_to_char(&self, account_id: u32, packet: &mut dyn Packet) {
        self.client_notification_channel.send(Notification::Char(
            CharNotification::new(account_id, std::mem::take(packet.raw_mut())))).expect("Failed to send packet to char");
    }

    fn handle_menu(&self, execution_thread: &Thread, params: Vec<value::Value>) -> Option<usize> {
        let menu_str = params.iter().map(|p| {
            if p.is_number() {
                format!("{}", p.number_value().unwrap())
            } else if p.is_string() {
                p.string_value().unwrap().clone()
            } else {
                String::new()
            }
        }).collect::<Vec<String>>().join(":");
        let mut packet_zc_menu_list = PacketZcMenuList::new();
        packet_zc_menu_list.naid = self.npc_id;
        packet_zc_menu_list.msg = menu_str;
        packet_zc_menu_list.packet_length = (PacketZcMenuList::base_len(self.server.packetver()) as i16 + packet_zc_menu_list.msg.len() as i16) + 1_i16;
        packet_zc_menu_list.fill_raw();
        self.send_packet_to_char(self.session.char_id(), &mut packet_zc_menu_list);
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

impl NativeMethodHandler for PlayerScriptHandler {
    fn handle(&self, native: &Native, params: Vec<value::Value>, execution_thread: &Thread, call_frame: &CallFrame) {
        if native.name.eq("print") {
            println!("{}", params.iter().map(|p| {
                match p {
                    value::Value::String(v) => v.as_ref().unwrap().clone(),
                    value::Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    value::Value::Reference(v) => format!("{:?}", v),
                    value::Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
        } else if native.name.eq("mes") {
            let mut packet_dialog = PacketZcSayDialog::new();
            packet_dialog.msg = params.iter().map(|text| text.string_value().unwrap().clone()).collect::<Vec<String>>().join("\n");
            packet_dialog.naid = self.npc_id;
            packet_dialog.packet_length = (PacketZcSayDialog::base_len(self.server.packetver()) as i16 + packet_dialog.msg.len() as i16) + 1_i16;
            packet_dialog.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_dialog);
        } else if native.name.eq("close") {
            let mut packet_dialog = PacketZcCloseDialog::new();
            packet_dialog.naid = self.npc_id;
            packet_dialog.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_dialog);
        } else if native.name.eq("next") {
            let mut packet_dialog = PacketZcWaitDialog::new();
            packet_dialog.naid = self.npc_id;
            packet_dialog.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_dialog);
            self.block_recv();
        } else if native.name.eq("input") {
            let variable_name = params[0].string_value().unwrap();
            if variable_name.ends_with('$') {
                let mut packet_zc_open_editdlgstr = PacketZcOpenEditdlgstr::new();
                packet_zc_open_editdlgstr.naid = self.npc_id;
                packet_zc_open_editdlgstr.fill_raw();
                self.send_packet_to_char(self.session.char_id(), &mut packet_zc_open_editdlgstr);
            } else {
                let mut packet_zc_open_editdlg = PacketZcOpenEditdlg::new();
                packet_zc_open_editdlg.naid = self.npc_id;
                packet_zc_open_editdlg.fill_raw();
                self.send_packet_to_char(self.session.char_id(), &mut packet_zc_open_editdlg);
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
        } else if native.name.eq("setglobalvariable") {
            self.handle_setglobalvariable(&params);
        } else if native.name.eq("getglobalvariable") {
            self.handle_getglobalvariable(params, execution_thread);
        } else if native.name.eq("setglobalarray") {
            self.handle_setglobalarray(&params);
        } else if native.name.eq("getglobalarray") {
            self.handle_getglobalarray(&params, execution_thread);
        } else if native.name.eq("removeitemsglobalarray") {
            self.handle_remove_item_from_globalarray(&params);
        } else if native.name.eq("select") {
            self.handle_menu(execution_thread, params);
        } else if native.name.eq("menu") {
            if let Some(_option) = self.handle_menu(execution_thread, params) {}
        } else if native.name.eq("loadconstant") {
            let constant_name = params[0].string_value().unwrap();
            if let Some(value) = load_constant(constant_name) {
                execution_thread.push_constant_on_stack(value);
            }
        } else if native.name.eq("getlook") {
            let look_type = params[0].number_value().unwrap();
            let char = if params.len() == 2 {
                // TODO
                panic!("getlook with char_id not yet supported")
            } else {
                self.server.get_character_unsafe(self.session.char_id())
            };
            let look_value = char.get_look(LookType::from_value(look_type as usize));
            execution_thread.push_constant_on_stack(value::Value::new_number(look_value as i32));
        } else if native.name.eq("setlook") {
            let look_type = params[0].number_value().unwrap();
            let look_value = params[1].number_value().unwrap();
            let char_id = if params.len() == 3 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.session.char_id()
            };
            self.server.add_to_next_tick(CharacterUpdateLook(CharacterLook { look_type: LookType::from_value(look_type as usize), look_value: look_value as u16, char_id }));
        } else if native.name.eq("strcharinfo") {
            let info_type = params[0].number_value().unwrap() as usize;
            let char = if params.len() == 2 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.server.get_character_unsafe(self.session.char_id())
            };
            let char_info = match info_type {
                0 => value::Value::new_string(char.name.clone()),
                1 => value::Value::new_string("TODO PARTY NAME".to_string()),
                2 => value::Value::new_string("TODO GUILD NAME".to_string()),
                3 => value::Value::new_string(char.current_map_name().clone()),
                _ => value::Value::new_string(format!("Unknown char info type {}", info_type))
            };
            execution_thread.push_constant_on_stack(char_info);
        } else if native.name.eq("message") {
            let _char_name = params[0].string_value().unwrap();
            let message = params[1].string_value().unwrap();
            let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new();
            packet_zc_notify_playerchat.set_msg(message.to_string());
            packet_zc_notify_playerchat.set_packet_length((PacketZcNotifyPlayerchat::base_len(self.server.packetver()) + message.len() + 1) as i16);
            packet_zc_notify_playerchat.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_notify_playerchat);
        } else if native.name.eq("dispbottom") {
            let _char_name: &String = params[2].string_value().unwrap_or(&"".to_string());
            let message = params[0].string_value().unwrap();
            let green = "0x00FF00".to_string();
            let color = params[1].string_value().unwrap_or(&green);
            let color_rgb = if color.starts_with("0x") {
                u32::from_str_radix(format!("{}{}{}", &color[6..8], &color[4..6], &color[2..4]).as_str(), 16).unwrap_or(65280)
            } else {
                65280
            };
            let mut packet_zc_npc_chat = PacketZcNpcChat::new();
            packet_zc_npc_chat.set_msg(message.to_string());
            packet_zc_npc_chat.set_color(color_rgb);
            packet_zc_npc_chat.set_account_id(self.session.char_id());
            packet_zc_npc_chat.set_packet_length((PacketZcNpcChat::base_len(self.server.packetver()) + message.len() + 1) as i16);
            packet_zc_npc_chat.fill_raw();
            packet_zc_npc_chat.pretty_debug();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_npc_chat);
        } else if native.name.eq("getbattleflag") {
            let constant_name = params[0].string_value().unwrap();
            let value = get_battle_flag(constant_name);
            execution_thread.push_constant_on_stack(value);
        } else if native.name.eq("warp") {
            let map_name = params[0].string_value().unwrap();
            let x = params[1].number_value().unwrap();
            let y = params[2].number_value().unwrap();
            let session = if params.len() == 4 {
                // TODO
                panic!("warp with char_id not yet supported")
            } else {
                self.session.clone()
            };
            self.server.schedule_warp_to_walkable_cell(map_name, x as u16, y as u16, session.char_id());
        } else if native.name.eq("sprintf") {
            let template = params[0].string_value().unwrap();
            let mut sprintf_args: Vec<&dyn Printf> = vec![];
            let nums = params[1..params.len()].iter().map(|p| if p.is_number() { Some(p.number_value().unwrap()) } else { None }).collect::<Vec<Option<i32>>>();
            params[1..params.len()].iter().enumerate().for_each(|(i, p)| {
                if p.is_number() {
                    sprintf_args.push(nums[i].as_ref().unwrap() as &dyn Printf);
                } else {
                    sprintf_args.push(p.string_value().unwrap() as &dyn Printf);
                }
            });
            let result = vsprintf(template, sprintf_args.as_slice());
            if let Ok(result) = result {
                execution_thread.push_constant_on_stack(value::Value::new_string(result));
            } else {
                error!("Unable to parse sprintf due to: {:?}", result.err().unwrap());
                execution_thread.push_constant_on_stack(value::Value::new_string(String::from("Unable to parse sprintf")));
            }
        } else if native.name.eq("cutin") {
            let file_name = params[0].string_value().unwrap();
            let position = params[1].number_value().unwrap();
            let mut file_name_array: [char; 64] = [0 as char; 64];
            file_name.fill_char_array(file_name_array.as_mut());
            let mut packet_zc_show_image2 = PacketZcShowImage2::new();
            packet_zc_show_image2.set_image_name(file_name_array);
            packet_zc_show_image2.set_atype(position as u8);
            packet_zc_show_image2.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_show_image2);
        } else if native.name.eq("purchaseitems") {
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems first argument should be array reference")).unwrap();
            let items_ids_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems second argument should be array reference")).unwrap();
            let items_amount_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let items_amounts: Vec<i16> = execution_thread.array_constants(items_amount_array).iter().map(|constant| *constant.value().number_value().as_ref().unwrap() as i16).collect::<Vec<i16>>();
            let mut items_ids_amount: Vec<(Value, i16)> = vec![];
            execution_thread.array_constants(items_ids_array).iter().enumerate().for_each(|(i, constant)| {
                if constant.value().is_number() { // TODO handle string
                    items_ids_amount.push((Value::Number(constant.value().number_value().unwrap()), items_amounts[i]))
                } else {
                    items_ids_amount.push((Value::String(constant.value().string_value().unwrap().clone()), items_amounts[i]))
                }
            });

            self.server.schedule_get_items(self.session.char_id(), &self.runtime, items_ids_amount, true);
        } else if native.name.eq("checkweight2") {
            let (owner_reference, reference) = params[0].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems first argument should be array reference")).unwrap();
            let items_ids_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            let (owner_reference, reference) = params[1].reference_value().map_err(|err|
                execution_thread.new_runtime_from_temporary(err, "purchaseitems second argument should be array reference")).unwrap();
            let items_amount_array = execution_thread.vm.array_from_heap_reference(owner_reference, reference).unwrap();
            self.runtime.block_on(async {
                let items_ids: Vec<i32> = execution_thread.array_constants(items_ids_array.clone()).iter().map(|constant| *constant.value().number_value().as_ref().unwrap() as i32).collect::<Vec<i32>>();
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
                let character_ref = self.server.get_character_unsafe(self.session.char_id());
                execution_thread.push_constant_on_stack(value::Value::new_number(if CharacterService::instance().check_weight(character_ref.as_ref(), items_total_weight as u32) { 1 } else { 0 }));
            });
        } else if native.name.eq("itemskill") {
            let skill_id = params[0].number_value().map_or(None, |id| Some(id as i32));
            let skill = if let Some(skill_id) = skill_id {
                self.configuration_service.get_skill_config_by_id(skill_id as u32)
            } else {
                let skill_name = params[0].string_value().unwrap();
                self.configuration_service.get_skill_config(skill_name.as_str())
            };
            let skill_level = params[1].number_value().unwrap();
            let check_requirements = params.get(2).unwrap_or(&value::Value::new_number(0)).number_value().unwrap_or(0) == 1;
            SkillService::instance().handle_skill(self.server.clone().as_ref(), skill, skill_level as u32, check_requirements, self.session.char_id());
        } else {
            if self.handle_shop(native, params, execution_thread, call_frame) {
                return;
            }
            error!("Native function \"{}\" not handled yet!", native.name);
        }
    }
}
