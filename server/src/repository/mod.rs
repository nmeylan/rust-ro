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
use crate::server::core::configuration::DatabaseConfig;
use crate::server::events::persistence_event::{DeleteItems, InventoryItemUpdate};
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
    async fn character_save_position(&self, account_id: u32, char_id: u32, map_name: String, x: u16, y: u16) -> Result<(), Error>;
    async fn character_update_status(&self, char_id: u32, db_column: String, value: u32) -> Result<(), Error>;
    async fn character_zeny_fetch(&self, char_id: u32) -> Result<i32, Error>;
}

#[async_trait]
pub trait InventoryRepository {
    async fn character_inventory_update(&self, inventory_update_items: &[InventoryItemUpdate], buy: bool) -> Result<(), Error>;
    async fn character_inventory_delete(&self, delete_items: DeleteItems) -> Result<PgQueryResult, Error>;
    async fn character_inventory_fetch(&self, char_id: i32) -> Result<Vec<InventoryItemModel>, Error>;
    async fn character_inventory_wearable_item_update(&self, items: Vec<InventoryItemModel>) -> Result<PgQueryResult, Error>;
}

#[async_trait]
pub trait ItemRepository {
    async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error>;
    async fn get_items(&self, ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error>;
    async fn get_item_script(&self, id: i32) -> Result<String, Error>;
    async fn get_weight(&self, ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error>;
    async fn get_all_items(&self) -> Result<Vec<ItemModel>, Error>;
}