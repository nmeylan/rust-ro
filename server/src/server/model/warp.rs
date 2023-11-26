use crate::server::model::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::model::Npc;

#[derive(SettersAll, Clone, Debug)]
pub struct Warp {
    pub map_name: String,
    pub name: String,
    pub id: u32,
    pub x: u16,
    pub y: u16,
    pub x_size: u16,
    pub y_size: u16,
    pub dest_map_name: String,
    pub to_x: u16,
    pub to_y: u16,
}

impl Default for Warp {
    fn default() -> Self {
        Self::new()
    }
}

impl Warp {
    pub fn new() -> Warp {
        Warp {
            id: 0,
            name: "".to_string(),
            map_name: "".to_string(),
            x: 0,
            y: 0,
            x_size: 0,
            y_size: 0,
            dest_map_name: "".to_string(),
            to_x: 0,
            to_y: 0
        }
    }

    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
}

impl Npc for Warp {
    fn get_map_name(&self) -> String {
        self.map_name.clone()
    }
}

impl ToMapItem for Warp {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, 45, MapItemType::Warp)
    }
}