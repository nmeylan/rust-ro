use sqlx::{MySqlPool, Pool, Database, MySql};
use tokio::runtime::Runtime;
use crate::server::configuration::DatabaseConfig;


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
}