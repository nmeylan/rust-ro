use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::SeqCst;
use models::position::Position;
use crate::util::hasher::NoopHasherU32;
use hashbrown::HashMap;

pub const UNKNOWN_MAP_ITEM: MapItem = MapItem::unknown();
pub const CHARACTER_MAX_MAP_ITEM_ID: u32 = 300000;
pub const MAP_INSTANCE_MAX_MAP_ITEM_ID: u32 = 100000;

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
    client_id: u32,
    generic_internal_id: u32,
    specific_internal_id: u32,
    client_item_class: i16,
    object_type: MapItemType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MapItemSnapshot {
    pub(crate) map_item: MapItem,
    pub(crate) position: Position
}

impl MapItem {
    pub const fn unknown() -> Self {
        Self {
            client_id: 0,
            generic_internal_id: 0,
            specific_internal_id: 0,
            client_item_class: 0,
            object_type: MapItemType::Unknown,
        }
    }
    pub fn new(client_id: u32, generic_internal_id: u32, specific_internal_id: u32, client_item_class: i16, object_type: MapItemType) -> Self {
        Self {
            client_id,
            generic_internal_id,
            specific_internal_id,
            client_item_class,
            object_type,
        }
    }
    pub fn client_id(&self) -> u32 {
        self.client_id
    }
    pub fn generic_internal_id(&self) -> u32 {
        self.generic_internal_id
    }
    pub fn specific_internal_id(&self) -> u32 {
        self.specific_internal_id
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
        self.client_id().hash(state);
    }
}

impl PartialEq<Self> for MapItem {
    fn eq(&self, other: &Self) -> bool {
        self.client_id() == other.client_id()
    }
}

impl Eq for MapItem{}


pub trait ToMapItem {
    fn to_map_item(&self) -> MapItem;
}

pub trait ToMapItemSnapshot {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot;
}

pub struct MapItems {
    deprecated_items: HashMap<u32, MapItem, NoopHasherU32>,
    items: Vec<MapItem>,
    ids_pool: VecDeque<u32>,
    sequence: AtomicU32,
    sequence_start: u32,
    sequence_max: u32,
}

impl MapItems {
    pub fn new(sequence_start: u32, sequence_max: u32) -> Self {
        Self {
            deprecated_items: HashMap::<u32, MapItem, NoopHasherU32>::with_hasher(NoopHasherU32::default()),
            items: vec![],
            ids_pool: Default::default(),
            sequence: AtomicU32::new(0),
            sequence_start,
            sequence_max,
        }
    }

    pub fn generate_internal_id_deprecated(&mut self) -> u32 {
        if let Some(recycled_id) = self.ids_pool.pop_front() {
            recycled_id
        } else {
            self.sequence.fetch_add(1, SeqCst)
        }
    }

    pub fn generate_id(&mut self) -> (u32, u32) {
        let internal_id = if let Some(recycled_id) = self.ids_pool.pop_front() {
            recycled_id
        } else {
            self.sequence.fetch_add(1, SeqCst)
        };
        (internal_id + self.sequence_start, internal_id)
    }

    pub fn get_deprecated(&self) -> &HashMap<u32, MapItem, NoopHasherU32> {
        &self.deprecated_items
    }

    pub fn get(&self) -> &Vec<MapItem> {
        &self.items
    }

    pub fn insert_deprecated(&mut self, id: u32, map_item: MapItem) {
        self.deprecated_items.insert_unique_unchecked(id, map_item);
    }

    pub fn insert(&mut self, mut map_item: MapItem) {
        if map_item.generic_internal_id == 0 {
            let (id, internal_id) = self.generate_id();
            map_item.generic_internal_id = internal_id;
            if map_item.client_id == 0 {
                map_item.client_id = id;
            }
        }
        self.items.insert(map_item.generic_internal_id as usize, map_item);
    }

    pub fn remove_deprecated(&mut self, internal_id: u32) {
        self.deprecated_items.remove(&internal_id);
        self.ids_pool.push_back(internal_id);
    }

    pub fn remove(&mut self, internal_id: u32) {
        self.items.remove(internal_id as usize);
        self.ids_pool.push_back(internal_id);
    }

    pub fn remove_with_id(&mut self, id: u32) {
        if let Some(internal_id) = self.items.iter().find(|i| i.client_id == id).map(|i| i.generic_internal_id) {
            self.items.remove(internal_id as usize);
            self.ids_pool.push_back(internal_id);
        }
    }
}