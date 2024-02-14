use std::mem;
use std::sync::{Arc, Once, RwLock};
use std::sync::mpsc::SyncSender;

use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::compiler::CompilationDetail;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value;
use rathena_script_lang_interpreter::lang::value::Native;
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use sprintf::{Printf, vsprintf};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::Receiver;
use models::enums::class::JobName;
use models::enums::look::LookType;
use models::enums::EnumWithNumberValue;
use models::enums::EnumWithStringValue;

use packets::packets::{Packet, PacketZcCloseDialog, PacketZcMenuList, PacketZcNotifyPlayerchat, PacketZcNpcChat, PacketZcOpenEditdlg, PacketZcOpenEditdlgstr, PacketZcSayDialog, PacketZcShowImage2, PacketZcWaitDialog};

use crate::server::model::session::Session;
use crate::server::model::events::client_notification::{CharNotification, Notification};
use crate::server::model::events::game_event::{CharacterLook, CharacterRemoveItem, CharacterRemoveItems, GameEvent};
use crate::server::model::events::game_event::GameEvent::CharacterUpdateLook;
use crate::server::script::constant::{get_battle_flag, load_constant};
use crate::server::Server;


use skill::ScriptSkillService;
use crate::repository::ItemRepository;
use crate::server::request_handler::atcommand::handle_set_job;

use crate::server::service::character::character_service::CharacterService;
use crate::server::service::global_config_service::GlobalConfigService;
use crate::server::service::script_service::ScriptService;
use crate::server::service::server_service::ServerService;
use crate::util::string::StringUtil;

mod global_variable_handler;
pub mod constant;
mod shop;
pub mod skill;
pub(crate) mod bonus;
mod interaction;

pub const VM_THREAD_CONSTANT_INDEX_NPC_ID: usize = 0;
pub const VM_THREAD_CONSTANT_INDEX_CHAR_ID: usize = 1;
pub const VM_THREAD_CONSTANT_INDEX_ACCOUNT_ID: usize = 2;

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

pub struct MapScriptHandler;

pub struct PlayerInteractionScriptHandler {
    pub client_notification_channel: SyncSender<Notification>,
    pub server: Arc<Server>,
    pub player_action_receiver: RwLock<Receiver<Vec<u8>>>,
    pub runtime: Runtime,
    pub configuration_service: &'static GlobalConfigService,
    player_script_handler: &'static PlayerScriptHandler,
}

impl PlayerInteractionScriptHandler {
    pub fn new(client_notification_channel: SyncSender<Notification>, server: Arc<Server>, player_action_receiver: RwLock<Receiver<Vec<u8>>>, runtime: Runtime, configuration_service: &'static GlobalConfigService) -> Self {
        Self {
            client_notification_channel,
            server,
            player_action_receiver,
            runtime,
            configuration_service,
            player_script_handler: crate::server::script::PlayerScriptHandler::instance(),
        }
    }
}


static mut SERVICE_INSTANCE: Option<PlayerScriptHandler> = None;
static SERVICE_INSTANCE_INIT: Once = Once::new();

pub struct PlayerScriptHandler {
    pub server: Arc<Server>,
    pub configuration_service: &'static GlobalConfigService,
}

impl PlayerScriptHandler {
    pub fn instance() -> &'static PlayerScriptHandler {
        unsafe { SERVICE_INSTANCE.as_ref().unwrap() }
    }

    pub fn init(configuration_service: &'static GlobalConfigService, server: Arc<Server>) {
        SERVICE_INSTANCE_INIT.call_once(|| unsafe {
            SERVICE_INSTANCE = Some(crate::server::script::PlayerScriptHandler {
                server,
                configuration_service,
            });
        });
    }
}


impl NativeMethodHandler for MapScriptHandler {
    fn handle(&self, native: &Native, params: Vec<value::Value>, execution_thread: &Thread, call_frame: &CallFrame, source_line: &CompilationDetail, class_name: String) {
        if native.name.eq("print") {
            println!("[DEBUG script {}.{} {} L#{}]: {}", class_name, call_frame.name, source_line.file_name, source_line.start_line, params.iter().map(|p| {
                match p {
                    value::Value::String(v) => v.as_ref().unwrap().clone(),
                    value::Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    value::Value::Reference(v) => format!("{v:?}"),
                    value::Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
        } else if native.name.eq("getglobalvariable") {
            let constant_name = params[0].string_value().unwrap();
            if let Some(value) = load_constant(constant_name) {
                execution_thread.push_constant_on_stack(value);
            } else {
                panic!("ScriptHandler - getglobalvariable no constant found with name {constant_name}");
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

impl PlayerInteractionScriptHandler {
    pub(crate) fn block_recv(&self) -> Option<Vec<u8>> {
        // TODO handle timeout!
        self.player_action_receiver.write().unwrap().blocking_recv()
    }

    pub(crate) fn send_packet_to_char(&self, account_id: u32, packet: &mut dyn Packet) {
        self.client_notification_channel.send(Notification::Char(
            CharNotification::new(account_id, std::mem::take(packet.raw_mut())))).expect("Failed to send packet to char");
    }

}

impl NativeMethodHandler for PlayerInteractionScriptHandler {
    fn handle(&self, native: &Native, params: Vec<value::Value>, execution_thread: &Thread, call_frame: &CallFrame, source_line: &CompilationDetail, class_name: String) {
        if self.handle_interaction(native, &params, execution_thread, call_frame){
            return;
        }
        self.player_script_handler.handle(native, params, execution_thread, call_frame, source_line, class_name);
    }
}

impl NativeMethodHandler for PlayerScriptHandler {
    fn handle(&self, native: &Native, params: Vec<value::Value>, execution_thread: &Thread, call_frame: &CallFrame, source_line: &CompilationDetail, class_name: String) {
        if native.name.eq("print") {
            println!("[DEBUG script {}.{} {} L#{}]: {}", class_name, call_frame.name, source_line.file_name, source_line.start_line, params.iter().map(|p| {
                match p {
                    value::Value::String(v) => v.as_ref().unwrap().clone(),
                    value::Value::Number(v) => format!("{}", v.as_ref().unwrap()),
                    value::Value::Reference(v) => format!("{v:?}"),
                    value::Value::ArrayEntry(_v) => { "array entry: TODO".to_string() }
                }
            }).collect::<Vec<String>>().join(" "));
        } else if native.name.eq("setglobalvariable") {
            self.handle_setglobalvariable(&params, execution_thread);
        } else if native.name.eq("getglobalvariable") {
            self.handle_getglobalvariable(params, execution_thread);
        } else if native.name.eq("setglobalarray") {
            self.handle_setglobalarray(&params, execution_thread);
        } else if native.name.eq("getglobalarray") {
            self.handle_getglobalarray(&params, execution_thread);
        } else if native.name.eq("removeitemsglobalarray") {
            self.handle_remove_item_from_globalarray(&params, execution_thread);
        }  else if native.name.eq("loadconstant") {
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
                self.server.state().get_character_unsafe(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID))
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
                execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID)
            };
            self.server.add_to_next_tick(CharacterUpdateLook(CharacterLook { look_type: LookType::from_value(look_type as usize), look_value: look_value as u16, char_id }));
        } else if native.name.eq("strcharinfo") {
            let info_type = params[0].number_value().unwrap() as usize;
            let char = if params.len() == 2 {
                // TODO
                panic!("setlook with char_id not yet supported")
            } else {
                self.server.state().get_character_unsafe(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID))
            };
            let char_info = match info_type {
                0 => value::Value::new_string(char.name.clone()),
                1 => value::Value::new_string("TODO PARTY NAME".to_string()),
                2 => value::Value::new_string("TODO GUILD NAME".to_string()),
                3 => value::Value::new_string(char.current_map_name().clone()),
                _ => value::Value::new_string(format!("Unknown char info type {info_type}"))
            };
            execution_thread.push_constant_on_stack(char_info);
        } else if native.name.eq("getbattleflag") {
            let constant_name = params[0].string_value().unwrap();
            let value = get_battle_flag(constant_name);
            execution_thread.push_constant_on_stack(value);
        } else if native.name.eq("warp") {
            let map_name = params[0].string_value().unwrap();
            let x = params[1].number_value().unwrap();
            let y = params[2].number_value().unwrap();
            let char_id = if params.len() == 4 {
                // TODO
                panic!("warp with char_id not yet supported")
            } else {
                execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID)
            };
            ServerService::instance().schedule_warp_to_walkable_cell(self.server.state_mut().as_mut(), map_name, x as u16, y as u16, char_id);
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
        } else if native.name.eq("itemskill") {
            let skill_id = params[0].number_value().ok();
            let skill = if let Some(skill_id) = skill_id {
                self.configuration_service.get_skill_config(skill_id as u32)
            } else {
                let skill_name = params[0].string_value().unwrap();
                self.configuration_service.get_skill_config_by_name(skill_name.as_str())
            };
            let skill_level = params[1].number_value().unwrap();
            let check_requirements = params.get(2).unwrap_or(&value::Value::new_number(0)).number_value().unwrap_or(0) == 1;
            ScriptSkillService::instance().handle_skill(self.server.clone().as_ref(), skill, skill_level as u32, check_requirements, execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID));
        } else if native.name.eq("jobname") {
            let job_number = params[0].number_value().expect("Expected jobname argument 0 to be a number");
            execution_thread.push_constant_on_stack(value::Value::new_string(JobName::from_value(job_number as usize).as_str().to_string()));
        } else if native.name.eq("eaclass") {
            let job_number = if !params.is_empty() {
                if params.len() == 2 {
                    warn!("eaclass does not handle the second argument yet!")
                }
                params[0].number_value().expect("Expected eaclass argument 0 to be a number")
            } else {
                self.server.state().get_character_unsafe(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID)).status.job as i32
            };
            execution_thread.push_constant_on_stack(value::Value::new_number(JobName::from_value(job_number as usize).mask() as i32));
        } else if native.name.eq("roclass") {
            let (is_male, mask) = if !params.is_empty() {
                let is_male = if params.len() == 2 {
                    !params[1].string_value().expect("Expected roclass argument 1 to be a string with value 'm' or 'f'").to_lowercase().eq("f")
                } else {
                    true
                };
                (is_male, params[0].number_value().expect("Expected eaclass argument 0 to be a number"))
            } else {
                (true, self.server.state().get_character_unsafe(execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID)).status.job as i32)
            };
            execution_thread.push_constant_on_stack(value::Value::new_number(JobName::from_mask(mask as u64, is_male).map_or(-1, |job| job.value() as i32)));
        } else if native.name.eq("ismounting") {
            warn!("ismounting returns false because feature is not implemented yet");
            execution_thread.push_constant_on_stack(value::Value::new_number(0));
        } else if native.name.eq("checkcart") {
            warn!("checkcart returns false because feature is not implemented yet");
            execution_thread.push_constant_on_stack(value::Value::new_number(0));
        } else if native.name.eq("checkriding") {
            warn!("checkriding returns false because feature is not implemented yet");
            execution_thread.push_constant_on_stack(value::Value::new_number(0));
        } else if native.name.eq("checkfalcon") {
            warn!("checkfalcon returns false because feature is not implemented yet");
            execution_thread.push_constant_on_stack(value::Value::new_number(0));
        } else if native.name.eq("jobchange") {
            let job = params[0].number_value().expect("Expected jobchange argument 0 to be a number");
            handle_set_job(self.server.clone().as_ref(), execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID), vec![job.to_string().as_ref()]);
        } else {
            error!("Native function \"{}\" not handled yet!", native.name);
        }
    }
}
