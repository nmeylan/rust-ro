use sqlx::{Error, FromRow};
use sqlx::mysql::MySqlRow;

#[derive(sqlx::FromRow, sql::SqlUpsert)]
pub struct CharRegStr {
    pub char_id: u32,
    pub key: String,
    pub index: u32,
    pub value: String,
}

#[derive(sqlx::FromRow, sql::SqlUpsert)]
pub struct CharRegNum {
    pub char_id: u32,
    pub key: String,
    pub index: u32,
    pub value: i32,
}

#[derive(sqlx::FromRow, sql::SqlUpsert)]
pub struct AccountRegStr {
    pub account_id: u32,
    pub key: String,
    pub index: u32,
    pub value: String,
}

#[derive(sqlx::FromRow, sql::SqlUpsert)]
pub struct AccountRegNum {
    pub account_id: u32,
    pub key: String,
    pub index: u32,
    pub value: i32,
}


#[derive(sqlx::FromRow, sql::SqlUpsert)]
pub struct ServerRegStr {
    pub key: String,
    pub index: u32,
    pub value: String,
}

#[derive(sqlx::FromRow, sql::SqlUpsert)]
pub struct ServerRegNum {
    pub key: String,
    pub index: u32,
    pub value: i32,
}
