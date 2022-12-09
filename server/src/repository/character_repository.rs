use sqlx::{Error};
use crate::Repository;
use crate::repository::model::item_model::InventoryItemUpdate;

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

    pub async fn character_inventory_update(&self, char_id: u32, inventory_update_items: &Vec<InventoryItemUpdate>) -> Result<Vec<(u32, u32)>, Error> {
        // let mut sql = "INSERT INTO table_name (id, char_id, nameid, amount, identify) VALUES".to_string();
        // inventory_update_items.iter().for_each(|item| sql = format!("{} ({}, {},{},{},{}),", sql, item.id.map(|id| id.to_string()).or_else(|| Some("null".to_string())).unwrap(), char_id, item.item_id, item.amount, 1));
        // sql = format!("{} ON DUPLICATE KEY UPDATE", sql);
        // inventory_update_items.iter().for_each(|item| sql = format!("{} amount += {},", sql, item.amount));
        // println!("{}", sql);
        Ok(vec![])
    }
}