use crate::server::enums::item::ItemType;

pub struct InventoryItem {
    pub item_id: u32,
    pub item_type: ItemType,
    pub amount: u16,
    pub weight: u16,
    pub name_english: String
}