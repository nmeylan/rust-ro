use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use sqlx::Error;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;

use packets::packets::{PacketZcCloseDialog, PacketZcSayDialog, PacketZcWaitDialog};

use crate::packets::packets::Packet;
use crate::repository::model::global_variable_registry_model::{CharRegNum, CharRegStr};
use crate::Server;
use crate::server::core::session::Session;

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
        } else if native.name.eq("getglobalarray") {
            // let variable_name = params[0].string_value().unwrap();
            // let variable_scope = params[1].string_value().unwrap();
            // let array_entries = self.find_global_array_entries(variable_name, variable_scope);
            // for entry in array_entries.iter() {
            //     execution_thread.push_constant_on_stack(entry.value.clone());
            //     execution_thread.push_constant_on_stack(Value::Number(Some(entry.index.unwrap() as i32)));
            // }
            // execution_thread.push_constant_on_stack(Value::Number(Some((array_entries.len() * 2) as i32)));
        } else {
            error!("Native function \"{}\" not handled yet!", native.name);
        }
    }
}