#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct CharRegStr {
    pub char_id: i32,
    pub key: String,
    pub index: i32,
    pub value: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct CharRegNum {
    pub char_id: i32,
    pub key: String,
    pub index: i32,
    pub value: i32,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct AccountRegStr {
    pub account_id: i32,
    pub key: String,
    pub index: i32,
    pub value: String,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct AccountRegNum {
    pub account_id: i32,
    pub key: String,
    pub index: i32,
    pub value: i32,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct ServerRegStr {
    pub varname: String,
    pub index: i32,
    pub value: String,
}

