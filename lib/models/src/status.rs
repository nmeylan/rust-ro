use accessor::SettersAll;
use enums::EnumWithMaskValueU64;
use enums::item::EquipmentLocation;
use crate::item::{Wearable, WearGear, WearWeapon};


#[derive(SettersAll, Debug, Default, Clone)]
pub struct Status {
    pub job: u32,
    pub hp: u32,
    pub sp: u32,
    pub max_hp: u32,
    pub max_sp: u32,
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
    pub base_atk: u32,
    pub matk_min: u32,
    pub matk_max: u32,
    pub speed: u16,
    pub hit: u32,
    pub flee: u32,
    pub crit: u32,
    pub def: u32,
    pub mdef: u32,
    pub look: Option<Look>,
    pub zeny: u32,
    pub base_level: u32,
    pub job_level: u32,
    pub status_point: u32,
    pub skill_point: u32,
    pub base_exp: u32,
    pub job_exp: u32,
    pub weapons: Vec<WearWeapon>,
    pub equipments: Vec<WearGear>
}

impl Status {
    pub fn right_hand_weapon(&self) -> Option<&WearWeapon> {
        self.weapons.iter().find(|w| w.location & EquipmentLocation::HandRight.as_flag() > 0)
    }

    pub fn equipped_gears(&self) -> &Vec<WearGear> {
        &self.equipments
    }

    pub fn equipped_weapons(&self) -> &Vec<WearWeapon> {
        &self.weapons
    }

    pub fn takeoff_weapon(&mut self, inventory_index: usize) {
        self.weapons.retain(|w| w.inventory_index != inventory_index);
    }

    pub fn wear_weapon(&mut self, wear_weapon: WearWeapon) {
        self.weapons.push(wear_weapon);
    }

    pub fn takeoff_equipment(&mut self, inventory_index: usize) {
        self.equipments.retain(|w| w.inventory_index != inventory_index);
    }

    pub fn wear_equipment(&mut self, wear_weapon: WearGear) {
        self.equipments.push(wear_weapon);
    }

    pub fn all_equipped_items(&self) -> Vec<&dyn Wearable> {
        let mut equipments = self.equipped_gears().iter().map(|e| e as &dyn Wearable).collect::<Vec<&dyn Wearable>>();
        let weapons = self.equipped_weapons().iter().map(|e| e as &dyn Wearable).collect::<Vec<&dyn Wearable>>();
        equipments.extend(weapons);
        equipments
    }
}


#[derive(SettersAll, Debug, Clone, Copy, Default)]
pub struct Look {
    pub hair: u16,
    pub hair_color: u32,
    pub clothes_color: u32,
    pub body: u32,
    pub weapon: u32,
    pub shield: u32,
    pub head_top: u32,
    pub head_middle: u32,
    pub head_bottom: u32,
    pub robe: u32,
}
