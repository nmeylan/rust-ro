use async_trait::async_trait;
use sqlx::{Error, Row};

use crate::Repository;
use crate::repository::model::item_model::{GetItemModel, ItemBuySellModel, ItemModel};
use crate::server::script::Value;
use crate::repository::{ItemRepository, PgRepository};


#[async_trait]
impl ItemRepository for PgRepository {
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
                Value::String(v) => { names.push(v.clone()) }
                Value::Number(v) => { ids.push(*v) }
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
        sqlx::query_as("SELECT * FROM item_db WHERE id < 12600 OR type in ('Weapon') ORDER BY id ASC")
            .fetch_all(&self.pool).await
    }

    async fn update_script_compilation(&self, to_update: Vec<(i32, Vec<u8>, u128)>) -> Result<(), Error> {
        let mut ids: Vec<i32> = Vec::with_capacity(to_update.len());
        let mut compilation_results: Vec<Vec<u8>> = Vec::with_capacity(to_update.len());
        let mut script_hashes: Vec<[u8; 16]> = Vec::with_capacity(to_update.len());
        for (id, compilation_result, script_hash) in to_update {
            ids.push(id);
            compilation_results.push(compilation_result);
            script_hashes.push(script_hash.to_le_bytes());
        }

        let query = r#"
        UPDATE item_db
        SET
            script_compilation = data.script_compilation,
            script_compilation_hash = data.script_compilation_hash
        FROM (
            SELECT *
            FROM unnest($1::int[], $2::bytea[], $3::bytea[]) as t(id, script_compilation, script_compilation_hash)
        ) AS data
        WHERE item_db.id = data.id
    "#;

        sqlx::query(query)
            .bind(&ids)
            .bind(&compilation_results)
            .bind(&script_hashes)
            .execute(&self.pool)
            .await.map(|_| ())
    }
}