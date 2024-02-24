use models::item::DroppedItem;
use crate::server::model::map_item::{MapItem, MapItemType, ToMapItem};

impl ToMapItem for DroppedItem {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.map_item_id, self.map_item_id, 0, self.item_id as i16, MapItemType::DroppedItem)
    }
}