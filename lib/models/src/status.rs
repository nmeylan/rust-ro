use crate::item::{WearAmmo, WearGear, WearGearSnapshot, WearWeapon, Wearable, WearAmmoSnapshot, WearWeaponSnapshot};
use accessor::{GettersAll, SettersAll};
use enums::item::EquipmentLocation;
use enums::size::Size;
use enums::EnumWithMaskValueU64;
use enums::weapon::WeaponType;

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
    pub base_atk: u32, // used for mob, calculated for player
    pub matk_min: u32, // used for mob, calculated for player
    pub matk_max: u32, // used for mob, calculated for player
    pub speed: u16,
    pub hit: u32,  // used for mob, calculated for player
    pub flee: u32, // used for mob, calculated for player
    pub crit: u32, // used for mob, calculated for player
    pub def: u32,  // used for mob, calculated for player
    pub mdef: u32, // used for mob, calculated for player
    pub look: Option<Look>,
    pub zeny: u32,
    pub base_level: u32,
    pub job_level: u32,
    pub status_point: u32,
    pub skill_point: u32,
    pub base_exp: u32,
    pub job_exp: u32,
    pub state: u64,
    pub size: Size,
    pub weapons: Vec<WearWeapon>,
    pub equipments: Vec<WearGear>,
    pub ammo: Option<WearAmmo>,
    pub known_skills: Vec<KnownSkill>,
}

#[derive(Clone, Copy, PartialEq, Debug, GettersAll, SettersAll)]
pub struct StatusSnapshot {
    job: u32,
    hp: u32,
    sp: u32,
    str: u16,
    agi: u16,
    vit: u16,
    int: u16,
    dex: u16,
    luk: u16,
    base_atk: u16,
    matk_min: u16,
    matk_max: u16,
    speed: u16,
    hit: u16,
    flee: u16,
    crit: u16,
    def: u16,
    mdef: u16,
    size: Size,
    state: u64,
    zeny: u32,
    fist_atk: u16,
    atk_given_by_cards: u16,
    aspd: f32,
    right_hand_weapon: Option<WearWeaponSnapshot>,
    right_hand_weapon_type: WeaponType,
    left_hand_weapon: Option<WearWeaponSnapshot>,
    upper_headgear: Option<WearGearSnapshot>,
    middle_headgear: Option<WearGearSnapshot>,
    lower_headgear: Option<WearGearSnapshot>,
    shield: Option<WearGearSnapshot>,
    body: Option<WearGearSnapshot>,
    shoes: Option<WearGearSnapshot>,
    shoulder: Option<WearGearSnapshot>,
    accessory_left: Option<WearGearSnapshot>,
    accessory_right: Option<WearGearSnapshot>,
    ammo: Option<WearAmmoSnapshot>,
}

impl StatusSnapshot {
    pub fn from(status: &Status) -> Self {
        let mut snapshot = Self {
            job: status.job,
            hp: status.hp,
            sp: status.sp,
            str: status.str,
            agi: status.agi,
            vit: status.vit,
            int: status.int,
            dex: status.dex,
            luk: status.luk,
            base_atk: 0,
            matk_min: 0,
            matk_max: 0,
            speed: status.speed,
            hit: status.hit as u16,
            flee: status.flee as u16,
            crit: status.crit as u16,
            def: status.def as u16,
            mdef: status.mdef as u16,
            size: status.size,
            state: status.state,
            zeny: status.zeny,
            fist_atk: 0,
            atk_given_by_cards: 0,
            aspd: 0.0,
            right_hand_weapon: status.right_hand_weapon().map(|w| w.to_snapshot()),
            right_hand_weapon_type: status.right_hand_weapon().map(|w| w.weapon_type).unwrap_or(WeaponType::Fist),
            left_hand_weapon: None,
            upper_headgear: None,
            middle_headgear: None,
            lower_headgear: None,
            shield: None,
            body: None,
            shoes: None,
            shoulder: None,
            accessory_left: None,
            accessory_right: None,
            ammo: status.equipped_ammo().map(|a| a.to_snapshot()),
        };
        for gear in status.equipped_gears() {
            let gear_snapshot = Some(gear.to_snapshot());
            if gear.location & EquipmentLocation::HeadTop.as_flag() > 0 {
                snapshot.upper_headgear = gear_snapshot;
            }
            if gear.location & EquipmentLocation::HeadMid.as_flag() > 0 {
                snapshot.middle_headgear = gear_snapshot;
            }
            if gear.location & EquipmentLocation::HeadLow.as_flag() > 0 {
                snapshot.lower_headgear = gear_snapshot;
            }
            if gear.location & EquipmentLocation::Armor.as_flag() > 0 {
                snapshot.body = gear_snapshot;
            }
            if gear.location & EquipmentLocation::Shoes.as_flag() > 0 {
                snapshot.shoes = gear_snapshot;
            }
            if gear.location & EquipmentLocation::HandLeft.as_flag() > 0 {
                snapshot.shield = gear_snapshot;
            }
            if gear.location & EquipmentLocation::Garment.as_flag() > 0 {
                snapshot.shoulder = gear_snapshot;
            }
            if gear.location & EquipmentLocation::AccessoryLeft.as_flag() > 0 {
                snapshot.accessory_left = gear_snapshot;
            }
            if gear.location & EquipmentLocation::AccessoryRight.as_flag() > 0 {
                snapshot.accessory_right = gear_snapshot;
            }
        }
        snapshot
    }

    pub fn weapon_upgrade_damage(&self) -> u16 {
        0
    }

    pub fn weapon_atk(&self) -> u16 {
        self.right_hand_weapon().map(|weapon| *weapon.attack() as u16).unwrap_or(0)
    }

    pub fn weapon_lvl(&self) -> Option<u16> {
        self.right_hand_weapon().map(|right_hand_weapon| *right_hand_weapon.level() as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct KnownSkill {
    pub value: enums::skill_enums::SkillEnum,
    pub level: u8,
}

impl Status {
    pub fn right_hand_weapon(&self) -> Option<&WearWeapon> {
        self.weapons
            .iter()
            .find(|w| w.location & EquipmentLocation::HandRight.as_flag() > 0)
    }

    pub fn equipped_gears(&self) -> &Vec<WearGear> {
        &self.equipments
    }

    pub fn equipped_weapons(&self) -> &Vec<WearWeapon> {
        &self.weapons
    }
    pub fn equipped_ammo(&self) -> &Option<WearAmmo> {
        &self.ammo
    }

    pub fn takeoff_weapon(&mut self, inventory_index: usize) {
        self.weapons
            .retain(|w| w.inventory_index != inventory_index);
    }

    pub fn wear_weapon(&mut self, wear_weapon: WearWeapon) {
        self.weapons.push(wear_weapon);
    }

    pub fn takeoff_all_equipment(&mut self) {
        self.weapons = vec![];
        self.equipments = vec![];
        self.takeoff_ammo();
    }
    pub fn takeoff_equipment(&mut self, inventory_index: usize) {
        self.equipments
            .retain(|w| w.inventory_index != inventory_index);
    }

    pub fn wear_equipment(&mut self, wear_weapon: WearGear) {
        self.equipments.push(wear_weapon);
    }

    pub fn takeoff_ammo(&mut self) {
        self.ammo = None;
    }
    pub fn wear_ammo(&mut self, wear_ammo: WearAmmo) {
        self.ammo = Some(wear_ammo);
    }

    pub fn all_equipped_items(&self) -> Vec<&dyn Wearable> {
        let mut equipments = self
            .equipped_gears()
            .iter()
            .map(|e| e as &dyn Wearable)
            .collect::<Vec<&dyn Wearable>>();
        let weapons = self
            .equipped_weapons()
            .iter()
            .map(|e| e as &dyn Wearable)
            .collect::<Vec<&dyn Wearable>>();
        equipments.extend(weapons);
        if let Some(ammo) = self.equipped_ammo()
            .as_ref() { equipments.push(ammo as &dyn Wearable) }
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
