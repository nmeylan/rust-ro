use async_trait::async_trait;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;
use crate::repository::{CharacterRepository, InventoryRepository};
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};

pub struct MockedRepository;

impl Default for MockedRepository {
    fn default() -> Self {
        Self {}
    }
}

#[async_trait]
impl InventoryRepository for MockedRepository {
    async fn character_inventory_update(&self, inventory_update_items: &[InventoryItemUpdate], buy: bool) -> Result<(), Error> {
        Ok(())
    }

    async fn character_inventory_delete(&self, delete_items: DeleteItems) -> Result<PgQueryResult, Error> {
        Ok(Default::default())
    }

    async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        Ok(Default::default())
    }

    async fn character_inventory_wearable_item_update(&self, items: Vec<InventoryItemModel>) -> Result<PgQueryResult, Error> {
        Ok(Default::default())
    }
}

#[async_trait]
impl CharacterRepository for MockedRepository {
    async fn character_save_position(&self, account_id: u32, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error> {
        Ok(())
    }

    async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error> {
        Ok(())
    }

    async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error> {
        Ok(Default::default())
    }
}