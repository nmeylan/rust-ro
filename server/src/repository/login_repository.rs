use async_trait::async_trait;
use sqlx::{Error, Row};

use crate::repository::{LoginRepository, PgRepository};

#[async_trait]
impl LoginRepository for PgRepository {
    async fn login(&self, username: String, password: String) -> Result<u32, Error> {
        sqlx::query("SELECT account_id FROM login WHERE userid = $1 AND user_pass = $2") // TODO add bcrypt on user_pass column, but not supported by hercules
            .bind(username)
            .bind(password)
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i32, _>("account_id") as u32)
    }
}
