use rand::RngCore;

use crate::repository::model::item_model::{DBItemType, InventoryItemModel};
use crate::server::service::global_config_service::GlobalConfigService;

pub fn create_inventory_item(item_name: &str, amount: i16) -> InventoryItemModel {
    let item = GlobalConfigService::instance().get_item_by_name(item_name);
    let mut rng = rand::thread_rng();
    InventoryItemModel {
        id: rng.next_u32() as i32,
        unique_id: 0,
        item_id: item.id,
        item_type: DBItemType::from_type(item.item_type),
        amount,
        refine: 0,
        is_identified: false,
        equip: 0,
        is_damaged: false,
        card0: 0,
        card1: 0,
        card2: 0,
        card3: 0,
        name_english: item.name_english.clone(),
        weight: item.weight,
    }
}
