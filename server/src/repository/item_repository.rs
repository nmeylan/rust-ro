use sqlx::Error;
use crate::Repository;
use crate::repository::model::item_model::{ItemBuySellModel};

impl Repository {
    pub async fn item_buy_sell_fetch_all_where_ids(&self, ids: Vec<i32>) -> Result<Vec<ItemBuySellModel>, Error> {
        self.select_many(format!("SELECT id, type, price_buy, price_sell, stack_amount, weight, name_english FROM item_db WHERE id {}", Self::in_clause(&ids)).as_str(), ids).await
    }
}