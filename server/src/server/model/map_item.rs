use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::server::model::position::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MapItemType {
    Character,
    Mob,
    Warp,
    Unknown,
    Npc,
    DroppedItem
}

impl Display for MapItemType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl MapItemType {
    pub fn value(&self) -> i16 {
        match *self {
            MapItemType::Character => 1,
            MapItemType::Mob => 5,
            MapItemType::Warp => 6,
            MapItemType::Npc => 6,
            MapItemType::Unknown => 0,
            MapItemType::DroppedItem => 0
        }
    }

}


#[derive(Debug, Copy, Clone)]
pub struct MapItem {
    id: u32,
    client_item_class: i16,
    object_type: MapItemType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MapItemSnapshot {
    map_item: MapItem,
    position: Position,
}

impl MapItem {
    pub const fn unknown() -> Self {
        Self {
            id: 0,
            client_item_class: 0,
            object_type: MapItemType::Unknown,
        }
    }
    pub fn new(id: u32, client_item_class: i16, object_type: MapItemType) -> Self {
        Self {
            id,
            client_item_class,
            object_type,
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn client_item_class(&self) -> i16 {
        self.client_item_class
    }
    pub fn object_type(&self) -> &MapItemType {
        &self.object_type
    }
    pub fn object_type_value(&self) -> i16 {
        self.object_type.value()
    }
}

impl MapItemSnapshot {
    pub fn new(map_item: MapItem, position: Position) -> Self {
        Self {
            map_item,
            position
        }
    }

    pub fn x(&self) -> u16 {
        self.position.x
    }

    pub fn y(&self) -> u16 {
        self.position.y
    }
    #[allow(dead_code)]
    pub fn position(&self) -> Position {
        self.position
    }

    pub fn map_item(&self) -> MapItem {
        self.map_item
    }
}

impl Hash for MapItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq<Self> for MapItem {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for MapItem{}


pub trait ToMapItem {
    fn to_map_item(&self) -> MapItem;
}

pub trait ToMapItemSnapshot {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot;
}