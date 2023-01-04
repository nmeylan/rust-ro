use sqlx::{Error, Executor, Row};
use sqlx::postgres::PgQueryResult;
use crate::repository::model::item_model::{InventoryItemModel};
use crate::repository::{Repository};
use crate::repository::persistence_error::PersistenceError;
use crate::server::events::persistence_event::{DeleteItems, InventoryItemUpdate};

impl Repository {
    pub async fn character_inventory_update(&self, inventory_update_items: &[InventoryItemUpdate], buy: bool) -> Result<(), Error> {
        let stackable_items = inventory_update_items.iter().filter(|item| item.stackable).collect::<Vec<&InventoryItemUpdate>>();
        let not_stackable_items = inventory_update_items.iter().filter(|item| !item.stackable).collect::<Vec<&InventoryItemUpdate>>();
        let mut tx = self.pool.begin().await.unwrap();
        let updated_item_ids_amounts: Vec<(i32, i16)> = inventory_update_items.iter().map(|item| (item.item_id, item.amount)).collect();
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identified) \
        (SELECT * FROM UNNEST($1::int4[], $2::int4[], $3::int2[], $4::bool[])) \
        ON CONFLICT (char_id, nameid, unique_id)\
         DO UPDATE set amount = inventory.amount + EXCLUDED.amount")
            .bind(stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(stackable_items.iter().map(|i| i.item_id).collect::<Vec<i32>>())
            .bind(stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())).await?;
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identified, unique_id) \
        (SELECT * FROM UNNEST($1::int4[], $2::int4[], $3::int2[], $4::bool[], $5::int8[]))")
            .bind(not_stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(not_stackable_items.iter().map(|i| i.item_id).collect::<Vec<i32>>())
            .bind(not_stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(not_stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())
            .bind(not_stackable_items.iter().map(|i| i.unique_id).collect::<Vec<i64>>())
        ).await?;
        if buy {
            let item_ids_prices = tx.fetch_all(sqlx::query("SELECT DISTINCT id, price_buy FROM item_db WHERE id IN (SELECT * FROM UNNEST($1::int4[]))")
                .bind(updated_item_ids_amounts.iter().map(|(id, _amount)| *id).collect::<Vec<i32>>())).await?;
            let cost = updated_item_ids_amounts.iter().fold(0, |mut acc, (id, amount)| {
                let price = item_ids_prices.iter().find(|item_price| item_price.get::<i32, _>(0) == *id).map_or(0, |item_price| item_price.get::<i32, _>(1));
                info!("{} cost {} zeny: {}", id, price, amount);
                acc += (*amount as i32) * price;
                acc
            });
            let updated_zeny = tx.fetch_all(sqlx::query("UPDATE char set zeny = zeny - $1 WHERE char_id = $2 RETURNING zeny;")
                .bind(cost)
                .bind(stackable_items[0].char_id)
            ).await?;
            let zeny: i32 = updated_zeny[0].get(0);
            info!("Remaining zeny {}", zeny);
            if zeny >= 0 {
                tx.commit().await
            } else {
                tx.rollback().await?;
                Err(PersistenceError::new("Rollbacking buy: not enough zeny".to_string()).into())
            }
        } else {
            tx.commit().await
        }

    }


    pub async fn character_inventory_delete(&self, delete_items: DeleteItems) -> Result<PgQueryResult, Error> {
        if delete_items.amount > 0 && delete_items.unique_id == 0 {
            sqlx::query("UPDATE inventory SET amount = $1 WHERE char_id = $2 AND id = $3 ")
                .bind(delete_items.amount)
                .bind(delete_items.char_id)
                .bind(delete_items.item_inventory_id)
            .execute(&self.pool).await
        } else {
            sqlx::query("DELETE FROM inventory WHERE char_id = $1 AND id = $2")
                .bind(delete_items.char_id)
                .bind(delete_items.item_inventory_id)
                .bind(delete_items.unique_id)
               .execute(&self.pool).await
        }
    }

    pub async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        sqlx::query_as(format!("SELECT inv.id, inv.unique_id, inv.nameid, inv.amount, inv.damaged, inv.refine, inv.identified, inv.equip, item.name_english, item.type, item.weight, inv.card0, inv.card1, inv.card2, inv.card3, {}
                            FROM inventory inv JOIN item_db item ON inv.nameid = item.id where inv.char_id = $1", Repository::item_location_fields("item")).as_str())
            .bind(char_id)
            .fetch_all(&self.pool).await
    }

    pub async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
        sqlx::query("SELECT zeny FROM char where char_id = $1")
            .bind(char_id as i32)
            .fetch_one(&self.pool).await.map(|row| Ok(row.get::<i32, _>(0)))?
    }
}