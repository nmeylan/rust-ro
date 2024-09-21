use sqlx::{Error, Postgres, Row};
use async_trait::async_trait;
use models::enums::skill_enums::SkillEnum;
use models::status::KnownSkill;
use crate::repository::{CharacterRepository, PgRepository};
use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};

#[async_trait]
impl CharacterRepository for PgRepository {
    async fn character_insert(&self, char_model: &CharInsertModel) -> Result<(), Error> {
        char_model.insert(&self.pool, "char").await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_info(&self, account_id: i32, char_name: &str) -> Result<CharacterInfoNeoUnionWrapped, Error> {
        sqlx::query_as::<_, CharacterInfoNeoUnionWrapped>("SELECT * from char WHERE name = $1 AND account_id = $2")
            .bind(char_name)
            .bind(account_id)
            .fetch_one(&self.pool)
            .await
    }


    async fn characters_info(&self, account_id: u32) -> Vec<CharacterInfoNeoUnionWrapped> {
        sqlx::query_as::<Postgres, CharacterInfoNeoUnionWrapped>("SELECT * FROM char WHERE account_id = $1")
            .bind(account_id as i32)
            .fetch_all(&self.pool).await.unwrap_or(vec![])
    }

    async fn character_delete_reserved(&self, account_id: u32, char_id: u32) -> Result<(), Error> {
        sqlx::query("UPDATE `char` SET delete_date = UNIX_TIMESTAMP(now() + INTERVAL 1 DAY) WHERE account_id = $1 AND char_id = $2")
            .bind(account_id as i32)
            .bind(char_id as i32)
            .execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_save_position(&self, account_id: u32, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
        sqlx::query("UPDATE char SET last_map = $1, last_x = $2, last_y = $3 WHERE account_id = $4 AND char_id = $5")
            .bind(map_name)
            .bind(x as i16)
            .bind(y as i16)
            .bind(account_id as i32)
            .bind(char_id as i32)
            .execute(&self.pool)
            .await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error> {
        let sql = format!("UPDATE char SET {db_column} = $1 WHERE char_id = $2"); // TODO sanitize db_column
        sqlx::query(&sql).bind(value as i32).bind(char_id as i32).execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
        sqlx::query("SELECT zeny FROM char WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await.map(|row| Ok(row.get::<i32, _>(0)))?
    }

    async fn character_allocated_skill_points(&self, char_id: u32) -> Result<i32, Error> {
        sqlx::query("SELECT sum(lv) FROM skill WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await.map(|row| Ok(row.get::<i32, _>(0)))?
    }

    async fn character_skills(&self, char_id: u32) -> Result<Vec<KnownSkill>, Error> {
        sqlx::query("SELECT id, lv FROM skill WHERE char_id = $1")
            .bind(char_id as i32)
            .fetch_all(&self.pool).await
            .map(|rows| rows.iter().map(|row| KnownSkill { value: SkillEnum::from_id(row.get::<i32, _>(0) as u32), level: row.get::<i16, _>(1) as u8 }).collect::<Vec<KnownSkill>>())
    }

    async fn character_fetch(&self, account_id: u32, char_num: u8) -> Result<CharSelectModel, Error> {
        sqlx::query_as::<_, CharSelectModel>("SELECT * FROM char WHERE account_id = $1 AND char_num = $2")
            .bind(account_id as i32)
            .bind(char_num as i16)
            .fetch_one(&self.pool).await
    }


    async fn character_reset_skills(&self, char_id: i32, skills: Vec<i32>) -> Result<(), Error> {
        sqlx::query("DELETE FROM skill WHERE char_id = $1 and id IN (SELECT * FROM UNNEST($2::int4[]))").bind(char_id).bind(skills).execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }

    async fn character_allocate_skill_point(&self, char_id: i32, skill_id: i32, increment: u8) -> Result<(), Error> {
        sqlx::query("INSERT INTO skill (char_id, id, lv, flag) values ($1, $2, $3, 0) ON CONFLICT (char_id, id) DO UPDATE set lv = skill.lv + EXCLUDED.lv")
            .bind(char_id).bind(skill_id).bind(increment as i16).execute(&self.pool).await
            .inspect_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
            })
            .map(|_| ())
    }
}