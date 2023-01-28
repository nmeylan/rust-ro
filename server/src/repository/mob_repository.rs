use sqlx::Error;

use crate::repository::model::mob_model::MobModel;
use crate::repository::Repository;

impl Repository {

    pub async fn get_all_mobs(&self) -> Result<Vec<MobModel>, Error> {
        sqlx::query_as("SELECT * FROM mob_db")
            .fetch_all(&self.pool).await
    }
}