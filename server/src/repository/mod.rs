pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;

use sqlx::{MySqlPool, Pool, MySql, Error, Type, Encode, Database, Row, FromRow};
use sqlx::mysql::MySqlRow;
use tokio::runtime::Runtime;
use crate::repository::model::item_model::ItemBuySellModel;
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

    pub async fn select_many<'q, O, B>(&self, query: &'q str, binds: Vec<B>) -> Result<Vec<O>, Error>
    where
        O: for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
        B: 'q + Send +  Encode<'q, MySql> + Type<MySql>
    {
        let result: Result<Vec<O>, Error> = {
            let mut query = sqlx::query_as::<_, O>(query);
            for bind in binds {
                query = query.bind(bind);
            }
            query.fetch_all(&self.pool).await
        };
        if result.is_err() {
            error!("DB error: select_many: {} {:?}", query, result.as_ref().err().unwrap());
        }
        result
    }
}