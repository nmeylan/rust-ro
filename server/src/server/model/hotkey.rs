#[derive(sqlx::FromRow, Debug, PartialEq, Clone)]
pub struct Hotkey {
    #[sqlx(rename = "hotkey")]
    pub index: i16,
    #[sqlx(rename = "type")]
    pub is_skill: i16,
    pub itemskill_id: i32,
    pub skill_lvl: i16,
}
