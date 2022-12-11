use crate::server::enums::item::ItemType;

#[derive(Clone)]
pub struct InventoryItem {
    pub item_id: u16,
    pub item_type: ItemType,
    pub amount: u16,
    pub weight: u16,
    pub name_english: String,
    pub is_identified: bool
}