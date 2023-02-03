use crate::server::model::position::Position;

pub struct EquippedItem {
    pub item_id: i32,
    pub removed_equip_location: i32,
    pub index: usize,
}

pub struct DroppedItem {
    pub map_item_id: u32,
    pub item_id: i32,
    pub location: Position,
    pub sub_location: Position,
    pub owner_id: Option<u32>,
    pub dropped_at: u128,
    pub amount: u16,
}