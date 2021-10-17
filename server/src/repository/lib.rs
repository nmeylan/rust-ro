use sqlx::{MySqlPool, MySql, Pool, Database};
use crate::server::configuration::DatabaseConfig;


pub struct Repository<T: Database> {
    pub pool: Pool<T>,
}

impl <T: Database> Repository<T> {
    pub async fn new_mysql(configuration: &DatabaseConfig) -> Repository<MySql> {
        let connection_url = format!("mysql://{}:{}@{}:{}/{}",
                             configuration.username, configuration.password.as_ref().unwrap(),
                             configuration.host, configuration.port,
                             configuration.db);
        let pool = MySqlPool::connect(
            &connection_url).await.unwrap();
        Repository {
            pool
        }
    }
}