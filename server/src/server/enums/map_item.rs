use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MapItemType {
    Character,
    Mob,
    Warp,
    Unknown
}

impl Display for MapItemType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl MapItemType {
    pub fn value(&self) -> i16 {
        match *self {
            MapItemType::Character => 1,
            MapItemType::Mob => 5,
            MapItemType::Warp => 6,
            MapItemType::Unknown => 0,
        }
    }

    pub fn from(value: i16) -> MapItemType {
        match value {
            1 => MapItemType::Character,
            5 => MapItemType::Mob,
            6 => MapItemType::Warp,
            _ => MapItemType::Unknown
        }
    }
}