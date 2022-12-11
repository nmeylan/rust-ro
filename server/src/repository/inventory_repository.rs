use sqlx::{Error, Executor};
use crate::repository::model::item_model::{GetItemModel, InventoryItemModel};
use crate::repository::Repository;
use crate::server::events::persistence_event::InventoryItemUpdate;

impl Repository {
    pub async fn character_inventory_update(&self, inventory_update_items: &Vec<InventoryItemUpdate>) -> Result<(), Error> {
        let stackable_items = inventory_update_items.iter().filter(|item| item.stackable).collect::<Vec<&InventoryItemUpdate>>();
        let not_stackable_items = inventory_update_items.iter().filter(|item| !item.stackable).collect::<Vec<&InventoryItemUpdate>>();

        let mut tx = self.pool.begin().await.unwrap();
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identified) \
        (SELECT * FROM UNNEST($1::int4[], $2::int2[], $3::int2[], $4::bool[])) \
        ON CONFLICT (char_id, nameid, unique_id)\
         DO UPDATE set amount = inventory.amount + (SELECT * FROM UNNEST($3::int2[]))")
            .bind(stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(stackable_items.iter().map(|i| i.item_id).collect::<Vec<i16>>())
            .bind(stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())).await?;
        tx.execute(sqlx::query("INSERT INTO inventory (char_id, nameid, amount, identified, unique_id) \
        (SELECT * FROM UNNEST($1::int4[], $2::int2[], $3::int2[], $4::bool[], $5::int8[])) ")
            .bind(not_stackable_items.iter().map(|i| i.char_id).collect::<Vec<i32>>())
            .bind(not_stackable_items.iter().map(|i| i.item_id).collect::<Vec<i16>>())
            .bind(not_stackable_items.iter().map(|i| i.amount).collect::<Vec<i16>>())
            .bind(not_stackable_items.iter().map(|i| i.identified).collect::<Vec<bool>>())
            .bind(not_stackable_items.iter().map(|i| i.unique_id).collect::<Vec<i64>>())
        ).await?;
        tx.commit().await
    }

    pub async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        sqlx::query_as("SELECT inv.id, inv.unique_id, inv.nameid, inv.amount, inv.damaged, inv.refine, inv.identified, inv.equip, item.name_english, item.type, item.weight, inv.card0, inv.card1, inv.card2, inv.card3
                            FROM inventory inv JOIN item_db item ON inv.nameid = item.id where inv.char_id = $1")
            .bind(char_id)
            .fetch_all(&self.pool).await
    }
}