use async_trait::async_trait;
use sqlx::{Error, Row};

use crate::Repository;
use crate::repository::model::item_model::{GetItemModel, ItemBuySellModel, ItemModel};
use crate::server::script::Value;
use crate::repository::ItemRepository;


#[async_trait]
impl ItemRepository for Repository {
    async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> {
        sqlx::query_as("SELECT id, type, price_buy, price_sell, stack_amount, weight, name_english FROM item_db item WHERE id IN (SELECT * FROM UNNEST($1::int4[])) ORDER BY id")
            .bind(ids)
            .fetch_all(&self.pool).await
    }

    async fn get_items(&self, ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error> {
        let mut names = vec![];
        let mut ids = vec![];
        ids_or_names.iter().for_each(|id| {
            match id {
                Value::String(v) => {names.push(v.clone())}
                Value::Number(v) => {ids.push(*v)}
            }
        });
        sqlx::query_as::<_, GetItemModel>("SELECT id, type, 0::int2 as amount, weight, name_english, name_aegis FROM item_db item WHERE id IN (SELECT * FROM UNNEST($1::int4[])) OR name_aegis ILIKE ANY($2::text[])")
            .bind(ids)
            .bind(names)
            .fetch_all(&self.pool).await
    }

    async fn get_item_script(&self, id: i32) -> Result<String, Error> {
        sqlx::query("SELECT script FROM item_db WHERE id = $1 AND script is not null")
            .bind(id)
            .fetch_one(&self.pool).await
            .map(|row| row.get::<String, _>(0))
    }

    async fn get_weight(&self, ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error> {
        sqlx::query_as("SELECT id, weight FROM item_db WHERE id IN (SELECT * FROM UNNEST($1::int4[]))")
            .bind(ids)
            .fetch_all(&self.pool).await
    }

    async fn get_all_items(&self) -> Result<Vec<ItemModel>, Error> {
        sqlx::query_as("SELECT * FROM item_db")
            .fetch_all(&self.pool).await
    }
}