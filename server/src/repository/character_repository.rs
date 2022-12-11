use std::future::join;
use futures::try_join;
use sqlx::{Error, Executor};
use sqlx::postgres::PgQueryResult;
use crate::Repository;
use crate::server::events::persistence_event::InventoryItemUpdate;

impl Repository {
    pub async fn character_save_position(&self, account_id: u32, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
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

    pub async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error> {
        let sql = format!("UPDATE char SET {} = $1 WHERE char_id = $2", db_column); // TODO sanitize db_column
        sqlx::query(&sql).bind(value as i32).bind(char_id as i32).execute(&self.pool).await
            .map_err(|e| {
                error!("DB error: {}", e.as_database_error().unwrap());
                e
            })
            .map(|_| ())
    }

    pub async fn character_inventory_update(&self, inventory_update_items: &Vec<InventoryItemUpdate>) -> Result<(), Error> {
        let stackable_items = inventory_update_items.iter().filter(|item| item.stackable).collect::<Vec<&InventoryItemUpdate>>();
        let not_stackable_items = inventory_update_items.iter().filter(|item| !item.stackable).collect::<Vec<&InventoryItemUpdate>>();

        let mut tx = self.pool.begin().await.unwrap();
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identify) \
        (SELECT * FROM UNNEST($1::int4[], $2::int2[], $3::int2[], $4::bool[])) \
        ON CONFLICT (char_id, nameid, unique_id)\
         DO UPDATE set amount = inventory.amount + (SELECT * FROM UNNEST($3::int2[]))")
            .bind(stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(stackable_items.iter().map(|i| i.item_id).collect::<Vec<i16>>())
            .bind(stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())).await?;
         tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identify, unique_id) \
        (SELECT * FROM UNNEST($1::int4[], $2::int2[], $3::int2[], $4::bool[], $5::int8[])) ")
            .bind(not_stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(not_stackable_items.iter().map(|i| i.item_id).collect::<Vec<i16>>())
            .bind(not_stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(not_stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())
            .bind(not_stackable_items.iter().map(|i| i.unique_id).collect::<Vec<i64>>())
        ).await?;
        tx.commit().await
    }
}