use enums::EnumWithMaskValueU64;
use enums::item::EquipmentLocation;
use enums::weapon::{AmmoType, WeaponType};
use crate::position::Position;


pub struct EquippedItem {
    pub item_id: i32,
    pub location: u64,
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
    pub is_identified: bool,
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


#[derive(Debug, Clone)]
pub struct NormalInventoryItem {
    pub item_id: i32,
    pub amount: i16,
    pub name_english: String,
}

#[derive(Debug, Clone, Copy)]
pub struct WearWeapon {
    pub item_id: i32,
    pub attack: u32,
    pub level: u8,
    pub weapon_type: WeaponType,
    pub location: u64,
    pub refine: i16,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
    pub inventory_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct WearGear {
    pub item_id: i32,
    pub level: u8,
    pub location: u64,
    pub refine: i16,
    pub card0: i16,
    pub def: i16,
    pub inventory_index: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct WearAmmo {
    pub item_id: i32,
    pub inventory_index: usize,
    pub ammo_type: AmmoType,
}

pub trait Wearable {
    fn location(&self) -> u64;
    fn item_id(&self) -> i32;
}

impl Wearable for WearGear {
    fn location(&self) -> u64 {
        self.location
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}
impl Wearable for WearWeapon {
    fn location(&self) -> u64 {
        self.location
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}

impl Wearable for EquippedItem {
    fn location(&self) -> u64 {
        self.location
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}

impl Wearable for WearAmmo {
    fn location(&self) -> u64 {
        EquipmentLocation::Ammo.as_flag()
    }

    fn item_id(&self) -> i32 {
        self.item_id
    }
}