#![allow(dead_code)]

use sqlx::{Decode, Postgres};
use sqlx::database::HasValueRef;
use sqlx::error::BoxDynError;
use sqlx::TypeInfo;
use crate::*;
use crate::item::ItemType::{DelayConsume, Usable};

#[derive(WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ItemType {
    #[value = 0]
    Healing,
    Unknown,
    Usable,
    Etc,
    Armor,
    Weapon,
    Card,
    PetEgg,
    PetArmor,
    Unknown2,
    Ammo,
    DelayConsume,
    ShadowGear,
    #[value = 18]
    Cash,
    Max,
}

#[derive(WithMaskValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum EquipmentLocation {
    #[mask_value = 1]
    HeadLow,
    #[mask_value = 512]
    HeadMid,
    #[mask_value = 256]
    HeadTop,
    #[mask_value = 2]
    HandRight,
    #[mask_value = 32]
    HandLeft,
    #[mask_value = 16]
    Armor,
    #[mask_value = 64]
    Shoes,
    #[mask_value = 4]
    Garment,
    #[mask_value = 8]
    AccessoryRight,
    #[mask_value = 128]
    AccessoryLeft,
    #[mask_value = 1024]
    CostumeHeadTop,
    #[mask_value = 2048]
    CostumeHeadMid,
    #[mask_value = 4096]
    CostumeHeadLow,
    #[mask_value = 8192]
    CostumeGarment,
    #[mask_value = 32768]
    Ammo,
    #[mask_value = 65536]
    ShadowArmor,
    #[mask_value = 131072]
    ShadowWeapon,
    #[mask_value = 262144]
    ShadowShield,
    #[mask_value = 524288]
    ShadowShoes,
    #[mask_value = 1048576]
    ShadowAccR,
    #[mask_value = 2097152]
    ShadowAccL,
    // Acc_R|Acc_L
    #[mask_value = 136]
    Accessory,
}

impl ItemType {
    pub fn from_string(value: &str) -> Self {
        match value {
            "Healing" => Self::Healing,
            "Unknown" => Self::Unknown,
            "Usable" => Self::Usable,
            "Etc" => Self::Etc,
            "Armor" => Self::Armor,
            "Weapon" => Self::Weapon,
            "Card" => Self::Card,
            "PetEgg" => Self::PetEgg,
            "PetArmor" => Self::PetArmor,
            "Unknown2" => Self::Unknown2,
            "Ammo" => Self::Ammo,
            "Delayconsume" => Self::DelayConsume,
            "Shadowgear" => Self::ShadowGear,
            "Cash" => Self::Cash,
            "Max" => Self::Max,
            &_ => Self::Unknown
        }
    }

    pub fn to_client_type(&self) -> usize {
        if *self == DelayConsume {
            return Usable.value();
        }
        self.value()
    }

    pub fn is_stackable(&self) -> bool {
        match self {
            ItemType::Healing | ItemType::Usable | ItemType::Etc | ItemType::Card | ItemType::Ammo | ItemType::DelayConsume => {
                true
            }
            ItemType::Unknown | ItemType::Armor | ItemType::Weapon | ItemType::PetEgg | ItemType::PetArmor | ItemType::Unknown2 | ItemType::ShadowGear | ItemType::Cash | ItemType::Max => {
                false
            }
        }
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self, ItemType::Healing | ItemType::Usable | ItemType::PetEgg | ItemType::DelayConsume | ItemType::Cash)
    }
    pub fn is_equipment(&self) -> bool {
        matches!(self, ItemType::Armor | ItemType::Weapon | ItemType::PetArmor | ItemType::ShadowGear)
    }
    pub fn is_etc(&self) -> bool {
        matches!(self, ItemType::Etc | ItemType::Card | ItemType::Unknown2 | ItemType::Ammo | ItemType::Max)
    }
}

impl<'r> Decode<'r, Postgres> for ItemType {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        Ok(ItemType::from_string(value))
    }
}

impl sqlx::Type<Postgres> for ItemType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <&str as sqlx::Type<sqlx::Postgres>>::type_info()
    }


    fn compatible(ty: &<sqlx::Postgres as sqlx::Database>::TypeInfo) -> bool {
        ty.name() == "VARCHAR"
    }
}


impl<'r> Decode<'r, Postgres> for EquipmentLocation {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<Postgres>>::decode(value)?;
        Err("Not implemented".into())
    }
}