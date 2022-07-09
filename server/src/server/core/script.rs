use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;

pub struct ScriptHandler {

}

impl NativeMethodHandler for ScriptHandler {
    fn handle(&self, native: &Native, params: Vec<Value>, execution_thread: &Thread, call_frame: &CallFrame) {
        if native.name.eq("print") {
            println!("{}", params.iter().map(|p| {
                match p {
                    Value::String(v) => v.as_ref().unwrap().clone(),
                    Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    Value::Reference(v) => format!("{:?}", v),
                    Value::ArrayEntry(_v) => { "array entry: TODO".to_string()}
                }
            }).collect::<Vec<String>>().join(" "));
            return;
        } else {
            error!("Native function {} not handled yet!", native.name);
        }
    }
}