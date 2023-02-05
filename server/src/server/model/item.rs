use crate::server::model::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::model::position::Position;

pub struct EquippedItem {
    pub item_id: i32,
    pub removed_equip_location: i32,
    pub index: usize,
}

#[derive(Clone, Debug, Copy)]
pub struct DroppedItem {
    pub map_item_id: u32,
    pub item_id: i32,
    pub location: Position,
    pub sub_location: Position,
    pub owner_id: Option<u32>,
    pub dropped_at: u128,
    pub amount: u16,
}

impl DroppedItem {
    pub fn item_id(&self) -> i32 {
        self.item_id
    }
    pub fn x(&self) -> u16 {
        self.location.x
    }
    pub fn y(&self) -> u16 {
        self.location.y
    }
}

impl ToMapItem for DroppedItem {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.map_item_id, self.item_id as i16, MapItemType::DroppedItem)
    }
}