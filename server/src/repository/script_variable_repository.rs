use sqlx::Error;
use crate::Repository;
use crate::repository::model::global_variable_registry_model::{AccountRegNum, AccountRegStr, CharRegNum, CharRegStr, ServerRegStr};

impl Repository {
    pub fn script_variable_char_num_save(&self, char_id: u32, key: String, index: u32, value: i32) {
        let char_reg_num = CharRegNum { char_id, key, index, value };
        self.runtime.block_on(async { char_reg_num.upsert(&self.pool, "char_reg_num").await.unwrap() });
    }

    pub fn script_variable_char_str_save(&self, char_id: u32, key: String, index: u32, value: String) {
        let char_reg_str = CharRegStr { char_id, key, index, value };
        self.runtime.block_on(async { char_reg_str.upsert(&self.pool, "char_reg_str").await.unwrap() });
    }

    pub fn script_variable_account_num_save(&self, account_id: u32, key: String, index: u32, value: i32) {
        let account_reg_num = AccountRegNum { account_id, key, index, value };
        self.runtime.block_on(async { account_reg_num.upsert(&self.pool, "global_acc_reg_num").await.unwrap() });
    }

    pub fn script_variable_account_str_save(&self, account_id: u32, key: String, index: u32, value: String) {
        let account_reg_str = AccountRegStr { account_id, key, index, value };
        self.runtime.block_on(async { account_reg_str.upsert(&self.pool, "global_acc_reg_str").await.unwrap() });
    }

    pub fn script_variable_server_num_save(&self, varname: String, index: u32, value: i32) {
        let serverRegStr = ServerRegStr { varname, index, value: value.to_string() };
        self.runtime.block_on(async { serverRegStr.upsert(&self.pool, "mapreg").await.unwrap() });
    }

    pub fn script_variable_server_str_save(&self, varname: String, index: u32, value: String) {
        let server_reg_str = ServerRegStr { varname, index, value };
        self.runtime.block_on(async { server_reg_str.upsert(&self.pool, "mapreg").await.unwrap() });
    }

    pub fn script_variable_char_str_fetch_one(&self, char_id: u32, variable_name: String, index: u32) -> String {
        let char_reg_str: Result<CharRegStr, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, CharRegStr>("SELECT * FROM `char_reg_str` WHERE `char_id` = ? AND `key` = ? AND `index` = ?")
                .bind(char_id).bind(variable_name.clone()).bind(index).fetch_one(&self.pool).await
        });
        if char_reg_str.is_err() {
            error!("char_permanent fetch_one string {} {:?}", variable_name, char_reg_str.as_ref().err().unwrap());
        }
        char_reg_str.map_or(String::from(""), |res| res.value.clone())
    }

    pub fn script_variable_char_num_fetch_one(&self, char_id: u32, variable_name: String, index: u32) -> i32 {
        let char_reg_num: Result<CharRegNum, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, CharRegNum>("SELECT * FROM `char_reg_num` WHERE `char_id` = ? AND `key` = ? AND `index` = ?")
                .bind(char_id).bind(variable_name.clone()).bind(index).fetch_one(&self.pool).await
        });
        if char_reg_num.is_err() {
            error!("char_permanent fetch_one number {} {:?}", variable_name, char_reg_num.as_ref().err().unwrap());
        }
        char_reg_num.map_or(0, |res| res.value.clone())
    }

    pub fn script_variable_account_str_fetch_one(&self, account_id: u32, variable_name: String, index: u32) -> String {
        let account_reg_str: Result<AccountRegStr, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, AccountRegStr>("SELECT * FROM `account_reg_str` WHERE `char_id` = ? AND `key` = ? AND `index` = ?")
                .bind(account_id).bind(variable_name.clone()).bind(index).fetch_one(&self.pool).await
        });
        if account_reg_str.is_err() {
            error!("account_permanent fetch_one string {} {:?}", variable_name, account_reg_str.as_ref().err().unwrap());
        }
        account_reg_str.map_or(String::from(""), |res| res.value.clone())
    }

    pub fn script_variable_account_num_fetch_one(&self, account_id: u32, variable_name: String, index: u32) -> i32 {
        let account_reg_num: Result<AccountRegNum, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, AccountRegNum>("SELECT * FROM `account_reg_num` WHERE `char_id` = ? AND `key` = ? AND `index` = ?")
                .bind(account_id).bind(variable_name.clone()).bind(index).fetch_one(&self.pool).await
        });
        if account_reg_num.is_err() {
            error!("account_permanent fetch_one number {} {:?}", variable_name, account_reg_num.as_ref().err().unwrap());
        }
        account_reg_num.map_or(0, |res| res.value.clone())
    }

    pub fn script_variable_server_str_fetch_one(&self, variable_name: String, index: u32) -> String {
        let server_reg: Result<ServerRegStr, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, ServerRegStr>("SELECT * FROM `mapreg` WHERE AND `varname` = ? AND `index` = ?")
                .bind(variable_name.clone()).bind(index).fetch_one(&self.pool).await
        });
        if server_reg.is_err() {
            error!("server_permanent fetch_one string {} {:?}", variable_name, server_reg.as_ref().err().unwrap());
        }
        server_reg.map_or(String::from(""), |res| res.value.clone())
    }

    pub fn script_variable_server_num_fetch_one(&self, variable_name: String, index: u32) -> i32 {
        let server_reg: Result<ServerRegStr, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, ServerRegStr>("SELECT * FROM `mapreg` WHERE AND `varname` = ? AND `index` = ?")
                .bind(variable_name.clone()).bind(index).fetch_one(&self.pool).await
        });
        if server_reg.is_err() {
            error!("server_permanent fetch_one number {} {:?}", variable_name, server_reg.as_ref().err().unwrap());
        }
        server_reg.map_or(0, |res| res.value.clone().parse::<i32>().unwrap())
    }

    pub fn script_variable_char_str_fetch_all(&self, char_id: u32, variable_name: String) -> Vec<(u32, String)> {
        let char_reg_str: Result<Vec<CharRegStr>, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, CharRegStr>("SELECT * FROM `char_reg_str` WHERE `char_id` = ? AND `key` = ?")
                .bind(char_id).bind(variable_name.clone()).fetch_all(&self.pool).await
        });
        if char_reg_str.is_err() {
            error!("char_permanent fetch_all string {} {:?}", variable_name, char_reg_str.as_ref().err().unwrap());
        }
        char_reg_str.as_ref().map_or(vec![], |rows| rows.iter().map(|r| (r.index, r.value.clone())).collect())
    }

    pub fn script_variable_char_num_fetch_all(&self, char_id: u32, variable_name: String) -> Vec<(u32, i32)> {
        let char_reg_num: Result<Vec<CharRegNum>, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, CharRegNum>("SELECT * FROM `char_reg_num` WHERE `char_id` = ? AND `key` = ? ")
                .bind(char_id).bind(variable_name.clone()).fetch_all(&self.pool).await
        });
        if char_reg_num.is_err() {
            error!("char_permanent fetch_all number {} {:?}", variable_name, char_reg_num.as_ref().err().unwrap());
        }
        char_reg_num.as_ref().map_or(vec![], |rows| rows.iter().map(|r| (r.index, r.value.clone())).collect())
    }

    pub fn script_variable_account_str_fetch_all(&self, account_id: u32, variable_name: String) ->  Vec<(u32, String)> {
        let account_reg_str: Result<Vec<AccountRegStr>, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, AccountRegStr>("SELECT * FROM `account_reg_str` WHERE `char_id` = ? AND `key` = ?")
                .bind(account_id).bind(variable_name.clone()).fetch_all(&self.pool).await
        });
        if account_reg_str.is_err() {
            error!("account_permanent fetch_all string {} {:?}", variable_name, account_reg_str.as_ref().err().unwrap());
        }
        let empty_rows = Vec::<CharRegStr>::new();
        account_reg_str.as_ref().map_or(vec![], |rows| rows.iter().map(|r| (r.index, r.value.clone())).collect())
    }

    pub fn script_variable_account_num_fetch_all(&self, account_id: u32, variable_name: String) -> Vec<(u32, i32)> {
        let account_reg_num: Result<Vec<AccountRegNum>, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, AccountRegNum>("SELECT * FROM `account_reg_num` WHERE `char_id` = ? AND `key` = ?")
                .bind(account_id).bind(variable_name.clone()).fetch_all(&self.pool).await
        });
        if account_reg_num.is_err() {
            error!("account_permanent fetch_all number {} {:?}", variable_name, account_reg_num.as_ref().err().unwrap());
        }
        account_reg_num.as_ref().map_or(vec![], |rows| rows.iter().map(|r| (r.index, r.value.clone())).collect())
    }

    pub fn script_variable_server_str_fetch_all(&self, variable_name: String) ->  Vec<(u32, String)> {
        let server_reg: Result<Vec<ServerRegStr>, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, ServerRegStr>("SELECT * FROM `mapreg` WHERE AND `varname` = ?")
                .bind(variable_name.clone()).fetch_all(&self.pool).await
        });
        if server_reg.is_err() {
            error!("server_permanent fetch_all string {} {:?}", variable_name, server_reg.as_ref().err().unwrap());
        }
        server_reg.as_ref().map_or(vec![], |rows| rows.iter().map(|r| (r.index, r.value.clone())).collect())
    }

    pub fn script_variable_server_num_fetch_all(&self, variable_name: String) -> Vec<(u32, i32)> {
        let server_reg: Result<Vec<ServerRegStr>, Error> = self.runtime.block_on(async {
            sqlx::query_as::<_, ServerRegStr>("SELECT * FROM `mapreg` WHERE AND `varname` = ?")
                .bind(variable_name.clone()).fetch_all(&self.pool).await
        });
        if server_reg.is_err() {
            error!("server_permanent fetch_all number {} {:?}", variable_name, server_reg.as_ref().err().unwrap());
        }
        server_reg.as_ref().map_or(vec![], |rows| rows.iter().map(|r| (r.index, r.value.clone().parse::<i32>().unwrap())).collect())
    }
}