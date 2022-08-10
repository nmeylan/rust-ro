use lazy_static::lazy_static;
use rathena_script_lang_interpreter::lang::thread::Thread;
use rathena_script_lang_interpreter::lang::value::Value;
use sqlx::Error;
use regex::Regex;

use crate::repository::model::global_variable_registry_model::{AccountRegNum, AccountRegStr, CharRegNum, CharRegStr, ServerRegNum, ServerRegStr};
use crate::server::script::constant::load_constant;
use crate::server::script::script::PlayerScriptHandler;

lazy_static! {
        static ref CONSTANT_REGEX: Regex = Regex::new("[A-Z_]*").unwrap();
    }

impl PlayerScriptHandler {
    pub fn handle_setglobalvariable(&self, params: &Vec<Value>) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
        let value = params[2].clone();
        if variable_scope == "char_permanent" {
            if value.is_number() {
                let char_reg_num = CharRegNum { char_id: self.session.character.as_ref().unwrap().char_id, key: variable_name.to_string(), index: 0, value: value.number_value().unwrap() };
                self.runtime.block_on(async { char_reg_num.upsert(&self.server.repository.pool, "char_reg_num_db").await.unwrap() });
            } else {
                let char_reg_str = CharRegStr { char_id: self.session.character.as_ref().unwrap().char_id, key: variable_name.to_string(), index: 0, value: value.string_value().unwrap().clone() };
                self.runtime.block_on(async { char_reg_str.upsert(&self.server.repository.pool, "char_reg_str_db").await.unwrap() });
            }
        } else if variable_scope == "account_permanent" {
            if value.is_number() {
                let account_reg_num = AccountRegNum { account_id: self.session.account_id, key: variable_name.to_string(), index: 0, value: value.number_value().unwrap() };
                self.runtime.block_on(async { account_reg_num.upsert(&self.server.repository.pool, "global_acc_reg_num_db").await.unwrap() });
            } else {
                let account_reg_str = AccountRegStr { account_id: self.session.account_id, key: variable_name.to_string(), index: 0, value: value.string_value().unwrap().clone() };
                self.runtime.block_on(async { account_reg_str.upsert(&self.server.repository.pool, "global_acc_reg_str_db").await.unwrap() });
            }
        } else if variable_scope == "server_permanent" {
            if value.is_number() {
                let server_reg_num = ServerRegNum { key: variable_name.to_string(), index: 0, value: value.number_value().unwrap() };
                self.runtime.block_on(async { server_reg_num.upsert(&self.server.repository.pool, "map_reg_num_db").await.unwrap() });
            } else {
                let server_reg_str = ServerRegStr { key: variable_name.to_string(), index: 0, value: value.string_value().unwrap().clone() };
                self.runtime.block_on(async { server_reg_str.upsert(&self.server.repository.pool, "map_reg_str_db").await.unwrap() });
            }
        }
    }

    pub fn handle_getglobalvariable(&self, params: Vec<Value>, execution_thread: &Thread) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
        if variable_scope == "account_permanent" {
            if variable_name.ends_with("\\$") {
                let account_reg_str: Result<AccountRegStr, Error> = self.runtime.block_on(async {
                    sqlx::query_as::<_, AccountRegStr>("SELECT * FROM `global_acc_reg_str_db` WHERE `account_id` = ? AND `key` = ? AND `index` = 0")
                        .bind(self.session.account_id).bind(variable_name).fetch_one(&self.server.repository.pool).await
                });
                if account_reg_str.is_err() {
                    error!("account_permanent {:?}", account_reg_str.as_ref().err().unwrap());
                }
                execution_thread.push_constant_on_stack(Value::String(Some(account_reg_str.as_ref().map_or(String::from(""), |r| r.value.clone()))));
            } else {
                let account_reg_num: Result<AccountRegNum, Error> = self.runtime.block_on(async {
                    sqlx::query_as::<_, AccountRegNum>("SELECT * FROM `global_acc_reg_num_db` WHERE `account_id` = ? AND `key` = ? AND `index` = 0")
                        .bind(self.session.account_id).bind(variable_name).fetch_one(&self.server.repository.pool).await
                });
                if account_reg_num.is_err() {
                    error!("account_permanent {:?}", account_reg_num.as_ref().err().unwrap());
                }
                execution_thread.push_constant_on_stack(Value::Number(Some(account_reg_num.as_ref().map_or(0, |r| r.value))));
            }
        } else if variable_scope == "char_permanent" {
            if let Some(value) = load_constant(variable_name) {
                execution_thread.push_constant_on_stack(value);
                return;
            }
            if variable_name.ends_with("\\$") {
                let char_reg_str: Result<CharRegStr, Error> = self.runtime.block_on(async {
                    sqlx::query_as::<_, CharRegStr>("SELECT * FROM `char_reg_str_db` WHERE `char_id` = ? AND `key` = ? AND `index` = 0")
                        .bind(self.session.character.as_ref().unwrap().char_id).bind(variable_name).fetch_one(&self.server.repository.pool).await
                });
                if char_reg_str.is_err() {
                    error!("char_permanent {:?}", char_reg_str.as_ref().err().unwrap());
                }
                execution_thread.push_constant_on_stack(Value::String(Some(char_reg_str.as_ref().map_or(String::from(""), |r| r.value.clone()))));
            } else {
                let char_reg_num: Result<CharRegNum, Error> = self.runtime.block_on(async {
                    sqlx::query_as::<_, CharRegNum>("SELECT * FROM `char_reg_num_db` WHERE `char_id` = ? AND `key` = ? AND `index` = 0")
                        .bind(self.session.character.as_ref().unwrap().char_id).bind(variable_name).fetch_one(&self.server.repository.pool).await
                });
                if char_reg_num.is_err() {
                    error!("char_permanent {:?}", char_reg_num.as_ref().err().unwrap());
                }
                execution_thread.push_constant_on_stack(Value::Number(Some(char_reg_num.as_ref().map_or(0, |r| r.value))));
            }
        } else if variable_scope == "server_permanent" {
            if variable_name.ends_with("\\$") {
                let server_reg_str: Result<ServerRegStr, Error> = self.runtime.block_on(async {
                    sqlx::query_as::<_, ServerRegStr>("SELECT * FROM `map_reg_str_db` WHERE `key` = ? AND `index` = 0")
                        .bind(variable_name).fetch_one(&self.server.repository.pool).await
                });
                if server_reg_str.is_err() {
                    error!("server_permanent {:?}", server_reg_str.as_ref().err().unwrap());
                }
                execution_thread.push_constant_on_stack(Value::String(Some(server_reg_str.as_ref().map_or(String::from(""), |r| r.value.clone()))));
            } else {
                let server_reg_num: Result<ServerRegNum, Error> = self.runtime.block_on(async {
                    sqlx::query_as::<_, ServerRegNum>("SELECT * FROM `map_reg_num_db` WHERE `key` = ? AND `index` = 0")
                        .bind(variable_name).fetch_one(&self.server.repository.pool).await
                });
                if server_reg_num.is_err() {
                    error!("server_permanent {:?}", server_reg_num.as_ref().err().unwrap());
                }
                execution_thread.push_constant_on_stack(Value::Number(Some(server_reg_num.as_ref().map_or(0, |r| r.value))));
            }
        }
    }

    pub fn handle_setglobalarray(&self, params: &Vec<Value>) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();
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
                        let char_reg_num = CharRegNum { char_id: self.session.character.as_ref().unwrap().char_id, key: variable_name.to_string(), index: array_index as u32, value: value.number_value().unwrap() };
                        self.runtime.block_on(async { char_reg_num.upsert(&self.server.repository.pool, "char_reg_num_db").await.unwrap() });
                    } else {
                        let char_reg_str = CharRegStr { char_id: self.session.character.as_ref().unwrap().char_id, key: variable_name.to_string(), index: array_index as u32, value: value.string_value().unwrap().clone() };
                        self.runtime.block_on(async { char_reg_str.upsert(&self.server.repository.pool, "char_reg_str_db").await.unwrap() });
                    }
                }
                "account_permanent" => {
                    if value.is_number() {
                        let account_reg_num = AccountRegNum { account_id: self.session.account_id, key: variable_name.to_string(), index: array_index as u32, value: value.number_value().unwrap() };
                        self.runtime.block_on(async { account_reg_num.upsert(&self.server.repository.pool, "global_acc_reg_num_db").await.unwrap() });
                    } else {
                        let account_reg_str = AccountRegStr { account_id: self.session.account_id, key: variable_name.to_string(), index: array_index as u32, value: value.string_value().unwrap().clone() };
                        self.runtime.block_on(async { account_reg_str.upsert(&self.server.repository.pool, "global_acc_reg_str_db").await.unwrap() });
                    }
                }
                "server_permanent" => {
                    if value.is_number() {
                        let server_reg_num = ServerRegNum { key: variable_name.to_string(), index: array_index as u32, value: value.number_value().unwrap() };
                        self.runtime.block_on(async { server_reg_num.upsert(&self.server.repository.pool, "map_reg_num_db").await.unwrap() });
                    } else {
                        let server_reg_str = ServerRegStr { key: variable_name.to_string(), index: array_index as u32, value: value.string_value().unwrap().clone() };
                        self.runtime.block_on(async { server_reg_str.upsert(&self.server.repository.pool, "map_reg_str_db").await.unwrap() });
                    }
                }
                &_ => error!("Variable scope {} is not supported yet!", variable_scope)
            }
            index += 2;
        }
    }
    pub fn handle_getglobalarray(&self, params: &Vec<Value>, execution_thread: &Thread) {
        let variable_name = params[0].string_value().unwrap();
        let variable_scope = params[1].string_value().unwrap();

        match variable_scope.as_str() {
            "char_permanent" => {
                if variable_name.ends_with("\\$") {
                    let char_reg_str: Result<Vec<CharRegStr>, Error> = self.runtime.block_on(async {
                        sqlx::query_as::<_, CharRegStr>("SELECT * FROM `char_reg_str_db` WHERE `char_id` = ? AND `key` = ?")
                            .bind(self.session.character.as_ref().unwrap().char_id).bind(variable_name).fetch_all(&self.server.repository.pool).await
                    });
                    if char_reg_str.is_err() {
                        error!("char_permanent {:?}", char_reg_str.as_ref().err().unwrap());
                    }
                    let empty_rows = Vec::<CharRegStr>::new();
                    let iter = char_reg_str.as_ref().map_or(&empty_rows, |r| r).iter();
                    let count = iter.len();
                    iter.for_each(|value| {
                        execution_thread.push_constant_on_stack(Value::new_string(value.value.clone()));
                        execution_thread.push_constant_on_stack(Value::new_number(value.index as i32));
                    });
                    execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
                } else {
                    let char_reg_num: Result<Vec<CharRegNum>, Error> = self.runtime.block_on(async {
                        sqlx::query_as::<_, CharRegNum>("SELECT * FROM `char_reg_num_db` WHERE `char_id` = ? AND `key` = ?")
                            .bind(self.session.character.as_ref().unwrap().char_id).bind(variable_name).fetch_all(&self.server.repository.pool).await
                    });
                    if char_reg_num.is_err() {
                        error!("char_permanent {:?}", char_reg_num.as_ref().err().unwrap());
                    }
                    let empty_rows = Vec::<CharRegNum>::new();
                    let iter = char_reg_num.as_ref().map_or(&empty_rows, |r| r).iter();
                    let count = iter.len();
                    iter.for_each(|value| {
                        execution_thread.push_constant_on_stack(Value::new_number(value.value.clone()));
                        execution_thread.push_constant_on_stack(Value::new_number(value.index as i32));
                    });
                    execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
                }
            }
            "account_permanent" => {
                if variable_name.ends_with("\\$") {
                    let account_reg_str: Result<Vec<AccountRegStr>, Error> = self.runtime.block_on(async {
                        sqlx::query_as::<_, AccountRegStr>("SELECT * FROM `global_acc_reg_str_db` WHERE `account_id` = ? AND `key` = ?")
                            .bind(self.session.account_id).bind(variable_name).fetch_all(&self.server.repository.pool).await
                    });
                    if account_reg_str.is_err() {
                        error!("account_permanent {:?}", account_reg_str.as_ref().err().unwrap());
                    }
                    let empty_rows = Vec::<AccountRegStr>::new();
                    let iter = account_reg_str.as_ref().map_or(&empty_rows, |r| r).iter();
                    let count = iter.len();
                    iter.for_each(|value| {
                        execution_thread.push_constant_on_stack(Value::new_string(value.value.clone()));
                        execution_thread.push_constant_on_stack(Value::new_number(value.index as i32));
                    });
                    execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
                } else {
                    let account_reg_num: Result<Vec<AccountRegNum>, Error> = self.runtime.block_on(async {
                        sqlx::query_as::<_, AccountRegNum>("SELECT * FROM `global_acc_reg_num_db` WHERE `account_id` = ? AND `key` = ?")
                            .bind(self.session.account_id).bind(variable_name).fetch_all(&self.server.repository.pool).await
                    });
                    if account_reg_num.is_err() {
                        error!("account_permanent {:?}", account_reg_num.as_ref().err().unwrap());
                    }
                    let empty_rows = Vec::<AccountRegNum>::new();
                    let iter = account_reg_num.as_ref().map_or(&empty_rows, |r| r).iter();
                    let count = iter.len();
                    iter.for_each(|value| {
                        execution_thread.push_constant_on_stack(Value::new_number(value.value.clone()));
                        execution_thread.push_constant_on_stack(Value::new_number(value.index as i32));
                    });
                    execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
                }
            }
            "server_permanent" => {
                if variable_name.ends_with("\\$") {
                    let server_reg_str: Result<Vec<ServerRegStr>, Error> = self.runtime.block_on(async {
                        sqlx::query_as::<_, ServerRegStr>("SELECT * FROM `map_reg_str_db` WHERE `key` = ?")
                            .bind(variable_name).fetch_all(&self.server.repository.pool).await
                    });
                    if server_reg_str.is_err() {
                        error!("server_permanent {:?}", server_reg_str.as_ref().err().unwrap());
                    }
                    let empty_rows = Vec::<ServerRegStr>::new();
                    let iter = server_reg_str.as_ref().map_or(&empty_rows, |r| r).iter();
                    let count = iter.len();
                    iter.for_each(|value| {
                        execution_thread.push_constant_on_stack(Value::new_string(value.value.clone()));
                        execution_thread.push_constant_on_stack(Value::new_number(value.index as i32));
                    });
                    execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
                } else {
                    let server_reg_num: Result<Vec<ServerRegNum>, Error> = self.runtime.block_on(async {
                        sqlx::query_as::<_, ServerRegNum>("SELECT * FROM `map_reg_num_db` WHERE `key` = ?")
                            .bind(variable_name).fetch_all(&self.server.repository.pool).await
                    });
                    if server_reg_num.is_err() {
                        error!("server_permanent {:?}", server_reg_num.as_ref().err().unwrap());
                    }
                    let empty_rows = Vec::<ServerRegNum>::new();
                    let iter = server_reg_num.as_ref().map_or(&empty_rows, |r| r).iter();
                    let count = iter.len();
                    iter.for_each(|value| {
                        execution_thread.push_constant_on_stack(Value::new_number(value.value.clone()));
                        execution_thread.push_constant_on_stack(Value::new_number(value.index as i32));
                    });
                    execution_thread.push_constant_on_stack(Value::Number(Some((count * 2) as i32)));
                }
            }
            &_ => error!("Variable scope {} is not supported yet!", variable_scope)
        }
    }
}