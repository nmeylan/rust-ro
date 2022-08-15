use sqlx::Error;
use crate::Repository;
use crate::repository::model::item_model::{ItemBuySellModel};

impl Repository {
    pub async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<u32>) -> Result<Vec<ItemBuySellModel>, Error> {
        let items: Result<Vec<ItemBuySellModel>, Error> = {
            let sql = format!("SELECT id, type, price_buy, price_sell FROM `item_db` WHERE `id` {}", Self::in_clause(&ids));
            let mut query = sqlx::query_as::<_, ItemBuySellModel>(sql.as_str());
            for id in ids {
                query = query.bind(id);
            }
            query.fetch_all(&self.pool).await
        };
        if items.is_err() {
            error!("DB error: item_fetch_all_where_ids {:?}", items.as_ref().err().unwrap());
        }
        items
    }
}