use async_trait::async_trait;
use crate::repository::{HotKeyRepository, PgRepository};
use crate::server::model::hotkey::Hotkey;
use sqlx::{Error, Executor};

#[async_trait]
impl HotKeyRepository for PgRepository {
    async fn save_hotkeys(&self, char_id: u32, hotkeys: &Vec<Hotkey>) -> Result<(), Error> {
        let mut transaction = self.pool.begin().await?;
        transaction.execute(sqlx::query(
            r#"
            DELETE FROM hotkey WHERE char_id = $1
            "#
        )
            .bind(char_id as i32))
            .await?;

        if !hotkeys.is_empty() {
            let indexes: Vec<i16> = hotkeys.iter().map(|h| h.index).collect();
            let is_skills: Vec<i16> = hotkeys.iter().map(|h| h.is_skill).collect();
            let itemskill_ids: Vec<i32> = hotkeys.iter().map(|h| h.itemskill_id).collect();
            let skill_lvls: Vec<i16> = hotkeys.iter().map(|h| h.skill_lvl).collect();

            transaction.execute(sqlx::query(
                r#"
                INSERT INTO hotkey (char_id, hotkey, type, itemskill_id, skill_lvl)
                SELECT $1, unnest($2::smallint[]), unnest($3::smallint[]), unnest($4::integer[]), unnest($5::smallint[])
                "#
            )
                .bind(char_id as i32)
                .bind(indexes)
                .bind(is_skills)
                .bind(itemskill_ids)
                .bind(skill_lvls))
                .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    async fn load_hotkeys(&self, char_id: u32) -> Result<Vec<Hotkey>, Error> {
        let rows = sqlx::query_as::<_, Hotkey>(
            r#"
            SELECT hotkey, type, itemskill_id, skill_lvl
            FROM hotkey
            WHERE char_id = $1
            "#
        )
            .bind(char_id as i32)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
    }
}