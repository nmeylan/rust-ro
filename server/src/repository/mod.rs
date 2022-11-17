pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;

use sqlx::{MySqlPool, Pool, MySql};
use tokio::runtime::Runtime;
use crate::server::core::configuration::DatabaseConfig;

pub struct Repository {
    pub pool: Pool<MySql>,
    pub runtime: Runtime
}

impl Repository {
    pub async fn new_mysql(configuration: &DatabaseConfig, runtime: Runtime) -> Repository {
        let connection_url = format!("mysql://{}:{}@{}:{}/{}",
                                     configuration.username, configuration.password.as_ref().unwrap(),
                                     configuration.host, configuration.port,
                                     configuration.db);
        let pool = MySqlPool::connect(
            &connection_url).await.unwrap();
        Repository {
            runtime,
            pool
        }
    }

    pub fn in_clause<T>(elements: &[T]) -> String {
        format!("IN ({})", elements.iter().map(|_| "?".to_string()).collect::<Vec<String>>().join(","))
    }
}