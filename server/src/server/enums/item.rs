use crate::server::enums::item::ItemType::{DelayConsume, Usable};

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
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
            return Usable.value()
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
        match self {
            ItemType::Healing | ItemType::Usable | ItemType::PetEgg | ItemType::DelayConsume | ItemType::Cash => { true }
            _ => false
        }
    }
    pub fn is_equipment(&self) -> bool {
        match self {
            ItemType::Armor | ItemType::Weapon | ItemType::PetArmor | ItemType::ShadowGear => { true }
            _ => false
        }
    }
    pub fn is_etc(&self) -> bool {
        match self {
            ItemType::Etc | ItemType::Card | ItemType::Unknown2 | ItemType::Ammo | ItemType::Max => { true }
            _ => false
        }
    }
}