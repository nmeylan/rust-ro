use async_trait::async_trait;
use models::status::KnownSkill;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;

use crate::repository::model::char_model::CharSelectModel;
use crate::repository::model::item_model::InventoryItemModel;
use crate::repository::{
    CharacterRepository, HotKeyRepository, InventoryRepository, ItemRepository, LoginRepository, MobRepository, Repository,
    ScriptVariableRepository,
};
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};
use crate::server::model::hotkey::Hotkey;

pub const MOCKED_UNKNOWN_CHAR_NAME: &str = "unknown";
pub const MOCKED_CHAR_ID: u32 = 150_001;
pub const MOCKED_ACCOUNT_ID: u32 = 2_000_001;

#[derive(Default)]
pub struct MockedRepository;

impl Repository for MockedRepository {}

#[async_trait]
impl HotKeyRepository for MockedRepository {
    async fn load_hotkeys(&self, _char_id: u32) -> Result<Vec<Hotkey>, Error> {
        Ok(vec![])
    }
}

impl ItemRepository for MockedRepository {}

impl MobRepository for MockedRepository {}

impl ScriptVariableRepository for MockedRepository {}

impl LoginRepository for MockedRepository {}

#[async_trait]
impl InventoryRepository for MockedRepository {
    async fn character_inventory_update_add(&self, _inventory_update_items: &[InventoryItemUpdate], _buy: bool) -> Result<(), Error> {
        Ok(())
    }

    async fn character_inventory_update_remove(
        &self,
        _inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>,
        _sell: bool,
    ) -> Result<(), Error> {
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
    async fn character_save_position(&self, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> {
        Ok(())
    }

    async fn character_update_status(&self, _char_id: u32, _db_column: String, _value: u32) -> Result<(), Error> {
        Ok(())
    }

    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> {
        Ok(Default::default())
    }

    async fn character_fetch(&self, _account_id: u32, _char_num: u8) -> Result<CharSelectModel, Error> {
        Ok(Default::default())
    }

    async fn character_with_name_fetch(&self, name: &str) -> Result<CharSelectModel, Error> {
        if name == MOCKED_UNKNOWN_CHAR_NAME {
            return Err(Error::RowNotFound);
        }
        Ok(CharSelectModel {
            char_id: MOCKED_CHAR_ID as i32,
            account_id: MOCKED_ACCOUNT_ID as i32,
            name: name.to_string(),
            last_map: "empty".to_string(),
            last_x: 10,
            last_y: 12,
            sex: "M".to_string(),
            ..Default::default()
        })
    }

    async fn character_skills(&self, _char_id: u32) -> Result<Vec<KnownSkill>, Error> {
        Ok(vec![])
    }
}
