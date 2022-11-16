use sqlx::{Error};
use crate::Repository;

impl Repository {
    pub async fn character_save_position(&self, account_id: u32, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
        sqlx::query("UPDATE `char` SET last_map = ?, last_x = ?, last_y = ? WHERE account_id = ? AND char_id = ?")
            .bind(map_name)
            .bind(x)
            .bind(y)
            .bind(account_id)
            .bind(char_id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
                e
            })
            .map(|_| ())
    }

    pub async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error> {
        let sql = format!("UPDATE `char` SET `{}` = ? WHERE `char_id` = ?", db_column); // TODO sanitize db_column
        sqlx::query(&sql).bind(value).bind(char_id).execute(&self.pool).await
            .map_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
                e
            })
            .map(|_| ())
    }
}