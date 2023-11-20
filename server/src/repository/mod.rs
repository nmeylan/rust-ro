pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;
pub mod inventory_repository;
pub mod persistence_error;
pub mod mob_repository;

use async_trait::async_trait;

use sqlx::{Error, PgPool};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use tokio::runtime::Runtime;
use crate::repository::model::item_model::{GetItemModel, InventoryItemModel, ItemBuySellModel, ItemModel};
use configuration::configuration::DatabaseConfig;
use models::status::KnownSkill;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};
use crate::server::script::Value;

pub struct Repository {
    pub pool: PgPool,
    pub runtime: Runtime,
}

impl Repository {
    pub async fn new_pg(configuration: &DatabaseConfig, runtime: Runtime) -> Repository {
        let connection_url = format!("postgresql://{}:{}@{}:{}/{}",
                                     configuration.username, configuration.password.as_ref().unwrap(),
                                     configuration.host, configuration.port,
                                     configuration.db);
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(&connection_url).await.unwrap();
        Repository {
            runtime,
            pool,
        }
    }
}

#[async_trait]
pub trait CharacterRepository {
    async fn character_save_position(&self, _account_id: u32, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> { todo!() }
    async fn character_update_status(&self, _char_id: u32, _db_column: String, _value: u32) -> Result<(), Error> { todo!() }
    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> { todo!() }
    async fn character_allocated_skill_points(&self, _char_id: u32) -> Result<i32, Error> { todo!() }
    async fn character_skills(&self, _char_id: u32) -> Result<Vec<KnownSkill>, Error> { todo!() }
    async fn character_reset_skills(&self, _char_id: i32, _skills: Vec<i32>) -> Result<(), Error> { todo!() }
    async fn character_allocate_skill_point(&self, _char_id: i32,  _skill_id: i32, _increment: u8) -> Result<(), Error> { todo!() }
}

#[async_trait]
pub trait InventoryRepository {
    async fn character_inventory_update_add(&self, _inventory_update_items: &[InventoryItemUpdate], _buy: bool) -> Result<(), Error> { todo!() }
    async fn character_inventory_update_remove(&self, _inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>, _sell: bool) -> Result<(), Error> { todo!() }
    async fn character_inventory_delete(&self, _delete_items: DeleteItems) -> Result<PgQueryResult, Error> { todo!() }
    async fn character_inventory_fetch(&self, _char_id: i32) -> Result<Vec<InventoryItemModel>, Error> { todo!() }
    async fn character_inventory_wearable_item_update(&self, _items: Vec<InventoryItemModel>) -> Result<PgQueryResult, Error> { todo!() }
}

#[async_trait]
pub trait ItemRepository {
    async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error>;
    async fn get_items(&self, ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error>;
    async fn get_item_script(&self, id: i32) -> Result<String, Error>;
    async fn get_weight(&self, ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error>;
    async fn get_all_items(&self) -> Result<Vec<ItemModel>, Error>;
}