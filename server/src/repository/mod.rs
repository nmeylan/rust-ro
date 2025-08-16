pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;
pub mod inventory_repository;
pub mod persistence_error;
pub mod mob_repository;
mod login_repository;
mod hotkey_repository;

use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::repository::model::item_model::{GetItemModel, InventoryItemModel, ItemBuySellModel, ItemModel};
use crate::repository::model::mob_model::MobModel;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};
use crate::server::model::hotkey::Hotkey;
use crate::server::script::Value;
use async_trait::async_trait;
use configuration::configuration::DatabaseConfig;
use models::status::{KnownSkill, Status, StatusSnapshot};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use sqlx::{Error, PgPool};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

pub struct PgRepository {
    pub pool: PgPool,
    pub runtime: Arc<Runtime>,
}

impl PgRepository {
    fn pool_options() -> PgPoolOptions {
        PgPoolOptions::new()
            .min_connections(10)
            .max_connections(10)
            .idle_timeout(Some(Duration::from_secs(60 * 60)))
            .max_lifetime(Some(Duration::from_secs(120 * 60)))
            .acquire_timeout(Duration::from_secs(5))
    }
    pub async fn new_pg(configuration: &DatabaseConfig, runtime: Arc<Runtime>) -> PgRepository {
        let connection_url = format!("postgresql://{}:{}@{}:{}/{}?keepalives=1&keepalives_idle=7200",
                                     configuration.username, configuration.password.as_ref().unwrap(),
                                     configuration.host, configuration.port,
                                     configuration.db);
        let pool = Self::pool_options()
            .connect(&connection_url).await.unwrap();
        PgRepository {
            runtime,
            pool,
        }
    }
    #[allow(dead_code)]
    pub fn new_pg_lazy(configuration: &DatabaseConfig, runtime: Arc<Runtime>) -> PgRepository {
        let connection_url = format!("postgresql://{}:{}@{}:{}/{}",
                                     configuration.username, configuration.password.as_ref().unwrap(),
                                     configuration.host, configuration.port,
                                     configuration.db);
        let pool = Self::pool_options()
            .connect_lazy(&connection_url).unwrap();
        PgRepository {
            runtime,
            pool,
        }
    }
}

pub trait Repository: Sync + Send
+ CharacterRepository
+ ItemRepository
+ InventoryRepository
+ MobRepository
+ ScriptVariableRepository
+ LoginRepository
+ HotKeyRepository
{}

impl Repository for PgRepository {}


#[async_trait]
pub trait LoginRepository {
    async fn login(&self, _username: String, _password: String) -> Result<u32, Error> { todo!() }
}

#[async_trait]
pub trait CharacterRepository {
    async fn characters_list_for_simulator(&self) -> Result<Vec<CharSelectModel>, Error> { todo!() }
    async fn character_insert(&self, _char_model: &CharInsertModel) -> Result<(), Error> { todo!() }
    async fn character_info(&self, _account_id: i32, _char_name: &str) -> Result<CharacterInfoNeoUnionWrapped, Error>{ todo!() }
    async fn characters_info(&self, _account_id: u32) -> Vec<CharacterInfoNeoUnionWrapped>{ todo!() }
    async fn character_delete_reserved(&self, _account_id: u32, _char_id: u32) -> Result<(), Error>{ todo!() }
    async fn character_save_position(&self, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> { todo!() }
    async fn character_update_status(&self, _char_id: u32, _db_column: String, _value: u32) -> Result<(), Error> { todo!() }
    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> { todo!() }
    async fn character_allocated_skill_points(&self, _char_id: u32) -> Result<i32, Error> { todo!() }
    async fn character_skills(&self, _char_id: u32) -> Result<Vec<KnownSkill>, Error> { todo!() }
    async fn character_fetch(&self, _account_id: u32, _char_num: u8) -> Result<CharSelectModel, Error> { todo!() }
    async fn character_with_id_fetch(&self, _char_id: u32) -> Result<CharSelectModel, Error> { todo!() }
    async fn character_reset_skills(&self, _char_id: i32, _skills: Vec<i32>) -> Result<(), Error> { todo!() }
    async fn character_allocate_skill_point(&self, _char_id: i32,  _skill_id: i32, _increment: u8) -> Result<(), Error> { todo!() }
    async fn characters_update(&self, statuses: Vec<&Status>, _statuses: Vec<StatusSnapshot>, _char_ids: Vec<i32>, _x: Vec<i16>, _y: Vec<i16>, _maps: Vec<String>) -> Result<(), Error> { todo!() }
    async fn character_save_temporary_bonus(&self, _char_id: u32, _account_id: u32, _temporary_bonuses: &models::status_bonus::TemporaryStatusBonuses) -> Result<(), Error> { todo!() }
    async fn character_load_temporary_bonus(&self, _char_id: u32, _account_id: u32) -> Result<models::status_bonus::TemporaryStatusBonuses, Error> { todo!() }
}

#[async_trait]
pub trait InventoryRepository {
    async fn character_inventory_update_add(&self, _inventory_update_items: &[InventoryItemUpdate], _buy: bool) -> Result<(), Error> { todo!() }
    async fn character_inventory_update_remove(&self, _inventory_update_items: &Vec<(InventoryItemModel, CharacterRemoveItem)>, _sell: bool) -> Result<(), Error> { todo!() }
    async fn character_inventory_delete(&self, _delete_items: DeleteItems) -> Result<PgQueryResult, Error> { todo!() }
    async fn character_inventory_fetch(&self, _char_id: i32) -> Result<Vec<InventoryItemModel>, Error> { todo!() }
    async fn character_inventory_wearable_item_update(&self, _items: Vec<InventoryItemModel>) -> Result<PgQueryResult, Error> { todo!() }
    async fn character_slot_card(&self, _char_id: i32, _card_inventory_item: &InventoryItemModel, _equipment_inventory_item: &InventoryItemModel) -> Result<i32, Error> { todo!() }
}

#[async_trait]
pub trait HotKeyRepository {
    async fn save_hotkeys(&self, _char_id: u32, _hotkeys: &Vec<Hotkey>) -> Result<(), Error> { todo!() }
    async fn load_hotkeys(&self, _char_id: u32) -> Result<Vec<Hotkey>, Error> { todo!() }
}


#[async_trait]
pub trait ItemRepository: Sync + Send  {
    async fn item_buy_sell_fetch_all_where_ids(&self, _ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> { todo!() }
    async fn get_items(&self, _ids_or_names: Vec<Value>) -> Result<Vec<GetItemModel>, Error> { todo!() }
    async fn get_item_script(&self, _id: i32) -> Result<String, Error> { todo!() }
    async fn get_weight(&self, _ids: Vec<i32>) -> Result<Vec<(i32, i32)>, Error> { todo!() }
    async fn get_all_items(&self) -> Result<Vec<ItemModel>, Error> { todo!() }
    async fn update_script_compilation(&self, _to_update: Vec<(i32, Vec<u8>, u128)>) -> Result<(), Error> { todo!() }
}

#[async_trait]
pub trait MobRepository {
    async fn get_all_mobs(&self) -> Result<Vec<MobModel>, Error> { todo!() }
}


pub trait ScriptVariableRepository {
    fn script_variable_char_num_save(&self, _char_id: u32, _key: String, _index: u32, _value: i32) { todo!() }
    fn script_variable_char_str_save(&self, _char_id: u32, _key: String, _index: u32, _value: String) { todo!() }
    fn script_variable_account_num_save(&self, _account_id: u32, _key: String, _index: u32, _value: i32){ todo!() }
    fn script_variable_account_str_save(&self, _account_id: u32, _key: String, _index: u32, _value: String){ todo!() }
    fn script_variable_server_num_save(&self, _varname: String, _index: u32, _value: i32) { todo!() }
    fn script_variable_server_str_save(&self, _varname: String, _index: u32, _value: String){ todo!() }
    fn script_variable_char_str_fetch_one(&self, _char_id: u32, _variable_name: String, _index: u32) -> String  { todo!() }
    fn script_variable_char_num_fetch_one(&self, _char_id: u32, _variable_name: String, _index: u32) -> i32 { todo!() }
    fn script_variable_account_str_fetch_one(&self, _account_id: u32, _variable_name: String, _index: u32) -> String { todo!() }
    fn script_variable_account_num_fetch_one(&self, _account_id: u32, _variable_name: String, _index: u32) -> i32{ todo!() }
    fn script_variable_server_str_fetch_one(&self, _variable_name: String, _index: u32) -> String{ todo!() }
    fn script_variable_server_num_fetch_one(&self, _variable_name: String, _index: u32) -> i32{ todo!() }
    fn script_variable_char_str_fetch_all(&self, _char_id: u32, _variable_name: String) -> Vec<(u32, String)>{ todo!() }
    fn script_variable_char_num_fetch_all(&self, _char_id: u32, _variable_name: String) -> Vec<(u32, i32)> { todo!() }
    fn script_variable_account_str_fetch_all(&self, _account_id: u32, _variable_name: String) ->  Vec<(u32, String)> { todo!() }
    fn script_variable_account_num_fetch_all(&self, _account_id: u32, _variable_name: String) -> Vec<(u32, i32)>{ todo!() }
    fn script_variable_server_str_fetch_all(&self, _variable_name: String) ->  Vec<(u32, String)>{ todo!() }
    fn script_variable_server_num_fetch_all(&self, _variable_name: String) -> Vec<(u32, i32)> { todo!() }
}