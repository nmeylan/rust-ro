use crate::item::{WearAmmo, WearGear, WearGearSnapshot, WearWeapon, Wearable, WearAmmoSnapshot, WearWeaponSnapshot};
use accessor::{GettersAll, SettersAll};
use crate::enums::bonus::BonusType;
use crate::enums::element::Element;
use crate::enums::item::EquipmentLocation;
use crate::enums::size::Size;
use crate::enums::EnumWithMaskValueU64;
use crate::enums::mob::MobRace;
use crate::enums::status::StatusEffect;
use crate::enums::weapon::WeaponType;

#[derive(SettersAll, GettersAll, Debug, Default, Clone)]
pub struct Status {
    pub job: u32,
    pub hp: u32,
    pub sp: u32,
    pub str: u16,
    pub agi: u16,
    pub vit: u16,
    pub int: u16,
    pub dex: u16,
    pub luk: u16,
    pub speed: u16,
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
    pub is_male: bool,
    pub weapons: Vec<WearWeapon>,
    pub equipments: Vec<WearGear>,
    pub ammo: Option<WearAmmo>,
    pub known_skills: Vec<KnownSkill>,
    pub effect: Option<StatusEffect>,
    pub bonuses: Vec<StatusBonus>,
    pub bonuses_temporary: Vec<TemporaryStatusBonus>,
}

#[derive(Clone, Debug, SettersAll, GettersAll)]
pub struct StatusSnapshot {
    job: u32,
    hp: u32,
    max_hp: u32,
    sp: u32,
    max_sp: u32,
    base_str: u16,
    base_agi: u16,
    base_vit: u16,
    base_int: u16,
    base_dex: u16,
    base_luk: u16,
    base_atk: u16,
    bonus_str: i16,
    bonus_agi: i16,
    bonus_vit: i16,
    bonus_int: i16,
    bonus_dex: i16,
    bonus_luk: i16,
    bonus_atk: u16,
    matk_min: u16,
    matk_max: u16,
    atk_left_side: i32,
    atk_right_side: i32,
    overupgrade_right_hand_atk_bonus: u8,
    overupgrade_left_hand_atk_bonus: u8,
    fist_atk: u16,
    matk_item_modifier: f32,
    speed: u16,
    hit: i16,
    flee: i16,
    crit: f32,
    def: i16,
    mdef: i16,
    size: Size,
    element: Element,
    race: MobRace,
    element_level: u8,
    state: u64,
    zeny: u32,
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
    effect: Option<StatusEffect>,
    known_skills: Vec<KnownSkill>,
    bonuses: Vec<StatusBonus>,
}

impl StatusSnapshot {
    pub fn new_for_mob(mob_id: u32, hp: u32, sp: u32, max_hp:u32, max_sp: u32,
                       str: u16, agi: u16, vit: u16, int: u16, dex: u16, luk: u16,
                       atk1: u16, _atk2: u16, matk1: u16, matk2: u16, speed: u16, def: u16, mdef: u16,
                       size: Size,
                       element: Element,
                       race: MobRace,
                       element_level: u8) -> Self {
        Self {
            job: mob_id,
            hp,
            max_hp,
            sp,
            max_sp,
            base_str: str,
            base_agi: agi,
            base_vit: vit,
            base_int: int,
            base_dex: dex,
            base_luk: luk,
            base_atk: atk1,
            bonus_str: 0,
            bonus_agi: 0,
            bonus_vit: 0,
            bonus_int: 0,
            bonus_dex: 0,
            bonus_luk: 0,
            bonus_atk: 0,
            matk_min: matk1,
            matk_max: matk2,
            atk_left_side: 0,
            atk_right_side: 0,
            overupgrade_right_hand_atk_bonus: 0,
            overupgrade_left_hand_atk_bonus: 0,
            fist_atk: 0,
            matk_item_modifier: 1.0,
            speed,
            hit: 0,
            flee: 0,
            crit: 0.0,
            def: def as i16,
            mdef: mdef as i16,
            size,
            element,
            race,
            element_level,
            state: 0,
            zeny: 0,
            aspd: 0.0,
            right_hand_weapon: None,
            right_hand_weapon_type: WeaponType::Fist,
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
            ammo: None,
            effect: None,
            known_skills: vec![],
            bonuses: vec![],
        }
    }
    /// Do not use this method directly, use StatusService::to_snapshot instead
    pub fn _from(status: &Status) -> Self {
        let mut snapshot = Self {
            job: status.job,
            hp: status.hp,
            max_hp: 0,
            sp: status.sp,
            max_sp: 0,
            base_str: status.str,
            base_agi: status.agi,
            base_vit: status.vit,
            base_int: status.int,
            base_dex: status.dex,
            base_luk: status.luk,
            base_atk: 0,
            bonus_str: 0,
            bonus_agi: 0,
            bonus_vit: 0,
            bonus_int: 0,
            bonus_dex: 0,
            bonus_luk: 0,
            bonus_atk: 0,
            matk_min: 0,
            matk_max: 0,
            atk_left_side: 0,
            atk_right_side: 0,
            overupgrade_right_hand_atk_bonus: 0,
            overupgrade_left_hand_atk_bonus: 0,
            fist_atk: 0,
            matk_item_modifier: 1.0,
            speed: status.speed,
            hit: 0,
            flee: 0,
            crit: 0.0,
            def: 0,
            mdef: 0,
            size: status.size,
            element: Element::Neutral,
            race: MobRace::DemiHuman,
            element_level: 1,
            state: status.state,
            zeny: status.zeny,
            aspd: 0.0,
            right_hand_weapon: status.right_hand_weapon().map(|w| w.to_snapshot()),
            right_hand_weapon_type: status.right_hand_weapon().map(|w| *w.weapon_type()).unwrap_or(WeaponType::Fist),
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
            effect: status.effect,
            known_skills: status.known_skills.clone(),
            bonuses: vec![],
        };
        for gear in status.equipped_gears() {
            let gear_snapshot = Some(gear.to_snapshot());
            if gear.location() & EquipmentLocation::HeadTop.as_flag() > 0 {
                snapshot.upper_headgear = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::HeadMid.as_flag() > 0 {
                snapshot.middle_headgear = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::HeadLow.as_flag() > 0 {
                snapshot.lower_headgear = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::Armor.as_flag() > 0 {
                snapshot.body = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::Shoes.as_flag() > 0 {
                snapshot.shoes = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::HandLeft.as_flag() > 0 {
                snapshot.shield = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::Garment.as_flag() > 0 {
                snapshot.shoulder = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::AccessoryLeft.as_flag() > 0 {
                snapshot.accessory_left = gear_snapshot;
            }
            if gear.location() & EquipmentLocation::AccessoryRight.as_flag() > 0 {
                snapshot.accessory_right = gear_snapshot;
            }
        }
        snapshot
    }

    pub fn weapon_upgrade_damage(&self) -> u16 {
        0
    }

    pub fn weapon_atk(&self) -> u16 {
        self.right_hand_weapon().map(|weapon| weapon.attack() as u16).unwrap_or(0)
    }

    pub fn weapon_lvl(&self) -> Option<u16> {
        self.right_hand_weapon().map(|right_hand_weapon| right_hand_weapon.level() as u16)
    }

    pub fn str(&self) -> u16 {
        (self.base_str as i16 + self.bonus_str).max(0) as u16
    }
    pub fn agi(&self) -> u16 {
        (self.base_agi as i16 + self.bonus_agi).max(0) as u16
    }
    pub fn vit(&self) -> u16 {
        (self.base_vit as i16 + self.bonus_vit).max(0) as u16
    }
    pub fn dex(&self) -> u16 {
        (self.base_dex as i16 + self.bonus_dex).max(0) as u16
    }
    pub fn int(&self) -> u16 {
        (self.base_int as i16 + self.bonus_int).max(0) as u16
    }
    pub fn luk(&self) -> u16 {
        (self.base_luk as i16 + self.bonus_luk).max(0) as u16
    }

    pub fn bonuses_mut(&mut self) -> &mut Vec<StatusBonus> {
        &mut self.bonuses
    }

    pub fn bonuses_raw(&self) -> Vec<&BonusType>{
        self.bonuses.iter().map(|b| &b.bonus).collect::<Vec<&BonusType>>()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct KnownSkill {
    pub value: crate::enums::skill_enums::SkillEnum,
    pub level: u8,
}

impl Status {
    pub fn right_hand_weapon(&self) -> Option<&WearWeapon> {
        self.weapons
            .iter()
            .find(|w| w.location() & EquipmentLocation::HandRight.as_flag() > 0)
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
            .retain(|w| w.inventory_index() != inventory_index);
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
            .retain(|w| w.inventory_index() != inventory_index);
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
        let mut equipments = Vec::with_capacity(self.equipped_gears().len() + self.equipped_weapons().len() + 1);
        for equipment in self.equipped_gears().iter() {
            equipments.push(equipment as &dyn Wearable);
        }
        for weapon in self.equipped_weapons().iter() {
            equipments.push(weapon as &dyn Wearable);
        }
        if let Some(ammo) = self.equipped_ammo()
            .as_ref() { equipments.push(ammo as &dyn Wearable) }
        equipments
    }

    pub fn attack_range(&self) -> u8 {
        self.right_hand_weapon().map(|w| w.range()).unwrap_or(1_u8)
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

#[derive(GettersAll, Debug, Clone, Copy)]
pub struct StatusBonus {
    bonus: BonusType,
}
impl StatusBonus {
    pub fn new(bonus: BonusType) -> StatusBonus {
        Self {
            bonus,
        }
    }
}
#[derive(GettersAll, Debug, Clone, Copy)]
pub struct TemporaryStatusBonus {
    bonus: BonusType,
    expire_at: u128,
    expire_after_count: u8,
    expire_counter: u8,
    // TODO can also expire after certain condition: hp absorbed (kyrie eleison)
}