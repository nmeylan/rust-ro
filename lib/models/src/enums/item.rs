#![allow(dead_code)]

use crate::enums::item::ItemType::{DelayConsume, Usable};
use crate::enums::*;

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(WithMaskValueU64, Debug, Copy, Clone, PartialEq, Eq)]
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
    AccessoryLeft,
    #[mask_value = 128]
    AccessoryRight,
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

#[derive(WithMaskValueU64)]
pub enum ItemClass {
    #[mask_value = 1]
    Normal,
    Upper,
    Baby,
    #[mask_all]
    All,
}

#[derive(WithMaskValueU64)]
pub enum ItemFlag {
    #[mask_value = 1]
    BuyingStore,
    DeadBranch,
    Container,
    UniqueId,
    BindOnEquip,
    DropAnnounce,
    NoConsume,
    DropEffect,
}

#[derive(WithMaskValueU64)]
pub enum ItemTradeFlag {
    #[mask_value = 1]
    NoDrop,
    NoTrade,
    TradePartner,
    NoSell,
    NoCart,
    NoStorage,
    NoGuildStorage,
    NoMail,
    NoAuction,
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
            &_ => Self::Unknown,
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
            ItemType::Healing
            | ItemType::Usable
            | ItemType::Etc
            | ItemType::Card
            | ItemType::Ammo
            | ItemType::DelayConsume => true,
            ItemType::Unknown
            | ItemType::Armor
            | ItemType::Weapon
            | ItemType::PetEgg
            | ItemType::PetArmor
            | ItemType::Unknown2
            | ItemType::ShadowGear
            | ItemType::Cash
            | ItemType::Max => false,
        }
    }

    pub fn is_consumable(&self) -> bool {
        matches!(
            self,
            ItemType::Healing
                | ItemType::Usable
                | ItemType::PetEgg
                | ItemType::DelayConsume
                | ItemType::Cash
        )
    }
    pub fn is_equipment(&self) -> bool {
        matches!(
            self,
            ItemType::Armor | ItemType::Weapon | ItemType::PetArmor | ItemType::ShadowGear
        )
    }
    pub fn is_wearable(&self) -> bool {
        matches!(
            self,
            ItemType::Armor | ItemType::Weapon | ItemType::Ammo | ItemType::ShadowGear
        )
    }
    pub fn should_be_identified_when_dropped(&self) -> bool {
        matches!(
            self,
            ItemType::Armor | ItemType::Weapon | ItemType::Ammo | ItemType::ShadowGear
        )
    }

    #[inline]
    pub fn is_card(&self) -> bool {
        matches!(self, ItemType::Card)
    }
    pub fn is_etc(&self) -> bool {
        matches!(
            self,
            ItemType::Etc | ItemType::Card | ItemType::Unknown2 | ItemType::Ammo | ItemType::Max
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, WithNumberValue)]
pub enum ItemGroup {
    #[value = 0]
    Bluebox,
    Violetbox,
    Cardalbum,
    Giftbox,
    Scrollbox,
    Findingore,
    Cookiebag,
    Firstaid,
    Herb,
    Fruit,
    Meat,
    Candy,
    Juice,
    Fish,
    Box,
    Gemstone,
    Resist,
    Ore,
    Food,
    Recovery,
    Mineral,
    Taming,
    Scroll,
    Quiver,
    Mask,
    Accesory,
    Jewel,
    Potion,
}
