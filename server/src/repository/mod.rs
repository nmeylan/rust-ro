pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;
pub mod inventory_repository;
pub mod persistence_error;

use sqlx::{PgPool};
use sqlx::postgres::{PgPoolOptions};
use tokio::runtime::Runtime;
use crate::server::core::configuration::DatabaseConfig;

pub struct Repository {
    pub pool: PgPool,
    pub runtime: Runtime
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
            pool
        }
    }
}
