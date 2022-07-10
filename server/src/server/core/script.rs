use std::cell::RefCell;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;

use packets::packets::{PacketZcCloseDialog, PacketZcSayDialog, PacketZcWaitDialog};

use crate::packets::packets::Packet;
use crate::Server;

pub struct ScriptHandler;

pub struct PlayerScriptHandler {
    pub tcp_stream: Arc<RwLock<TcpStream>>,
    pub npc_id: u32,
    pub server: Arc<Server>,
    pub player_action_receiver: RwLock<Receiver<Vec<u8>>>,
    pub runtime: Runtime,
}

impl NativeMethodHandler for ScriptHandler {
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
            return;
        }
        error!("Native function \"{}\" not handled yet!", native.name);
    }
}

impl PlayerScriptHandler {

    fn block_recv(&self) -> Option<Vec<u8>> {
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
            return;
        } else if native.name.eq("mes") {
            let mut packet_dialog = PacketZcSayDialog::new();
            packet_dialog.msg = params[0].string_value().unwrap().clone();
            packet_dialog.naid = self.npc_id;
            packet_dialog.packet_length = (PacketZcSayDialog::base_len(self.server.packetver()) as i16 + packet_dialog.msg.len() as i16) + 1 as i16;
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
            info!("Wait player next");
            self.block_recv();
            info!("Receive player next");
        } else {
            error!("Native function \"{}\" not handled yet!", native.name);
        }
    }
}