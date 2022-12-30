use sqlx::{Error, Row};
use crate::Repository;
use crate::repository::model::item_model::{GetItemModel, ItemBuySellModel};

impl Repository {
    pub async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> {
        sqlx::query_as("SELECT id, type, price_buy, price_sell, stack_amount, weight, name_english FROM item_db WHERE id IN (SELECT * FROM UNNEST($1::int4[])) ORDER BY id")
            .bind(ids)
            .fetch_all(&self.pool).await
    }

    pub async fn get_items(&self, ids: Vec<i32>) -> Result<Vec<GetItemModel>, Error> {
        sqlx::query_as("SELECT id, type, 0::int2 as amount, weight, name_english FROM item_db as amount WHERE id IN (SELECT * FROM UNNEST($1::int4[]))")
            .bind(ids)
            .fetch_all(&self.pool).await
    }

    pub async fn get_item_script(&self, id: i32) -> Result<String, Error> {
        sqlx::query("SELECT script FROM item_db WHERE id = $1 AND script is not null")
            .bind(id)
            .fetch_one(&self.pool).await
            .map(|row| row.get::<String, _>(0))
    }

    pub async fn get_weight(&self, ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error> {
        sqlx::query_as("SELECT id, weight FROM item_db WHERE id IN (SELECT * FROM UNNEST($1::int4[]))")
            .bind(ids)
            .fetch_all(&self.pool).await
    }
}