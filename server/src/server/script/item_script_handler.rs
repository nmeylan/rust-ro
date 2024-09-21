use rathena_script_lang_interpreter::lang::call_frame::CallFrame;
use rathena_script_lang_interpreter::lang::compiler::CompilationDetail;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value;
use rathena_script_lang_interpreter::lang::value::{Native, Value};
use rathena_script_lang_interpreter::lang::vm::NativeMethodHandler;
use models::enums::bonus::BonusType;
use models::enums::skill_enums::SkillEnum;
use models::status::Status;
use crate::server::script::bonus::BonusScriptHandler;
use crate::server::script::constant::load_constant;
use crate::server::script::{PlayerScriptHandler, VM_THREAD_CONSTANT_INDEX_CHAR_ID};
use crate::server::service::global_config_service::GlobalConfigService;

pub struct DynamicItemScriptHandler<'character> {
    status: &'character Status,
    item_id: u32,
    configuration_service: &'static GlobalConfigService,
    bonus: BonusScriptHandler,
}

impl<'character> DynamicItemScriptHandler<'character> {
    pub fn new(configuration_service: &'static GlobalConfigService, status: &'character Status, item_id: u32) -> Self {
        Self {
            status,
            item_id,
            configuration_service,
            bonus: BonusScriptHandler::new(),
        }
    }
    pub fn drain(&self) -> Vec<BonusType> {
        let mut write_guard = self.bonus.bonuses.write().unwrap();
        write_guard.drain(0..).collect()
    }
}

impl<'character> NativeMethodHandler for DynamicItemScriptHandler<'character> {
    fn handle(&self, native: &Native, params: Vec<value::Value>, execution_thread: &Thread, _call_frame: &CallFrame, _source_line: &CompilationDetail, _class_name: String) {
        if native.name.eq("skill") {
            let skill_name = params[0].string_value().unwrap();
            let skill = SkillEnum::from_name(skill_name.as_str());
            let skill_level = params[1].number_value().unwrap() as u8;
            let b = &self.bonus;
            crate::bonus!(b, BonusType::EnableSkillId(skill.id(), skill_level))
        } else if native.name.eq("bonus") {
            self.bonus.handle_bonus(params);
        } else if native.name.eq("bonus2") {
            self.bonus.handle_bonus2(params);
        } else if native.name.eq("bonus3") {
            self.bonus.handle_bonus3(params);
        } else if native.name.eq("bonus4") {
            self.bonus.handle_bonus4(params);
        } else if native.name.eq("bonus5") {
            self.bonus.handle_bonus5(params);
        } else if native.name.eq("isequipped") {
            let mut match_count = 0;
            let equipments = self.status.all_equipped_items();
            for param in params.iter() {
                equipments.iter().find(|e| e.item_id() == param.number_value().unwrap())
                    .map(|_| match_count += 1);
            }
            if match_count == params.len() {
                execution_thread.push_constant_on_stack(Value::new_number(1));
            } else {
                execution_thread.push_constant_on_stack(Value::new_number(0));
            }
        } else if native.name.eq("getrefine") {
            let maybe_refinement = self.status.equipments.iter().find(|e| e.item_id == self.item_id as i32)
                .map(|e| e.refine);
            let refinement = if let Some(refinement) = maybe_refinement {
                refinement
            } else {
                self.status.weapons.iter().find(|w| w.item_id == self.item_id as i32)
                    .map(|e| e.refine).unwrap_or_default()
            };
            execution_thread.push_constant_on_stack(Value::new_number(refinement as i32));
        } else if native.name.eq("readparam") {
            if params[0].is_string() {
                let value = match params[0].string_value().unwrap().as_str() {
                    "bAgi" | "bagi" => Value::new_number(self.status.agi as i32),
                    "bVit" | "bvit" => Value::new_number(self.status.vit as i32),
                    "bStr" | "bstr" => Value::new_number(self.status.str as i32),
                    "bDex" | "bdex" => Value::new_number(self.status.dex as i32),
                    "bInt" | "bint" => Value::new_number(self.status.int as i32),
                    "bLuk" | "bluk" => Value::new_number(self.status.luk as i32),
                    _ => Value::new_number(0)
                };
                execution_thread.push_constant_on_stack(value);
            } else {
                panic!("Can't readparam for value {}, failed to execute script for item {}", params[0].number_value().unwrap(), self.item_id);
            }
        } else if native.name.eq("getskilllv") {
            let _skill = if params[0].is_string() {
                SkillEnum::from_name(params[0].string_value().unwrap())
            } else {
                SkillEnum::from_id(params[0].number_value().unwrap() as u32)
            };
            let skill_level = self.status.known_skills.iter().find(|know_skill| matches!(know_skill.value, _skill)).map_or(0, |know_skill| know_skill.level);
            execution_thread.push_constant_on_stack(Value::new_number(skill_level as i32));
        } else if native.name.eq("getglobalvariable") {
            let variable_name = params[0].string_value().unwrap();
            let variable_scope = params[1].string_value().unwrap();
            if variable_scope.as_str() == "char_permanent" {
                if let Some(value) = load_constant(variable_name) {
                    execution_thread.push_constant_on_stack(value);
                    return;
                }
                let _char_id = execution_thread.get_constant(VM_THREAD_CONSTANT_INDEX_CHAR_ID);
                if let Some(value) = PlayerScriptHandler::load_special_char_variable(self.status, variable_name) {
                    execution_thread.push_constant_on_stack(value);
                    return;
                }
            }
            panic!("Can't load global {} with name {}, failed to execute script for item {}", variable_scope, variable_name, self.item_id);
        } else if native.name.eq("loadconstant") || native.name.eq("getglobalvariable") {
            let constant_name = params[0].string_value().unwrap();
            if let Some(value) = load_constant(constant_name) {
                execution_thread.push_constant_on_stack(value);
            }
        } else {
            panic!("Function {} is not implemented, failed to execute script for item {}", native.name, self.item_id);
        }
    }
}