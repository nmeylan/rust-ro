pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;
pub mod inventory_repository;
pub mod persistence_error;
pub mod mob_repository;
mod login_repository;

use async_trait::async_trait;

use sqlx::{Error, PgPool};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use tokio::runtime::Runtime;
use crate::repository::model::item_model::{GetItemModel, InventoryItemModel, ItemBuySellModel, ItemModel};
use configuration::configuration::DatabaseConfig;
use models::status::KnownSkill;
use crate::repository::model::char_model::{CharInsertModel, CharSelectModel, CharacterInfoNeoUnionWrapped};
use crate::repository::model::mob_model::MobModel;
use crate::server::model::events::game_event::CharacterRemoveItem;
use crate::server::model::events::persistence_event::{DeleteItems, InventoryItemUpdate};
use crate::server::script::Value;

pub struct PgRepository {
    pub pool: PgPool,
    pub runtime: Runtime,
}

impl PgRepository {
    pub async fn new_pg(configuration: &DatabaseConfig, runtime: Runtime) -> PgRepository {
        let connection_url = format!("postgresql://{}:{}@{}:{}/{}",
                                     configuration.username, configuration.password.as_ref().unwrap(),
                                     configuration.host, configuration.port,
                                     configuration.db);
        let pool = PgPoolOptions::new()
            .min_connections(5)
            .max_connections(5)
            .connect(&connection_url).await.unwrap();
        PgRepository {
            runtime,
            pool,
        }
    }
    pub fn new_pg_lazy(configuration: &DatabaseConfig, runtime: Runtime) -> PgRepository {
        let connection_url = format!("postgresql://{}:{}@{}:{}/{}",
                                     configuration.username, configuration.password.as_ref().unwrap(),
                                     configuration.host, configuration.port,
                                     configuration.db);
        let pool = PgPoolOptions::new()
            .max_connections(20)
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
+ LoginRepository  {}

impl Repository for PgRepository {}


#[async_trait]
pub trait LoginRepository {
    async fn login(&self, _username: String, _password: String) -> Result<u32, Error> { todo!() }
}

#[async_trait]
pub trait CharacterRepository {
    async fn character_insert(&self, _char_model: &CharInsertModel) -> Result<(), Error> { todo!() }
    async fn character_info(&self, _account_id: i32, _char_name: &str) -> Result<CharacterInfoNeoUnionWrapped, Error>{ todo!() }
    async fn characters_info(&self, _account_id: u32) -> Vec<CharacterInfoNeoUnionWrapped>{ todo!() }
    async fn character_delete_reserved(&self, _account_id: u32, _char_id: u32) -> Result<(), Error>{ todo!() }
    async fn character_save_position(&self, _account_id: u32, _char_id: u32, _map_name: String, _x: u16, _y: u16) -> Result<(), Error> { todo!() }
    async fn character_update_status(&self, _char_id: u32, _db_column: String, _value: u32) -> Result<(), Error> { todo!() }
    async fn character_zeny_fetch(&self, _char_id: u32) -> Result<i32, Error> { todo!() }
    async fn character_allocated_skill_points(&self, _char_id: u32) -> Result<i32, Error> { todo!() }
    async fn character_skills(&self, _char_id: u32) -> Result<Vec<KnownSkill>, Error> { todo!() }
    async fn character_fetch(&self, _account_id: u32, _char_num: u8) -> Result<CharSelectModel, Error> { todo!() }
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
    fn script_variable_char_str_save(&self, _char_id: u32, key: String, index: u32, _value: String) { todo!() }
    fn script_variable_account_num_save(&self, _account_id: u32, _key: String, index: u32, _value: i32){ todo!() }
    fn script_variable_account_str_save(&self, _account_id: u32, _key: String, index: u32, _value: String){ todo!() }
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