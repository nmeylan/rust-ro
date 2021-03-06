use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;

use packets::packets::{PacketZcCloseDialog, PacketZcMenuList, PacketZcNotifyPlayerchat, PacketZcSayDialog, PacketZcSpriteChange2, PacketZcWaitDialog};

use crate::packets::packets::Packet;
use crate::Server;
use crate::server::core::session::Session;
use crate::server::core::status::LookType;
use crate::server::script::constant::{get_battle_flag, load_constant};

pub struct ScriptHandler;

pub struct PlayerScriptHandler {
    pub tcp_stream: Arc<RwLock<TcpStream>>,
    pub npc_id: u32,
    pub server: Arc<Server>,
    pub player_action_receiver: RwLock<Receiver<Vec<u8>>>,
    pub runtime: Runtime,
    pub session: Arc<Session>,
}

impl NativeMethodHandler for ScriptHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, _execution_thread: &Thread, _call_frame: &CallFrame) {
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
        }
        error!("Native function \"{}\" not handled yet!", native.name);
    }
}

impl PlayerScriptHandler {
    fn block_recv(&self) -> Option<Vec<u8>> {
        // TODO handle timeout!
        self.player_action_receiver.write().unwrap().blocking_recv()
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
            socket_send!(self.tcp_stream, packet_dialog.raw());
        } else if native.name.eq("close") {
            let mut packet_dialog = PacketZcCloseDialog::new();
            packet_dialog.naid = self.npc_id;
            packet_dialog.fill_raw();
            socket_send!(self.tcp_stream, packet_dialog.raw());
        } else if native.name.eq("next") {
            let mut packet_dialog = PacketZcWaitDialog::new();
            packet_dialog.naid = self.npc_id;
            packet_dialog.fill_raw();
            socket_send!(self.tcp_stream, packet_dialog.raw());
            self.block_recv();
        } else if native.name.eq("setglobalvariable") {
            self.handle_setglobalvariable(&params);
        } else if native.name.eq("getglobalvariable") {
            self.handle_getglobalvariable(params, execution_thread);
        } else if native.name.eq("setglobalarray") {
            // let variable_name = params[0].string_value().unwrap();
            // let variable_scope = params[1].string_value().unwrap();
            // let mut index = 2;
            // loop {
            //     if index >= params.len() {
            //         break;
            //     }
            //     let array_index = params[index].number_value().unwrap();
            //     let value = params[index + 1].clone();
            //     self.global_variable_store.lock().unwrap().push(
            //         GlobalVariableEntry {
            //             name: variable_name.clone(),
            //             value,
            //             scope: variable_scope.clone(),
            //             index: Some(array_index as usize)
            //         }
            //     );
            //     index += 2;
            // }
        } else if native.name.eq("getglobalarray") {} else if native.name.eq("getglobalarray") {
            // let variable_name = params[0].string_value().unwrap();
            // let variable_scope = params[1].string_value().unwrap();
            // let array_entries = self.find_global_array_entries(variable_name, variable_scope);
            // for entry in array_entries.iter() {
            //     execution_thread.push_constant_on_stack(entry.value.clone());
            //     execution_thread.push_constant_on_stack(Value::Number(Some(entry.index.unwrap() as i32)));
            // }
            // execution_thread.push_constant_on_stack(Value::Number(Some((array_entries.len() * 2) as i32)));
        } else if native.name.eq("select") {
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
            socket_send!(self.tcp_stream, packet_zc_menu_list.raw());
            let selected_option = self.block_recv();
            if let Some(selected_option) = selected_option {
                let selected_option = u8::from_le_bytes([selected_option[0]]);
                if selected_option == 255 {
                    execution_thread.abort();
                    return;
                }
                execution_thread.push_constant_on_stack(Value::Number(Some(selected_option as i32)));
            } else {
                execution_thread.abort();
            }
        } else if native.name.eq("loadconstant") {
            let constant_name = params[0].string_value().unwrap();
            let value = load_constant(constant_name);
            execution_thread.push_constant_on_stack(value);
        } else if native.name.eq("getlook") {
            let look_type = params[0].number_value().unwrap();
            let char = if params.len() == 2 {
                // TODO
                panic!("getlook with char_id not yet supported")
            } else {
                self.session.character.as_ref().unwrap()
            };
            let look_value = char.get_look(LookType::from_value(look_type as usize));
            execution_thread.push_constant_on_stack(Value::new_number(look_value as i32));
        } else if native.name.eq("setlook") {
            let look_type = params[0].number_value().unwrap();
            let look_value = params[1].number_value().unwrap();
            let char = if params.len() == 3 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.session.character.as_ref().unwrap()
            };
            char.change_look(LookType::from_value(look_type as usize), look_value as u32, &self.runtime, self.server.clone());
            let mut packet_zc_sprite_change = PacketZcSpriteChange2::new();
            packet_zc_sprite_change.set_gid(self.session.character.as_ref().unwrap().char_id);
            packet_zc_sprite_change.set_atype(look_type as u8);
            packet_zc_sprite_change.set_value(look_value);
            packet_zc_sprite_change.fill_raw();
            // TODO: [multiplayer] send to all other char
            socket_send!(self.tcp_stream, packet_zc_sprite_change.raw());
        } else if native.name.eq("strcharinfo") {
            let info_type = params[0].number_value().unwrap() as usize;
            let char = if params.len() == 2 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.session.character.as_ref().unwrap()
            };
            let char_info = match info_type  {
                0 => Value::new_string(char.name.clone()),
                1 => Value::new_string("TODO PARTY NAME".to_string()),
                2 => Value::new_string("TODO GUILD NAME".to_string()),
                3 => Value::new_string(read_lock!(char.current_map).as_ref().unwrap().name.clone()),
                _ => Value::new_string(format!("Unknown char info type {}", info_type))
            };
            execution_thread.push_constant_on_stack(char_info);
        } else if native.name.eq("message") {
            let message = params[0].string_value().unwrap();
            let mut packet_zc_notify_playerchat = PacketZcNotifyPlayerchat::new();
            packet_zc_notify_playerchat.set_msg(message.to_string());
            packet_zc_notify_playerchat.set_packet_length((PacketZcNotifyPlayerchat::base_len(self.server.packetver()) + message.len() + 1) as i16);
            packet_zc_notify_playerchat.fill_raw();
            socket_send!(self.tcp_stream, packet_zc_notify_playerchat.raw());
        } else if native.name.eq("getbattleflag") {
            let constant_name = params[0].string_value().unwrap();
            let value = get_battle_flag(constant_name);
            execution_thread.push_constant_on_stack(value);
        } else {
            error!("Native function \"{}\" not handled yet!", native.name);
        }
    }
}