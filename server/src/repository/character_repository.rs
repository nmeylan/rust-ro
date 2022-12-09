use sqlx::{Error};
use crate::Repository;
use crate::repository::model::item_model::InventoryItemUpdate;

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

    pub async fn character_inventory_update(&self, char_id: u32, inventory_update_items: &Vec<InventoryItemUpdate>) -> Result<Vec<(u32, u32)>, Error> {
        // let mut sql = "INSERT INTO table_name (id, char_id, nameid, amount, identify) VALUES".to_string();
        // inventory_update_items.iter().for_each(|item| sql = format!("{} ({}, {},{},{},{}),", sql, item.id.map(|id| id.to_string()).or_else(|| Some("null".to_string())).unwrap(), char_id, item.item_id, item.amount, 1));
        // sql = format!("{} ON DUPLICATE KEY UPDATE", sql);
        // inventory_update_items.iter().for_each(|item| sql = format!("{} amount += {},", sql, item.amount));
        // println!("{}", sql);
        Ok(vec![])
    }
}