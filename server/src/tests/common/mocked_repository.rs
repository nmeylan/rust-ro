use async_trait::async_trait;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;
use crate::repository::{CharacterRepository, InventoryRepository};
use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::item_model::InventoryItemModel;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};

#[derive(Default)]
pub struct MockedRepository;



#[async_trait]
impl InventoryRepository for MockedRepository {
    async fn character_inventory_update_add(&self, _inventory_update_items: &[InventoryItemUpdate], _buy: bool) -> Result<(), Error> {
        Ok(())
    }

    async fn character_inventory_update_remove(&self, _inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>, _sell: bool) -> Result<(), Error> {
        Ok(())
    }

    async fn character_inventory_delete(&self, _delete_items: DeleteItems) -> Result<PgQueryResult, Error> {
        Ok(Default::default())
    }

    async fn character_inventory_fetch(&self, _char_id: i32) -> Result<Vec<InventoryItemModel>, Error> {
        Ok(Default::default())
    }

    async fn character_inventory_wearable_item_update(&self, _items: Vec<InventoryItemModel>) -> Result<PgQueryResult, Error> {
        Ok(Default::default())
    }

}

#[async_trait]
impl CharacterRepository for MockedRepository {
    async fn character_save_position(&self, _account_id: u32, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> {
        Ok(())
    }

    async fn character_update_status(&self, _char_id: u32, _db_column: String, _value: u32) -> Result<(), Error> {
        Ok(())
    }

    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> {
        Ok(Default::default())
    }

    async fn character_fetch(&self, account_id: u32, char_num: u8) -> Result<CharSelectModel, Error> {
        Ok(Default::default())
    }
}