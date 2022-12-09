pub mod model;
pub mod character_repository;
pub mod script_variable_repository;
pub mod item_repository;

use std::fmt::format;
use sqlx::{PgPool, Pool, Type, Encode, Database, Row, FromRow, Postgres, Decode, ValueRef, Error};
use sqlx::database::HasValueRef;
use sqlx::postgres::{PgPoolOptions, PgRow, PgValueRef};
use tokio::runtime::Runtime;
use crate::repository::model::item_model::ItemBuySellModel;
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

    pub fn in_clause<T>(elements: &[T]) -> String {
        format!("IN ({})", elements.iter().enumerate().map(|(i, _)| format!("${}", i + 1)).collect::<Vec<String>>().join(","))
    }

    pub async fn select_many<'q, O, B>(&self, query: &'q str, binds: Vec<B>) -> Result<Vec<O>, Error>
    where
        O: for<'r> FromRow<'r, PgRow> + Send + Unpin,
        B: 'q + Send +  Encode<'q, Postgres> + Type<Postgres>
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
