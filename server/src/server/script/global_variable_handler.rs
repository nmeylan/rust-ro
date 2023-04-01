use lazy_static::lazy_static;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::Value;
use regex::Regex;
use crate::server::state::character::Character;
use crate::server::model::events::game_event::CharacterZeny;
use crate::server::model::events::game_event::GameEvent::CharacterUpdateZeny;

use crate::server::script::constant::load_constant;
use crate::server::script::{GlobalVariableEntry, GlobalVariableScope};
use crate::server::script::PlayerScriptHandler;

lazy_static! {
        static ref CONSTANT_REGEX: Regex = Regex::new("[A-Z_]*").unwrap();
    }

impl PlayerScriptHandler {
    pub fn handle_setglobalvariable(&self, params: &[Value]) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
        let value = params[2].clone();
        match variable_scope.as_str() {
            "char_permanent" => {
                let char_id = self.session.char_id();
                let character = self.server.state().get_character_unsafe(char_id);
                if self.store_special_char_variable(character, variable_name, &value) {
                    return;
                }
                if value.is_number() {
                    self.server.repository.script_variable_char_num_save(char_id, variable_name.to_string(), 0, value.number_value().unwrap());
                } else {
                    self.server.repository.script_variable_char_str_save(char_id, variable_name.to_string(), 0, value.string_value().unwrap().clone());
                }
            }
            "account_permanent" => {
                if value.is_number() {
                    self.server.repository.script_variable_account_num_save(self.session.account_id, variable_name.to_string(), 0, value.number_value().unwrap());
                } else {
                    self.server.repository.script_variable_account_str_save(self.session.account_id, variable_name.to_string(), 0, value.string_value().unwrap().clone());
                }
            }
            "server_permanent" => {
                if value.is_number() {
                    self.server.repository.script_variable_server_num_save(variable_name.to_string(), 0, value.number_value().unwrap());
                } else {
                    self.server.repository.script_variable_server_str_save(variable_name.to_string(), 0, value.string_value().unwrap().clone());
                }
            }
            "char_temporary" => {
                let char_id = self.session.char_id();
                let character = self.server.state().get_character_unsafe(char_id);
                let mut script_variable_store = character.script_variable_store.lock().unwrap();
                let value = match value {
                    Value::String(v) => crate::server::script::Value::String(v.unwrap()),
                    Value::Number(v) => crate::server::script::Value::Number(v.unwrap()),
                    _ => { panic!("Can't store any variable other than Number or String") }
                };
                script_variable_store.push(
                    GlobalVariableEntry { name: variable_name.clone(), value, scope: GlobalVariableScope::CharTemporary, index: None }
                );
            }
            &_ => error!("Variable scope {} is not supported yet!", variable_scope)
        }
    }

    pub fn handle_getglobalvariable(&self, params: Vec<Value>, execution_thread: &Thread) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
        match variable_scope.as_str() {
            "char_permanent" => {
                if let Some(value) = load_constant(variable_name) {
                    execution_thread.push_constant_on_stack(value);
                    return;
                }
                let char_id = self.session.char_id();
                let character = self.server.state().get_character_unsafe(char_id);
                if let Some(value) = Self::load_special_char_variable(character, variable_name) {
                    execution_thread.push_constant_on_stack(value);
                    return;
                }
                if variable_name.ends_with('$') {
                    execution_thread.push_constant_on_stack(Value::new_string(self.server.repository.script_variable_char_str_fetch_one(char_id, variable_name.clone(), 0)));
                } else {
                    execution_thread.push_constant_on_stack(Value::new_number(self.server.repository.script_variable_char_num_fetch_one(char_id, variable_name.clone(), 0)));
                }
            }
            "account_permanent" => {
                let account_id = self.session.account_id;
                if variable_name.ends_with('$') {
                    execution_thread.push_constant_on_stack(Value::new_string(self.server.repository.script_variable_account_str_fetch_one(account_id, variable_name.clone(), 0)));
                } else {
                    execution_thread.push_constant_on_stack(Value::new_number(self.server.repository.script_variable_account_num_fetch_one(account_id, variable_name.clone(), 0)));
                }
            }
            "server_permanent" => {
                if variable_name.ends_with('$') {
                    execution_thread.push_constant_on_stack(Value::new_string(self.server.repository.script_variable_server_str_fetch_one(variable_name.clone(), 0)));
                } else {
                    execution_thread.push_constant_on_stack(Value::new_number(self.server.repository.script_variable_server_num_fetch_one(variable_name.clone(), 0)));
                }
            }
            "char_temporary" => {
                let char_id = self.session.char_id();
                let character = self.server.state().get_character_unsafe(char_id);
                let script_variable_store = character.script_variable_store.lock().unwrap();
                let entry = script_variable_store.find_global_by_name_and_scope(variable_name, &GlobalVariableScope::CharTemporary);
                if let Some(entry) = entry {
                    let value = match entry.value {
                        crate::server::script::Value::String(v) => Value::String(Some(v)),
                        crate::server::script::Value::Number(v) => Value::Number(Some(v))
                    };
                    execution_thread.push_constant_on_stack(value);
                } else if variable_name.ends_with('$') {
                    execution_thread.push_constant_on_stack(Value::String(Some(String::new())));
                } else {
                    execution_thread.push_constant_on_stack(Value::Number(Some(0)));
                }
            }
            &_ => error!("Variable scope {} is not supported yet!", variable_scope)
        }
    }

    pub fn handle_setglobalarray(&self, params: &Vec<Value>) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
        let char_id = self.session.char_id();
        let character = self.server.state().get_character_unsafe(char_id);
        let mut char_temporary_mutex = if variable_scope == "char_temporary" {
            Some(character.script_variable_store.lock().unwrap())
        } else {
            None
        };
        let mut index = 2;
        loop {
            if index >= params.len() {
                break;
            }
            let array_index = params[index].number_value().unwrap();
            let value = params[index + 1].clone();
            match variable_scope.as_str() {
                "char_permanent" => {
                    if value.is_number() {
                        self.server.repository.script_variable_char_num_save(self.session.char_id(), variable_name.to_string(), array_index as u32, value.number_value().unwrap());
                    } else {
                        self.server.repository.script_variable_char_str_save(self.session.char_id(), variable_name.to_string(), array_index as u32, value.string_value().unwrap().clone());
                    }
                }
                "account_permanent" => {
                    if value.is_number() {
                        self.server.repository.script_variable_account_num_save(self.session.account_id, variable_name.to_string(), array_index as u32, value.number_value().unwrap());
                    } else {
                        self.server.repository.script_variable_account_str_save(self.session.account_id, variable_name.to_string(), array_index as u32, value.string_value().unwrap().clone());
                    }
                }
                "server_permanent" => {
                    if value.is_number() {
                        self.server.repository.script_variable_server_num_save(variable_name.to_string(), array_index as u32, value.number_value().unwrap());
                    } else {
                        self.server.repository.script_variable_server_str_save(variable_name.to_string(), array_index as u32, value.string_value().unwrap().clone());
                    }
                }
                "char_temporary" => {
                    let script_variable_store = char_temporary_mutex.as_mut().unwrap();
                    let value = match value {
                        Value::String(v) => crate::server::script::Value::String(v.unwrap()),
                        Value::Number(v) => crate::server::script::Value::Number(v.unwrap()),
                        _ => { panic!("Can't store any variable other than Number or String") }
                    };
                    script_variable_store.push(
                        GlobalVariableEntry { name: variable_name.clone(), value, scope: GlobalVariableScope::CharTemporary, index: Some(array_index as usize) }
                    );
                }
                &_ => error!("Variable scope {} is not supported yet!", variable_scope)
            }
            index += 2;
        }
    }
    pub fn handle_remove_item_from_globalarray(&self, params: &[Value]) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
        let start_index = params[2].number_value().unwrap();
        let end_index = params[3].number_value().unwrap();
        if variable_scope == "char_temporary" {
            let char_id = self.session.char_id();
            let character = self.server.state().get_character_unsafe(char_id);
            let mut script_variable_store = character.script_variable_store.lock().unwrap();
            for i in start_index..end_index {
                script_variable_store.remove_global_by_name_and_scope_and_index(variable_name, &GlobalVariableScope::CharTemporary, i as usize);
            }
        } else {
            error!("handle_remove_item_from_globalarray not supported yet for scope {}", variable_scope);
        };
    }
    pub fn handle_getglobalarray(&self, params: &[Value], execution_thread: &Thread) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();

        let char_id = self.session.char_id();
        let character = self.server.state().get_character_unsafe(char_id);
        let mut char_temporary_mutex = if variable_scope == "char_temporary" {
            Some(character.script_variable_store.lock().unwrap())
        } else {
            None
        };

        match variable_scope.as_str() {
            "char_permanent" => {
                let char_id = self.session.char_id();
                if variable_name.ends_with('$') {
                    let rows = self.server.repository.script_variable_char_str_fetch_all(char_id, variable_name.clone());
                    Self::push_array_str_elements_on_stack(execution_thread, rows);
                } else {
                    let rows = self.server.repository.script_variable_char_num_fetch_all(char_id, variable_name.clone());
                    Self::push_array_num_elements_on_stack(execution_thread, rows);
                }
            }
            "account_permanent" => {
                let account_id = self.session.account_id;
                if variable_name.ends_with('$') {
                    let rows = self.server.repository.script_variable_account_str_fetch_all(account_id, variable_name.clone());
                    Self::push_array_str_elements_on_stack(execution_thread, rows);
                } else {
                    let rows = self.server.repository.script_variable_account_num_fetch_all(account_id, variable_name.clone());
                    Self::push_array_num_elements_on_stack(execution_thread, rows);
                }
            }
            "server_permanent" => {
                if variable_name.ends_with('$') {
                    let rows = self.server.repository.script_variable_server_str_fetch_all(variable_name.clone());
                    Self::push_array_str_elements_on_stack(execution_thread, rows);
                } else {
                    let rows = self.server.repository.script_variable_server_num_fetch_all(variable_name.clone());
                    Self::push_array_num_elements_on_stack(execution_thread, rows);
                }
            }
            "char_temporary" => {
                let script_variable_store = char_temporary_mutex.as_mut().unwrap();
                let array_entries = script_variable_store.find_global_array_entries(variable_name, GlobalVariableScope::CharTemporary);
                for entry in array_entries.iter() {
                    let value = match entry.value.clone() {
                        crate::server::script::Value::String(v) => Value::String(Some(v)),
                        crate::server::script::Value::Number(v) => Value::Number(Some(v))
                    };
                    execution_thread.push_constant_on_stack(value);
                    execution_thread.push_constant_on_stack(Value::Number(Some(entry.index.unwrap() as i32)));
                }
                execution_thread.push_constant_on_stack(Value::Number(Some((array_entries.len() * 2) as i32)));
            }
            &_ => error!("Variable scope {} is not supported yet!", variable_scope)
        }
    }

    fn push_array_str_elements_on_stack(execution_thread: &Thread, rows: Vec<(u32, String)>) {
        let count = rows.len();
        for (index, value) in rows {
            execution_thread.push_constant_on_stack(Value::new_string(value));
            execution_thread.push_constant_on_stack(Value::new_number(index as i32));
        }
        execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
    }

    fn push_array_num_elements_on_stack(execution_thread: &Thread, rows: Vec<(u32, i32)>) {
        let count = rows.len();
        for (index, value) in rows {
            execution_thread.push_constant_on_stack(Value::new_number(value));
            execution_thread.push_constant_on_stack(Value::new_number(index as i32));
        }
        execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
    }

    fn load_special_char_variable(char: &Character, special_variable_name: &str) -> Option<Value> {
        match special_variable_name {
            "Zeny" => Some(Value::new_number(char.get_zeny() as i32)),
            "Class" => Some(Value::new_number(char.get_job() as i32)),
            "BaseLevel" => Some(Value::new_number(char.get_base_level() as i32)),
            "JobLevel" => Some(Value::new_number(char.get_job_level() as i32)),
            "SkillPoint" => Some(Value::new_number(char.get_skill_point() as i32)),
            "BaseClass   " => Some(Value::new_number(char.get_job() as i32)),
            &_ => None
        }
    }
    fn store_special_char_variable(&self, char: &Character, special_variable_name: &str, value: &Value) -> bool {
        match special_variable_name {
            "Zeny" => {
                self.server.add_to_next_tick(CharacterUpdateZeny(CharacterZeny { char_id: char.char_id, zeny: Some(value.number_value().unwrap() as u32) }));
                true
            }
            &_ => false
        }
    }
}