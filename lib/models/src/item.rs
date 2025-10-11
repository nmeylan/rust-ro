use accessor::GettersAll;

use crate::enums::EnumWithMaskValueU64;
use crate::enums::element::Element;
use crate::enums::item::EquipmentLocation;
use crate::enums::weapon::{AmmoType, WeaponType};
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

#[derive(Debug, Clone, Copy, GettersAll)]
pub struct WearWeapon {
    pub item_id: i32,
    pub attack: u32,
    pub level: u8,
    pub weapon_type: WeaponType,
    pub location: u64,
    pub refine: u8,
    pub element: Element,
    pub card0: i16,
    pub card1: i16,
    pub card2: i16,
    pub card3: i16,
    pub inventory_index: usize,
    pub range: u8,
}

impl WearWeapon {
    pub fn to_snapshot(&self) -> WearWeaponSnapshot {
        WearWeaponSnapshot {
            item_id: self.item_id,
            attack: self.attack,
            level: self.level,
            weapon_type: self.weapon_type,
            element: self.element,
            refine: self.refine,
            card0: self.card0,
            card1: self.card1,
            card2: self.card2,
            card3: self.card3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll)]
pub struct WearWeaponSnapshot {
    item_id: i32,
    attack: u32,
    level: u8,
    weapon_type: WeaponType,
    element: Element,
    refine: u8,
    card0: i16,
    card1: i16,
    card2: i16,
    card3: i16,
}

#[derive(Debug, Clone, Copy, GettersAll)]
pub struct WearGear {
    pub item_id: i32,
    pub level: u8,
    pub location: u64,
    pub refine: u8,
    pub card0: i16,
    pub def: i16,
    pub inventory_index: usize,
}

impl WearGear {
    pub fn to_snapshot(&self) -> WearGearSnapshot {
        WearGearSnapshot {
            item_id: self.item_id,
            level: self.level,
            refine: self.refine,
            card0: self.card0,
            def: self.def,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll)]
pub struct WearGearSnapshot {
    item_id: i32,
    level: u8,
    refine: u8,
    card0: i16,
    def: i16,
}

#[derive(Debug, Clone, Copy)]
pub struct WearAmmo {
    pub item_id: i32,
    pub inventory_index: usize,
    pub ammo_type: AmmoType,
    pub element: Element,
    pub attack: u8,
}

impl WearAmmo {
    pub fn to_snapshot(&self) -> WearAmmoSnapshot {
        WearAmmoSnapshot {
            item_id: self.item_id,
            ammo_type: self.ammo_type,
            element: self.element,
            attack: self.attack,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll)]
pub struct WearAmmoSnapshot {
    item_id: i32,
    ammo_type: AmmoType,
    element: Element,
    attack: u8,
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
