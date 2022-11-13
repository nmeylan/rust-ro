use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use rand::Rng;
use crate::server::core::position::Position;
use crate::server::enums::map_item::MapItemType;
use crate::server::npc::script::Script;
use crate::server::server::{Server, UNKNOWN_MAP_ITEM};
use crate::util::cell::MyRefMut;


#[derive(Debug, Copy, Clone)]
pub struct MapItem {
    id: u32,
    client_item_class: i16,
    object_type: MapItemType,
}

#[derive(Debug, Copy, Clone)]
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

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn map_item(&self) -> MapItem {
        self.map_item
    }
}


impl Server {
    #[inline]
    pub fn map_item_x_y(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Position> {
        match map_item.object_type() {
            MapItemType::Character => {
                let characters = self.characters.borrow();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(Position{ x: character.x(), y: character.y(), dir: 3 }); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(mob) = map_instance.get_mob(map_item.id()) {
                        return Some(Position{ x: mob.x(), y: mob.y(), dir: 3 }); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(Position{ x: warp.x(), y: warp.y(), dir: 0 });
                    }
                }
                None
            }
            MapItemType::Unknown => {
                None
            }
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(Position{ x: script.x(), y: script.y(), dir: script.dir() });
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item_name(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<String> {
        match map_item.object_type() {
            MapItemType::Character => {
                let characters = self.characters.borrow();
                if let Some(character) = characters.get(&map_item.id()) {
                    return Some(character.name.clone()); // TODO add dir to character
                }
                None
            }
            MapItemType::Mob => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(mob) = map_instance.get_mob(map_item.id()) {
                        return Some(mob.name.clone()); // TODO add dir to character
                    }
                }
                None
            }
            MapItemType::Warp => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(warp) = map_instance.get_warp(map_item.id()) {
                        return Some(warp.name.clone());
                    }
                }
                None
            }
            MapItemType::Unknown => {
                None
            }
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script.name().clone());
                    }
                }
                None
            }
        }
    }

    #[inline]
    pub fn map_item(&self, map_item: u32, map_name: &String, map_instance_id: u8) -> Option<MapItem> {
        let characters = self.characters.borrow();
        if let Some(character) = characters.get(&map_item) {
            return Some(character.to_map_item()); // TODO add dir to character
        }
        if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
            if let Some(mob) = map_instance.get_mob(map_item) {
                return Some(mob.to_map_item()); // TODO add dir to character
            }
            if let Some(warp) = map_instance.get_warp(map_item) {
                return Some(warp.to_map_item());
            }
            if let Some(script) = map_instance.get_script(map_item) {
                return Some(script.to_map_item());
            }
        }
        None
    }

    pub fn map_item_script(&self, map_item: &MapItem, map_name: &String, map_instance_id: u8) -> Option<Arc<Script>> {
        match map_item.object_type() {
            MapItemType::Npc => {
                if let Some(map_instance) = self.get_map_instance(&map_name, map_instance_id) {
                    if let Some(script) = map_instance.get_script(map_item.id()) {
                        return Some(script.clone());
                    }
                }
                None
            }
            _ => None
        }
    }


    pub fn generate_id(map_items: &mut MyRefMut<HashMap<u32, MapItem>>) -> u32 {
        let mut id: u32;
        loop {
            id = rand::thread_rng().gen::<u32>();
            if !map_items.contains_key(&id) {
                map_items.insert(id, UNKNOWN_MAP_ITEM);
                break;
            }
        }
        id
    }
}

pub trait ToMapItem {
    fn to_map_item(&self) -> MapItem;
}

pub trait ToMapItemSnapshot {
    fn to_map_item_snapshot(&self) -> MapItemSnapshot;
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