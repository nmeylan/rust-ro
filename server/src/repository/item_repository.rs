use std::mem;
use sqlx::{Decode, Error, Row};
use sqlx::postgres::PgRow;
use tokio::join;
use crate::Repository;
use crate::repository::model::item_model::{GetItemModel, ItemBuySellModel, ItemModel};
use crate::server::script::Value;

impl Repository {
    pub fn item_location_fields(table_prefix: &str) -> String {
        format!("(
                   (coalesce({0}.location_head_low, 0) << 0)
                   | (coalesce({0}.location_right_hand, 0) << 1)
                   | (coalesce({0}.location_garment, 0) << 2)
                   | (coalesce({0}.location_left_accessory, 0) << 3)
                   | (coalesce({0}.location_armor, 0) << 4)
                   | (coalesce({0}.location_left_hand, 0) << 5)
                   | (coalesce({0}.location_shoes, 0) << 6)
                   | (coalesce({0}.location_right_accessory, 0) << 7)
                   | (coalesce({0}.location_head_top, 0)::int4 << 8)
                   | (coalesce({0}.location_head_mid, 0)::int4 << 9)
                   | (coalesce({0}.location_costume_head_top, 0)::int4 << 10)
                   | (coalesce({0}.location_costume_head_mid, 0)::int4 << 11)
                   | (coalesce({0}.location_costume_head_low, 0)::int4 << 12)
                   | (coalesce({0}.location_costume_garment, 0)::int4 << 13)
                   | (coalesce({0}.location_ammo, 0)::int4 << 15)
                   | (coalesce({0}.location_shadow_armor, 0)::int4 << 16)
                   | (coalesce({0}.location_shadow_weapon, 0)::int4 << 17)
                   | (coalesce({0}.location_shadow_shield, 0)::int4 << 18)
                   | (coalesce({0}.location_shadow_shoes, 0)::int4 << 19)
                   | (coalesce({0}.location_shadow_right_accessory, 0)::int4 << 20)
                   | (coalesce({0}.location_shadow_left_accessory, 0)::int4 << 21)
           ) as location", table_prefix)
    }
    pub async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> {
        sqlx::query_as(format!("SELECT id, type, price_buy, price_sell, stack_amount, weight, name_english, {} FROM item_db item WHERE id IN (SELECT * FROM UNNEST($1::int4[])) ORDER BY id", Self::item_location_fields("item")).as_str())
            .bind(ids)
            .fetch_all(&self.pool).await
    }

    pub async fn get_items(&self, ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error> {
        let mut names = vec![];
        let mut ids = vec![];
        ids_or_names.iter().for_each(|id| {
            match id {
                Value::String(v) => {names.push(v.clone())}
                Value::Number(v) => {ids.push(*v)}
            }
        });
        sqlx::query_as::<_, GetItemModel>(format!("SELECT id, type, 0::int2 as amount, weight, name_english, name_aegis, {} FROM item_db item WHERE id IN (SELECT * FROM UNNEST($1::int4[])) OR name_aegis ILIKE ANY($2::text[])", Self::item_location_fields("item")).as_str())
            .bind(ids)
            .bind(names)
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

    pub async fn get_item_full(&self, id: i32) -> Result<ItemModel, Error> {
        sqlx::query_as("SELECT * FROM item_db WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool).await
    }

    pub async fn get_items_full(&self, ids: Vec<i32>) -> Result<Vec<ItemModel>, Error> {
        sqlx::query_as("SELECT * FROM item_db WHERE id IN (SELECT * FROM UNNEST($1::int4[]))")
            .bind(ids)
            .fetch_all(&self.pool).await
    }
}