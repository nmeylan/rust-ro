use sqlx::{MySqlPool, MySql, Pool, Database};


pub struct Repository<T: Database> {
    pub pool: Pool<T>,
}

impl <T: Database> Repository<T> {
    pub async fn new_mysql() -> Repository<MySql> {
        let pool = MySqlPool::connect("mysql://root:root@localhost:3306/ragnarok").await.unwrap();
        Repository {
            pool
        }
    }
}