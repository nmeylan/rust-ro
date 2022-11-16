use std::env::var;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{SyncSender};

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use sprintf::{ConversionSpecifier, Printf, vsprintf};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver};

use packets::packets::{PacketZcCloseDialog, PacketZcMenuList, PacketZcNotifyPlayerchat, PacketZcOpenEditdlg, PacketZcOpenEditdlgstr, PacketZcPcPurchaseItemlist, PacketZcSayDialog, PacketZcSelectDealtype, PacketZcShowImage2, PacketZcSpriteChange2, PacketZcWaitDialog, PurchaseItem};

use crate::packets::packets::Packet;
use crate::Server;
use crate::server::service::character_movement::{change_map_packet};
use crate::server::events::client_notification::{CharNotification, Notification};
use crate::server::events::game_event::CharacterLook;
use crate::server::events::game_event::GameEvent::CharacterUpdateLook;
use crate::server::core::session::Session;
use crate::server::state::status::LookType;
use crate::server::script::constant::{get_battle_flag, load_constant};
use crate::util::string::StringUtil;

pub struct ScriptHandler;

pub struct PlayerScriptHandler {
    pub client_notification_channel: SyncSender<Notification>,
    pub npc_id: u32,
    pub server: Arc<Server>,
    pub player_action_receiver: RwLock<Receiver<Vec<u8>>>,
    pub runtime: Runtime,
    pub session: Arc<Session>,
}

impl NativeMethodHandler for ScriptHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, _call_frame: &CallFrame) {
        if native.name.eq("print") {
            println!("{}", params.iter().map(|p| {
                match p {
                    Value::String(v) => v.as_ref().unwrap().clone(),
                    Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    Value::Reference(v) => format!("{:?}", v),
                    Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
            return;
        } else if native.name.eq("getglobalvariable") {
            let constant_name = params[0].string_value().unwrap();
            if let Some(value) = load_constant(constant_name) {
                execution_thread.push_constant_on_stack(value);
                return;
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
            CharNotification::new(account_id, std::mem::take(packet.raw_mut()))));
    }

    fn handle_menu(&self, execution_thread: &Thread, params: Vec<Value>) -> Option<usize> {
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
            execution_thread.push_constant_on_stack(Value::Number(Some(selected_option as i32)));
            return Some(selected_option as usize);
        } else {
            execution_thread.abort();
            return None;
        }
    }
}

impl NativeMethodHandler for PlayerScriptHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, call_frame: &CallFrame) {
        if native.name.eq("print") {
            println!("{}", params.iter().map(|p| {
                match p {
                    Value::String(v) => v.as_ref().unwrap().clone(),
                    Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    Value::Reference(v) => format!("{:?}", v),
                    Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
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
                        execution_thread.push_constant_on_stack(Value::new_string(message));
                    }
                } else {
                    let input_value = i32::from_le_bytes([input_value[0], input_value[1], input_value[2], input_value[3]]);
                    execution_thread.push_constant_on_stack(Value::new_number(input_value));
                }
            } else {
                execution_thread.abort();
            }
        } else  if native.name.eq("setglobalvariable") {
            self.handle_setglobalvariable(&params);
        } else if native.name.eq("getglobalvariable") {
            self.handle_getglobalvariable(params, execution_thread);
        } else if native.name.eq("setglobalarray") {
            self.handle_setglobalarray(&params);
        } else if native.name.eq("getglobalarray") {
            self.handle_getglobalarray(&params, execution_thread);
        } else if native.name.eq("select") {
            self.handle_menu(execution_thread, params);
        } else if native.name.eq("menu") {
            if let Some(option) = self.handle_menu(execution_thread, params) {}
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
            execution_thread.push_constant_on_stack(Value::new_number(look_value as i32));
        } else if native.name.eq("setlook") {
            let look_type = params[0].number_value().unwrap();
            let look_value = params[1].number_value().unwrap();
            let char_id = if params.len() == 3 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.session.char_id()
            };
            self.server.add_to_next_tick(CharacterUpdateLook(CharacterLook{look_type: LookType::from_value(look_type as usize), look_value: look_value as u32, char_id}));
        } else if native.name.eq("strcharinfo") {
            let info_type = params[0].number_value().unwrap() as usize;
            let char = if params.len() == 2 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.server.get_character_unsafe(self.session.char_id())
            };
            let char_info = match info_type {
                0 => Value::new_string(char.name.clone()),
                1 => Value::new_string("TODO PARTY NAME".to_string()),
                2 => Value::new_string("TODO GUILD NAME".to_string()),
                3 => Value::new_string(char.current_map_name().clone()),
                _ => Value::new_string(format!("Unknown char info type {}", info_type))
            };
            execution_thread.push_constant_on_stack(char_info);
        } else if native.name.eq("message") {
            let char_name = params[0].string_value().unwrap();
            let message = params[1].string_value().unwrap();
            let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new();
            packet_zc_notify_playerchat.set_msg(message.to_string());
            packet_zc_notify_playerchat.set_packet_length((PacketZcNotifyPlayerchat::base_len(self.server.packetver()) + message.len() + 1) as i16);
            packet_zc_notify_playerchat.fill_raw();
            self.send_packet_to_char(self.session.char_id(), &mut packet_zc_notify_playerchat);
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
            change_map_packet(map_name, x as u16, y as u16, session, self.server.as_ref());
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
                execution_thread.push_constant_on_stack(Value::new_string(result));
            } else {
                error!("Unable to parse sprintf due to: {:?}", result.err().unwrap());
                execution_thread.push_constant_on_stack(Value::new_string(String::from("Unable to parse sprintf")));
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

        } else {
            if self.handle_shop(native, params, execution_thread, call_frame) {
                return;
            }
            error!("Native function \"{}\" not handled yet!", native.name);
        }
    }

}