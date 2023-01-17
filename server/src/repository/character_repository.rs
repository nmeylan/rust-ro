use sqlx::{Error, Row};
use crate::Repository;
use async_trait::async_trait;
use crate::repository::CharacterRepository;

#[async_trait]
impl CharacterRepository for Repository {
    async fn character_save_position(&self, account_id: u32, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
        sqlx::query("UPDATE char SET last_map = $1, last_x = $2, last_y = $3 WHERE account_id = $4 AND char_id = $5")
            .bind(map_name)
            .bind(x as i16)
            .bind(y as i16)
            .bind(account_id as i32)
            .bind(char_id as i32)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
                e
            })
            .map(|_| ())
    }

    async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error> {
        let sql = format!("UPDATE char SET {} = $1 WHERE char_id = $2", db_column); // TODO sanitize db_column
        sqlx::query(&sql).bind(value as i32).bind(char_id as i32).execute(&self.pool).await
            .map_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
                e
            })
            .map(|_| ())
    }

    async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
        sqlx::query("SELECT zeny FROM char where char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await.map(|row| Ok(row.get::<i32, _>(0)))?
    }

}