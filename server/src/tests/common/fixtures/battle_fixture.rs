use std::{env, fs};
use std::fmt::Formatter;
use std::path::Path;
use std::ptr::eq;
use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, SeqAccess, Visitor};
use models::enums::element::Element;
use configuration::serde_helper::deserialize_number_enum;
use crate::models::enums::EnumWithNumberValue;
use models::enums::bonus::BonusType;
use crate::tests::common::character_helper::equip_item_from_id;

#[derive(Deserialize, GettersAll, Debug, Clone, Default)]
pub struct BattleFixture {
    #[serde(rename = "_id")]
    id: String,
    job: String,
    desc: Option<String>,
    #[serde(rename = "baseLevel")]
    base_level: u32,
    #[serde(rename = "jobLevel")]
    job_level: u32,
    #[serde(rename = "baseStr")]
    base_str: u16,
    #[serde(rename = "baseAgi")]
    base_agi: u16,
    #[serde(rename = "baseVit")]
    base_vit: u16,
    #[serde(rename = "baseDex")]
    base_dex: u16,
    #[serde(rename = "baseInt")]
    base_int: u16,
    #[serde(rename = "baseLuk")]
    base_luk: u16,
    #[serde(rename = "equipments", deserialize_with = "deserialize_equipments")]
    equipments: Equipments,
    #[serde(default, rename = "speedPotion")]
    speed_potion: Option<u32>,
    #[serde(rename = "skillToUse")]
    skill_to_use: SkillLevel,
    #[serde(default, rename = "supportiveSkills")]
    supportive_skills: Vec<SkillLevel>,
    #[serde(rename = "arrow")]
    ammo: Option<String>,
    #[serde(rename = "targetName")]
    target: String,
    // Expectation
    #[serde(rename = "maxHp")]
    max_hp: u16,
    #[serde(rename = "maxSp")]
    max_sp: u16,
    #[serde(rename = "bonusStr")]
    bonus_str: i16,
    #[serde(rename = "bonusAgi")]
    bonus_agi: i16,
    #[serde(rename = "bonusVit")]
    bonus_vit: i16,
    #[serde(rename = "bonusDex")]
    bonus_dex: i16,
    #[serde(rename = "bonusInt")]
    bonus_int: i16,
    #[serde(rename = "bonusLuk")]
    bonus_luk: i16,
    hit: i16,
    flee: i16,
    #[serde(rename = "battleHit")]
    battle_hit: u16,
    #[serde(rename = "battleFlee")]
    battle_flee: u16,
    crit: f32,
    #[serde(rename = "critATK")]
    crit_atk: Vec<u16>,
    #[serde(rename = "battleCritAtk")]
    battle_crit_atk: Vec<u16>,
    def: i16,
    mdef: i16,
    cast: f32,
    #[serde(rename = "perfectDodge")]
    perfect_dodge: f32,
    aspd: f32,
    #[serde(rename = "aspdForDisplay")]
    aspd_displayed: f32,
    #[serde(rename = "atkLeft")]
    atk_left: u16,
    #[serde(rename = "atkRight")]
    atk_right: u16,
    #[serde(rename = "baseATK")]
    base_atk: u16,
    #[serde(rename = "minWeaponAttackCalc")]
    min_weapon_attack_calc: f32,
    #[serde(rename = "avgWeaponAttackCalc")]
    avg_weapon_attack_calc: f32,
    #[serde(rename = "maxWeaponAttackCalc")]
    max_weapon_attack_calc: f32,
    #[serde(rename = "minDmg")]
    min_dmg: u32,
    #[serde(rename = "avgDmg")]
    avg_dmg: f32,
    #[serde(rename = "maxDmg")]
    max_dmg: u32,
    #[serde(rename = "minDamageReceived")]
    min_dmg_received: Option<f32>,
    #[serde(rename = "avgDamageReceived")]
    avg_dmg_received: Option<f32>,
    #[serde(rename = "maxDamageReceived")]
    max_dmg_received: Option<f32>,
    #[serde(rename = "matkMin")]
    matk_min: u16,
    #[serde(rename = "matkMax")]
    matk_max: u16,
    #[serde(deserialize_with = "deserialize_number_enum")]
    element: Element
}
#[derive(GettersAll, Debug, Clone, Default)]
pub struct Equipments {
    weapon: Option<Equipment>,
    weapon_left: Option<Equipment>,
    body: Option<Equipment>,
    shield: Option<Equipment>,
    shoes: Option<Equipment>,
    shoulder: Option<Equipment>,
    accessory1: Option<Equipment>,
    accessory2: Option<Equipment>,
    upper_headgear: Option<Equipment>,
    middle_headgear: Option<Equipment>,
    lower_headgear: Option<Equipment>,
}

impl BattleFixture {
    pub fn load(path: &str) -> Vec<Self> {
        let path = Path::new(path);
        if !path.exists() {
            panic!("fixture file does not exists at {}", env::current_dir().unwrap().join(path).to_str().unwrap());
        }
        let fixtures: Vec<BattleFixture> = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        fixtures
    }

    pub fn all_equipments(&self) -> Vec<&Equipment> {
        let mut equipments = vec![];
        self.equipments.weapon().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.weapon_left().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.accessory1().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.accessory2().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.upper_headgear().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.middle_headgear().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.lower_headgear().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.body().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.shoulder().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.shoes().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        self.equipments.shield().as_ref().filter(|e| e.item_id() > 0).map(|e| equipments.push(e));
        equipments
    }
}

#[derive(Deserialize, GettersAll, Debug, Clone, Default)]
pub struct SkillLevel {
    level: u8,
    skid: u32,
}

#[derive(Deserialize, GettersAll, Debug, Clone, Default)]
pub struct Equipment {
    #[serde(default, rename = "itemId")]
    item_id: i32,
    #[serde(default)]
    name: String,
    #[serde(default)]
    refinement: u8,
    #[serde(default)]
    cards: Vec<Card>,
    #[serde(default)]
    bonuses: Vec<BonusTypeWrapper>
}

#[derive(Deserialize, GettersAll, Debug, Clone, Default)]
pub struct Card {
    #[serde(rename = "itemId")]
    item_id: i16,
    name: String,
    #[serde(default)]
    bonuses: Vec<BonusTypeWrapper>
}

#[derive(Debug, Clone)]
pub struct BonusTypeWrapper(pub(crate) BonusType);

impl Default for BonusTypeWrapper {
    fn default() -> Self {
        BonusTypeWrapper {
            0: BonusType::DisableHpRegen,
        }
    }
}

fn deserialize_equipments<'de, D>(deserializer: D) -> Result<Equipments, D::Error>
    where
        D: Deserializer<'de>,
{
    deserializer.deserialize_map(EquipmentVisitor{})
}

struct EquipmentVisitor;
impl <'de>Visitor<'de> for EquipmentVisitor {
    type Value = Equipments;

    fn expecting(&self, _formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_seq<A>(self, _seq: A) -> Result<Equipments, A::Error> where A: SeqAccess<'de> {
        todo!()
        // while let Some(element) = seq.next_element()? {
        //     println!("{:?}", element);
        // }
    }

    fn visit_map<A>(self, mut map: A) -> Result<Equipments, A::Error> where A: MapAccess<'de> {
        let mut weapon: Option<Equipment> = None;
        let mut weapon_left: Option<Equipment> = None;
        let mut body: Option<Equipment> = None;
        let mut shield: Option<Equipment> = None;
        let mut shoes: Option<Equipment> = None;
        let mut shoulder: Option<Equipment> = None;
        let mut accessory1: Option<Equipment> = None;
        let mut accessory2: Option<Equipment> = None;
        let mut upper_headgear: Option<Equipment> = None;
        let mut middle_headgear: Option<Equipment> = None;
        let mut lower_headgear: Option<Equipment> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "accessory1" => accessory1 = map.next_value::<Option<Equipment>>()?,
                "accessory2" => accessory2 = map.next_value::<Option<Equipment>>()?,
                "weapon" => weapon = map.next_value::<Option<Equipment>>()?,
                "weaponLeft" => weapon_left = map.next_value::<Option<Equipment>>()?,
                "body" => body = map.next_value::<Option<Equipment>>()?,
                "shoulder" => shoulder = map.next_value::<Option<Equipment>>()?,
                "shoes" => shoes = map.next_value::<Option<Equipment>>()?,
                "shield" => shield = map.next_value::<Option<Equipment>>()?,
                "upperHeadgear" => upper_headgear = map.next_value::<Option<Equipment>>()?,
                "middleHeadgear" => middle_headgear = map.next_value::<Option<Equipment>>()?,
                "lowerHeadgear" => lower_headgear = map.next_value::<Option<Equipment>>()?,
                _ => { map.next_value::<Option<Equipment>>()?;}
            }
        }
        Ok(Equipments{
            weapon,
            weapon_left,
            body,
            shield,
            shoes,
            shoulder,
            accessory1,
            accessory2,
            upper_headgear,
            middle_headgear,
            lower_headgear,
        })
    }
}


impl<'de> Deserialize<'de> for BonusTypeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de>, {
        deserializer.deserialize_map(BonusTypeWrapperVisitor)
    }
}

struct BonusTypeWrapperVisitor;
impl <'de>Visitor<'de> for BonusTypeWrapperVisitor {
    type Value = BonusTypeWrapper;

    fn expecting(&self, _formatter: &mut Formatter) -> std::fmt::Result {
        todo!()
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {

        let mut bonus: Option<String> = None;
        let mut value: Option<i32> = None;
        let mut value2: Option<i32> = None;
        while let Some(key) = map.next_key::<String>()? {
            if key.as_str() == "bonus" {
                bonus = Some(map.next_value::<String>()?);
            } else if key.as_str() == "value" {
                value = Some(map.next_value::<i32>()?);
            }else if key.as_str() == "value2" {
                value2 = Some(map.next_value::<i32>()?);
            }
        }
        if let Some(bonus) = bonus {
            // To add entry, copy from BonusType enum, then perform following replacements 
            // ([A-z]*)\(([A-z0-9]+)\),
            // "$1" => BonusType::$1(value.unwrap() as $2),
            // 
            // ([A-z]*)\(([A-z0-9]+), ([A-z0-9]+)\),
            // "$1" => BonusType::$1(value.unwrap() as $2, value2.unwrap() as $3),
            //
            // ([A-z]+),
            // "$1" => BonusType::$1,
            let bonus = match bonus.as_str() {
                "Str" => BonusType::Str(value.unwrap() as i8),
                "Agi" => BonusType::Agi(value.unwrap() as i8),
                "Vit" => BonusType::Vit(value.unwrap() as i8),
                "Int" => BonusType::Int(value.unwrap() as i8),
                "Dex" => BonusType::Dex(value.unwrap() as i8),
                "Luk" => BonusType::Luk(value.unwrap() as i8),
                "AllStats" => BonusType::AllStats(value.unwrap() as i8),
                "Hit" => BonusType::Hit(value.unwrap() as i16),
                "HitPercentage" => BonusType::HitPercentage(value.unwrap() as i8),
                "Flee" => BonusType::Flee(value.unwrap() as i16),
                "Crit" => BonusType::Crit(value.unwrap() as i8),
                "PerfectDodge" => BonusType::PerfectDodge(value.unwrap() as i8),
                "Aspd" => BonusType::Aspd(value.unwrap() as i8),
                "AspdPercentage" => BonusType::AspdPercentage(value.unwrap() as i8),
                "Maxhp" => BonusType::Maxhp(value.unwrap() as i32),
                "Maxsp" => BonusType::Maxsp(value.unwrap() as i32),
                "MaxhpPercentage" => BonusType::MaxhpPercentage(value.unwrap() as i8),
                "MaxspPercentage" => BonusType::MaxspPercentage(value.unwrap() as i8),
                "Atk" => BonusType::Atk(value.unwrap() as i16),
                "Def" => BonusType::Def(value.unwrap() as i16),
                "VitDefPercentage" => BonusType::VitDefPercentage(value.unwrap() as i8),
                "DefPercentage" => BonusType::DefPercentage(value.unwrap() as i8),
                "Mdef" => BonusType::Mdef(value.unwrap() as i16),
                "Matk" => BonusType::Matk(value.unwrap() as i16),
                "MatkBasedOnStaffPercentage" => BonusType::MatkBasedOnStaffPercentage(value.unwrap() as i8),
                "MatkPercentage" => BonusType::MatkPercentage(value.unwrap() as i8),
                "AtkPercentage" => BonusType::AtkPercentage(value.unwrap() as i8),
                "PerfectHitPercentage" => BonusType::PerfectHitPercentage(value.unwrap() as i8),
                "ElementWeapon" => BonusType::ElementWeapon(Element::from_value(value.unwrap() as usize)),
                "ElementDefense" => BonusType::ElementDefense(Element::from_value(value.unwrap() as usize)),
                "CriticalDamagePercentage" => BonusType::CriticalDamagePercentage(value.unwrap() as i8),
                "CastTimePercentage" => BonusType::CastTimePercentage(value.unwrap() as i8),
                "CastTimeWhenUsingSkillIdPercentage" => BonusType::CastTimeWhenUsingSkillIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "AfterCastDelayPercentage" => BonusType::AfterCastDelayPercentage(value.unwrap() as i8),
                "NaturalHpRecoveryPercentage" => BonusType::NaturalHpRecoveryPercentage(value.unwrap() as i8),
                "NaturalSpRecoveryPercentage" => BonusType::NaturalSpRecoveryPercentage(value.unwrap() as i8),
                "HpRegenFromItemPercentage" => BonusType::HpRegenFromItemPercentage(value.unwrap() as i8),
                "HpRegenFromItemIDPercentage" => BonusType::HpRegenFromItemIDPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "HpRegenFromHerbPercentage" => BonusType::HpRegenFromHerbPercentage(value.unwrap() as i8),
                "HpRegenFromFruitPercentage" => BonusType::HpRegenFromFruitPercentage(value.unwrap() as i8),
                "HpRegenFromMeatPercentage" => BonusType::HpRegenFromMeatPercentage(value.unwrap() as i8),
                "HpRegenFromCandyPercentage" => BonusType::HpRegenFromCandyPercentage(value.unwrap() as i8),
                "HpRegenFromJuicePercentage" => BonusType::HpRegenFromJuicePercentage(value.unwrap() as i8),
                "HpRegenFromFishPercentage" => BonusType::HpRegenFromFishPercentage(value.unwrap() as i8),
                "HpRegenFromFoodPercentage" => BonusType::HpRegenFromFoodPercentage(value.unwrap() as i8),
                "HpRegenFromPotionPercentage" => BonusType::HpRegenFromPotionPercentage(value.unwrap() as i8),
                "GainHpWhenKillingEnemy" => BonusType::GainHpWhenKillingEnemy(value.unwrap() as i8),
                "GainHpWhenKillingEnemyWithMagicAttack" => BonusType::GainHpWhenKillingEnemyWithMagicAttack(value.unwrap() as i8),
                "GainSpWhenKillingEnemyWithMagicAttack" => BonusType::GainSpWhenKillingEnemyWithMagicAttack(value.unwrap() as i8),
                "HpRegenFromSkillPercentage" => BonusType::HpRegenFromSkillPercentage(value.unwrap() as i8),
                "DisableHpRegen" => BonusType::DisableHpRegen,
                "DisableSpRegen" => BonusType::DisableSpRegen,
                "GainSpWhenHittingEnemy" => BonusType::GainSpWhenHittingEnemy(value.unwrap() as i8),
                "GainSpWhenKillingEnemy" => BonusType::GainSpWhenKillingEnemy(value.unwrap() as i8),
                "SpConsumption" => BonusType::SpConsumption(value.unwrap() as i8),
                "NormalAttackPercentage" => BonusType::NormalAttackPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstSizeSmallPercentage" => BonusType::PhysicalDamageAgainstSizeSmallPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstSizeMediumPercentage" => BonusType::PhysicalDamageAgainstSizeMediumPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstSizeLargePercentage" => BonusType::PhysicalDamageAgainstSizeLargePercentage(value.unwrap() as i8),
                "MagicalDamageAgainstSizeSmallPercentage" => BonusType::MagicalDamageAgainstSizeSmallPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstSizeMediumPercentage" => BonusType::MagicalDamageAgainstSizeMediumPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstSizeLargePercentage" => BonusType::MagicalDamageAgainstSizeLargePercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceFormlessPercentage" => BonusType::PhysicalDamageAgainstRaceFormlessPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceUndeadPercentage" => BonusType::PhysicalDamageAgainstRaceUndeadPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceBrutePercentage" => BonusType::PhysicalDamageAgainstRaceBrutePercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRacePlantPercentage" => BonusType::PhysicalDamageAgainstRacePlantPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceInsectPercentage" => BonusType::PhysicalDamageAgainstRaceInsectPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceFishPercentage" => BonusType::PhysicalDamageAgainstRaceFishPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceDemonPercentage" => BonusType::PhysicalDamageAgainstRaceDemonPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceDemiHumanPercentage" => BonusType::PhysicalDamageAgainstRaceDemiHumanPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceAngelPercentage" => BonusType::PhysicalDamageAgainstRaceAngelPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstRaceDragonPercentage" => BonusType::PhysicalDamageAgainstRaceDragonPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceFormlessPercentage" => BonusType::MagicalDamageAgainstRaceFormlessPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceUndeadPercentage" => BonusType::MagicalDamageAgainstRaceUndeadPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceBrutePercentage" => BonusType::MagicalDamageAgainstRaceBrutePercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRacePlantPercentage" => BonusType::MagicalDamageAgainstRacePlantPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceInsectPercentage" => BonusType::MagicalDamageAgainstRaceInsectPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceFishPercentage" => BonusType::MagicalDamageAgainstRaceFishPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceDemonPercentage" => BonusType::MagicalDamageAgainstRaceDemonPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceDemiHumanPercentage" => BonusType::MagicalDamageAgainstRaceDemiHumanPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceAngelPercentage" => BonusType::MagicalDamageAgainstRaceAngelPercentage(value.unwrap() as i8),
                "MagicalDamageAgainstRaceDragonPercentage" => BonusType::MagicalDamageAgainstRaceDragonPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementNeutralPercentage" => BonusType::PhysicalDamageAgainstElementNeutralPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementWaterPercentage" => BonusType::PhysicalDamageAgainstElementWaterPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementEarthPercentage" => BonusType::PhysicalDamageAgainstElementEarthPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementFirePercentage" => BonusType::PhysicalDamageAgainstElementFirePercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementWindPercentage" => BonusType::PhysicalDamageAgainstElementWindPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementPoisonPercentage" => BonusType::PhysicalDamageAgainstElementPoisonPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementHolyPercentage" => BonusType::PhysicalDamageAgainstElementHolyPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementDarkPercentage" => BonusType::PhysicalDamageAgainstElementDarkPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementGhostPercentage" => BonusType::PhysicalDamageAgainstElementGhostPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstElementUndeadPercentage" => BonusType::PhysicalDamageAgainstElementUndeadPercentage(value.unwrap() as i8),
                "DamageAgainstMobGroupGoblinPercentage" => BonusType::DamageAgainstMobGroupGoblinPercentage(value.unwrap() as i8),
                "DamageAgainstMobGroupKoboldPercentage" => BonusType::DamageAgainstMobGroupKoboldPercentage(value.unwrap() as i8),
                "DamageAgainstMobGroupOrcPercentage" => BonusType::DamageAgainstMobGroupOrcPercentage(value.unwrap() as i8),
                "DamageAgainstMobGroupGolemPercentage" => BonusType::DamageAgainstMobGroupGolemPercentage(value.unwrap() as i8),
                "DamageAgainstMobGroupGuardianPercentage" => BonusType::DamageAgainstMobGroupGuardianPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceFormlessPercentage" => BonusType::CriticalAgainstRaceFormlessPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceUndeadPercentage" => BonusType::CriticalAgainstRaceUndeadPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceBrutePercentage" => BonusType::CriticalAgainstRaceBrutePercentage(value.unwrap() as i8),
                "CriticalAgainstRacePlantPercentage" => BonusType::CriticalAgainstRacePlantPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceInsectPercentage" => BonusType::CriticalAgainstRaceInsectPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceFishPercentage" => BonusType::CriticalAgainstRaceFishPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceDemonPercentage" => BonusType::CriticalAgainstRaceDemonPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceDemiHumanPercentage" => BonusType::CriticalAgainstRaceDemiHumanPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceAngelPercentage" => BonusType::CriticalAgainstRaceAngelPercentage(value.unwrap() as i8),
                "CriticalAgainstRaceDragonPercentage" => BonusType::CriticalAgainstRaceDragonPercentage(value.unwrap() as i8),
                "ChanceToInflictStatusPoisonOnAttackPercentage" => BonusType::ChanceToInflictStatusPoisonOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusStunOnAttackPercentage" => BonusType::ChanceToInflictStatusStunOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusFreezeOnAttackPercentage" => BonusType::ChanceToInflictStatusFreezeOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusCurseOnAttackPercentage" => BonusType::ChanceToInflictStatusCurseOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBlindOnAttackPercentage" => BonusType::ChanceToInflictStatusBlindOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusSleepOnAttackPercentage" => BonusType::ChanceToInflictStatusSleepOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusSilenceOnAttackPercentage" => BonusType::ChanceToInflictStatusSilenceOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBurningOnAttackPercentage" => BonusType::ChanceToInflictStatusBurningOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusChaosOnAttackPercentage" => BonusType::ChanceToInflictStatusChaosOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBleedingOnAttackPercentage" => BonusType::ChanceToInflictStatusBleedingOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusStoneOnAttackPercentage" => BonusType::ChanceToInflictStatusStoneOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusWeaponBreakOnAttackPercentage" => BonusType::ChanceToInflictStatusWeaponBreakOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusArmorBreakOnAttackPercentage" => BonusType::ChanceToInflictStatusArmorBreakOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackPercentage" => BonusType::ChanceToInflictStatusComaOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackOnBossClassPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnBossClassPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackOnNormalClassPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnNormalClassPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceFormlessPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceFormlessPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceUndeadPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceUndeadPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceBrutePercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceBrutePercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRacePlantPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRacePlantPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceInsectPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceInsectPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceFishPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceFishPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceDemonPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceDemonPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceDemiHumanPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceDemiHumanPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceAngelPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceAngelPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceDragonPercentage" => BonusType::ChanceToInflictStatusComaOnAttackRaceDragonPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusPoisonToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusPoisonToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusStunToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusStunToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusFreezeToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusFreezeToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusCurseToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusCurseToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBlindToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusBlindToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusSleepToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusSleepToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusSilenceToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusSilenceToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBurningToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusBurningToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusChaosToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusChaosToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBleedingToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusBleedingToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusStoneToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusStoneToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusWeaponBreakToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusWeaponBreakToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusArmorBreakToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusArmorBreakToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusComaToSelfOnAttackPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusPoisonWhenHitPercentage" => BonusType::ChanceToInflictStatusPoisonWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusStunWhenHitPercentage" => BonusType::ChanceToInflictStatusStunWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusFreezeWhenHitPercentage" => BonusType::ChanceToInflictStatusFreezeWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusCurseWhenHitPercentage" => BonusType::ChanceToInflictStatusCurseWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBlindWhenHitPercentage" => BonusType::ChanceToInflictStatusBlindWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusSleepWhenHitPercentage" => BonusType::ChanceToInflictStatusSleepWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusSilenceWhenHitPercentage" => BonusType::ChanceToInflictStatusSilenceWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBurningWhenHitPercentage" => BonusType::ChanceToInflictStatusBurningWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusChaosWhenHitPercentage" => BonusType::ChanceToInflictStatusChaosWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusBleedingWhenHitPercentage" => BonusType::ChanceToInflictStatusBleedingWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusStoneWhenHitPercentage" => BonusType::ChanceToInflictStatusStoneWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusWeaponBreakWhenHitPercentage" => BonusType::ChanceToInflictStatusWeaponBreakWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusArmorBreakWhenHitPercentage" => BonusType::ChanceToInflictStatusArmorBreakWhenHitPercentage(value.unwrap()as f32),
                "ChanceToInflictStatusComaWhenHitPercentage" => BonusType::ChanceToInflictStatusComaWhenHitPercentage(value.unwrap()as f32),
                "ResistanceToStatusPoisonPercentage" => BonusType::ResistanceToStatusPoisonPercentage(value.unwrap() as i8),
                "ResistanceToStatusStunPercentage" => BonusType::ResistanceToStatusStunPercentage(value.unwrap() as i8),
                "ResistanceToStatusFreezePercentage" => BonusType::ResistanceToStatusFreezePercentage(value.unwrap() as i8),
                "ResistanceToStatusCursePercentage" => BonusType::ResistanceToStatusCursePercentage(value.unwrap() as i8),
                "ResistanceToStatusBurningPercentage" => BonusType::ResistanceToStatusBurningPercentage(value.unwrap() as i8),
                "ResistanceToStatusBlindPercentage" => BonusType::ResistanceToStatusBlindPercentage(value.unwrap() as i8),
                "ResistanceToStatusSleepPercentage" => BonusType::ResistanceToStatusSleepPercentage(value.unwrap() as i8),
                "ResistanceToStatusSilencePercentage" => BonusType::ResistanceToStatusSilencePercentage(value.unwrap() as i8),
                "ResistanceToStatusChaosPercentage" => BonusType::ResistanceToStatusChaosPercentage(value.unwrap() as i8),
                "ResistanceToStatusBleedingPercentage" => BonusType::ResistanceToStatusBleedingPercentage(value.unwrap() as i8),
                "ResistanceToStatusStonePercentage" => BonusType::ResistanceToStatusStonePercentage(value.unwrap() as i8),
                "ResistanceToStatusWeaponBreakPercentage" => BonusType::ResistanceToStatusWeaponBreakPercentage(value.unwrap() as i8),
                "ResistanceToStatusArmorBreakPercentage" => BonusType::ResistanceToStatusArmorBreakPercentage(value.unwrap() as i8),
                "BreakArmorPercentage" => BonusType::BreakArmorPercentage(value.unwrap() as i8),
                "BreakWeaponPercentage" => BonusType::BreakWeaponPercentage(value.unwrap() as i8),
                "ClassChangePercentageOnHit" => BonusType::ClassChangePercentageOnHit(value.unwrap() as i8),
                "LongRangeCriticalChance" => BonusType::LongRangeCriticalChance(value.unwrap() as i8),
                // Only when attacking with ranged weapon
                "IncreaseDamageAgainstClassBossBaseOnDef" => BonusType::IncreaseDamageAgainstClassBossBaseOnDef,
                "IncreaseDamageAgainstClassNormalBaseOnDef" => BonusType::IncreaseDamageAgainstClassNormalBaseOnDef,
                "IncreaseDamageAgainstClassGuardianBaseOnDef" => BonusType::IncreaseDamageAgainstClassGuardianBaseOnDef,
                "PhysicalDamageAgainstClassBossPercentage" => BonusType::PhysicalDamageAgainstClassBossPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstClassNormalPercentage" => BonusType::PhysicalDamageAgainstClassNormalPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstClassGuardianPercentage" => BonusType::PhysicalDamageAgainstClassGuardianPercentage(value.unwrap() as i8),
                "PhysicalDamageAgainstMobIdPercentage" => BonusType::PhysicalDamageAgainstMobIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "ResistanceDamageFromClassBossPercentage" => BonusType::ResistanceDamageFromClassBossPercentage(value.unwrap() as i8),
                "ResistanceDamageFromClassNormalPercentage" => BonusType::ResistanceDamageFromClassNormalPercentage(value.unwrap() as i8),
                "ResistanceDamageFromClassGuardianPercentage" => BonusType::ResistanceDamageFromClassGuardianPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementNeutralPercentage" => BonusType::ResistanceDamageFromElementNeutralPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementWaterPercentage" => BonusType::ResistanceDamageFromElementWaterPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementEarthPercentage" => BonusType::ResistanceDamageFromElementEarthPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementFirePercentage" => BonusType::ResistanceDamageFromElementFirePercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementWindPercentage" => BonusType::ResistanceDamageFromElementWindPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementPoisonPercentage" => BonusType::ResistanceDamageFromElementPoisonPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementHolyPercentage" => BonusType::ResistanceDamageFromElementHolyPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementDarkPercentage" => BonusType::ResistanceDamageFromElementDarkPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementGhostPercentage" => BonusType::ResistanceDamageFromElementGhostPercentage(value.unwrap() as i8),
                "ResistanceDamageFromElementUndeadPercentage" => BonusType::ResistanceDamageFromElementUndeadPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceFormlessPercentage" => BonusType::ResistanceDamageFromRaceFormlessPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceUndeadPercentage" => BonusType::ResistanceDamageFromRaceUndeadPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceBrutePercentage" => BonusType::ResistanceDamageFromRaceBrutePercentage(value.unwrap() as i8),
                "ResistanceDamageFromRacePlantPercentage" => BonusType::ResistanceDamageFromRacePlantPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceInsectPercentage" => BonusType::ResistanceDamageFromRaceInsectPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceFishPercentage" => BonusType::ResistanceDamageFromRaceFishPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceDemonPercentage" => BonusType::ResistanceDamageFromRaceDemonPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceDemiHumanPercentage" => BonusType::ResistanceDamageFromRaceDemiHumanPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceAngelPercentage" => BonusType::ResistanceDamageFromRaceAngelPercentage(value.unwrap() as i8),
                "ResistanceDamageFromRaceDragonPercentage" => BonusType::ResistanceDamageFromRaceDragonPercentage(value.unwrap() as i8),
                "ResistanceDamageFromSizeSmallPercentage" => BonusType::ResistanceDamageFromSizeSmallPercentage(value.unwrap() as i8),
                "ResistanceDamageFromSizeMediumPercentage" => BonusType::ResistanceDamageFromSizeMediumPercentage(value.unwrap() as i8),
                "ResistanceDamageFromSizeLargePercentage" => BonusType::ResistanceDamageFromSizeLargePercentage(value.unwrap() as i8),

                "SkillDelayIncDecPercentage" => BonusType::SkillDelayIncDecPercentage(value.unwrap() as i8),
                "DoubleAttackChancePercentage" => BonusType::DoubleAttackChancePercentage(value.unwrap() as i8),
                "HealSkillPercentage" => BonusType::HealSkillPercentage(value.unwrap() as i8),
                "HealSkillIdPercentage" => BonusType::HealSkillIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "IgnoreDefClassNormal" => BonusType::IgnoreDefClassNormal,
                "IgnoreDefClassBoss" => BonusType::IgnoreDefClassBoss,
                "IgnoreDefClassGuardian" => BonusType::IgnoreDefClassGuardian,
                "IgnoreDefRaceAngel" => BonusType::IgnoreDefRaceAngel,
                "IgnoreDefRaceBrute" => BonusType::IgnoreDefRaceBrute,
                "IgnoreDefRaceDemiHuman" => BonusType::IgnoreDefRaceDemiHuman,
                "IgnoreDefRaceDemon" => BonusType::IgnoreDefRaceDemon,
                "IgnoreDefRaceDragon" => BonusType::IgnoreDefRaceDragon,
                "IgnoreDefRaceFish" => BonusType::IgnoreDefRaceFish,
                "IgnoreDefRaceFormless" => BonusType::IgnoreDefRaceFormless,
                "IgnoreDefRaceInsect" => BonusType::IgnoreDefRaceInsect,
                "IgnoreDefRacePlant" => BonusType::IgnoreDefRacePlant,
                "IgnoreDefRacePlayerHuman" => BonusType::IgnoreDefRacePlayerHuman,
                "IgnoreDefRacePlayerDoram" => BonusType::IgnoreDefRacePlayerDoram,
                "IgnoreDefRaceUndead" => BonusType::IgnoreDefRaceUndead,
                "IgnoreDefRaceFormlessPercentage" => BonusType::IgnoreDefRaceFormlessPercentage(value.unwrap() as i8),
                "IgnoreDefRaceUndeadPercentage" => BonusType::IgnoreDefRaceUndeadPercentage(value.unwrap() as i8),
                "IgnoreDefRaceBrutePercentage" => BonusType::IgnoreDefRaceBrutePercentage(value.unwrap() as i8),
                "IgnoreDefRacePlantPercentage" => BonusType::IgnoreDefRacePlantPercentage(value.unwrap() as i8),
                "IgnoreDefRaceInsectPercentage" => BonusType::IgnoreDefRaceInsectPercentage(value.unwrap() as i8),
                "IgnoreDefRaceFishPercentage" => BonusType::IgnoreDefRaceFishPercentage(value.unwrap() as i8),
                "IgnoreDefRaceDemonPercentage" => BonusType::IgnoreDefRaceDemonPercentage(value.unwrap() as i8),
                "IgnoreDefRaceDemiHumanPercentage" => BonusType::IgnoreDefRaceDemiHumanPercentage(value.unwrap() as i8),
                "IgnoreDefRaceAngelPercentage" => BonusType::IgnoreDefRaceAngelPercentage(value.unwrap() as i8),
                "IgnoreDefRaceDragonPercentage" => BonusType::IgnoreDefRaceDragonPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceFormlessPercentage" => BonusType::IgnoreMDefRaceFormlessPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceUndeadPercentage" => BonusType::IgnoreMDefRaceUndeadPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceBrutePercentage" => BonusType::IgnoreMDefRaceBrutePercentage(value.unwrap() as i8),
                "IgnoreMDefRacePlantPercentage" => BonusType::IgnoreMDefRacePlantPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceInsectPercentage" => BonusType::IgnoreMDefRaceInsectPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceFishPercentage" => BonusType::IgnoreMDefRaceFishPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceDemonPercentage" => BonusType::IgnoreMDefRaceDemonPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceDemiHumanPercentage" => BonusType::IgnoreMDefRaceDemiHumanPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceAngelPercentage" => BonusType::IgnoreMDefRaceAngelPercentage(value.unwrap() as i8),
                "IgnoreMDefRaceDragonPercentage" => BonusType::IgnoreMDefRaceDragonPercentage(value.unwrap() as i8),
                "IgnoreMDefClassNormalPercentage" => BonusType::IgnoreMDefClassNormalPercentage(value.unwrap() as i8),
                "IgnoreMDefClassBossPercentage" => BonusType::IgnoreMDefClassBossPercentage(value.unwrap() as i8),
                "IgnoreMDefClassGuardianPercentage" => BonusType::IgnoreMDefClassGuardianPercentage(value.unwrap() as i8),
                "ResistanceRangeAttackPercentage" => BonusType::ResistanceRangeAttackPercentage(value.unwrap() as i8),
                "DamageRangedAtkPercentage" => BonusType::DamageRangedAtkPercentage(value.unwrap() as i8),
                "ResistanceMagicAttackPercentage" => BonusType::ResistanceMagicAttackPercentage(value.unwrap() as i8),
                "MagicAttackReflectChancePercentage" => BonusType::MagicAttackReflectChancePercentage(value.unwrap() as i8),
                "MeleeAttackReflectChancePercentage" => BonusType::MeleeAttackReflectChancePercentage(value.unwrap() as i8),
                "SplashRadius" => BonusType::SplashRadius(value.unwrap() as i8),
                "SpeedPercentage" => BonusType::SpeedPercentage(value.unwrap() as i8),
                "EnableFullHpSpRecoverOnResurrect" => BonusType::EnableFullHpSpRecoverOnResurrect,
                "EnableSeeHidden" => BonusType::EnableSeeHidden,
                "EnableNoCancelCast" => BonusType::EnableNoCancelCast,
                "EnableNoGemstoneRequired" => BonusType::EnableNoGemstoneRequired,
                "EnableIgnoreSizeModifier" => BonusType::EnableIgnoreSizeModifier,
                "EnableNoKnockback" => BonusType::EnableNoKnockback,
                "EnableNoWalkDelay" => BonusType::EnableNoWalkDelay,
                "UnbreakableArmor" => BonusType::UnbreakableArmor,
                "UnbreakableShoulder" => BonusType::UnbreakableShoulder,
                "UnbreakableHelm" => BonusType::UnbreakableHelm,
                "UnbreakableShield" => BonusType::UnbreakableShield,
                "UnbreakableShoes" => BonusType::UnbreakableShoes,
                "UnbreakableWeapon" => BonusType::UnbreakableWeapon,
                "ResistancePhysicalAttackFromMobIdPercentage" => BonusType::ResistancePhysicalAttackFromMobIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "DropChanceItemIdPercentage" => BonusType::DropChanceItemIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "DropChanceJewelPercentage" => BonusType::DropChanceJewelPercentage(value.unwrap() as i8),
                "DropChanceOrePercentage" => BonusType::DropChanceOrePercentage(value.unwrap() as i8),
                "DropChanceRecoveryPercentage" => BonusType::DropChanceRecoveryPercentage(value.unwrap() as i8),
                "DropChanceFoodPercentage" => BonusType::DropChanceFoodPercentage(value.unwrap() as i8),
                "KnockbackWhenUsingSkillId" => BonusType::KnockbackWhenUsingSkillId(value.unwrap() as u32, value2.unwrap() as i8),
                "GainExpWhenKillingRaceFormlessPercentage" => BonusType::GainExpWhenKillingRaceFormlessPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceUndeadPercentage" => BonusType::GainExpWhenKillingRaceUndeadPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceBrutePercentage" => BonusType::GainExpWhenKillingRaceBrutePercentage(value.unwrap() as i8),
                "GainExpWhenKillingRacePlantPercentage" => BonusType::GainExpWhenKillingRacePlantPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceInsectPercentage" => BonusType::GainExpWhenKillingRaceInsectPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceFishPercentage" => BonusType::GainExpWhenKillingRaceFishPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceDemonPercentage" => BonusType::GainExpWhenKillingRaceDemonPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceDemiHumanPercentage" => BonusType::GainExpWhenKillingRaceDemiHumanPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceAngelPercentage" => BonusType::GainExpWhenKillingRaceAngelPercentage(value.unwrap() as i8),
                "GainExpWhenKillingRaceDragonPercentage" => BonusType::GainExpWhenKillingRaceDragonPercentage(value.unwrap() as i8),
                "GainZenyWhenKillingMonster" => BonusType::GainZenyWhenKillingMonster(value.unwrap() as u16, value2.unwrap() as i8),
                // "zeny" => BonusType::zeny, chance
                "HpDrainWhenAttackingPercentage" => BonusType::HpDrainWhenAttackingPercentage(value.unwrap() as i8, value2.unwrap() as i8),
                // hp "percentage" => BonusType::percentage, chance
                "SpDrainWhenAttackingPercentage" => BonusType::SpDrainWhenAttackingPercentage(value.unwrap() as i8, value2.unwrap() as i8),
                // sp "percentage" => BonusType::percentage, chance
                "SpDrainWhenAttackingRaceFormless" => BonusType::SpDrainWhenAttackingRaceFormless(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceUndead" => BonusType::SpDrainWhenAttackingRaceUndead(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceBrute" => BonusType::SpDrainWhenAttackingRaceBrute(value.unwrap() as u16),
                "SpDrainWhenAttackingRacePlant" => BonusType::SpDrainWhenAttackingRacePlant(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceInsect" => BonusType::SpDrainWhenAttackingRaceInsect(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceFish" => BonusType::SpDrainWhenAttackingRaceFish(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceDemon" => BonusType::SpDrainWhenAttackingRaceDemon(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceDemiHuman" => BonusType::SpDrainWhenAttackingRaceDemiHuman(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceAngel" => BonusType::SpDrainWhenAttackingRaceAngel(value.unwrap() as u16),
                "SpDrainWhenAttackingRaceDragon" => BonusType::SpDrainWhenAttackingRaceDragon(value.unwrap() as u16),
                "SpDrainWhenKillingRaceFormless" => BonusType::SpDrainWhenKillingRaceFormless(value.unwrap() as u16),
                "SpDrainWhenKillingRaceUndead" => BonusType::SpDrainWhenKillingRaceUndead(value.unwrap() as u16),
                "SpDrainWhenKillingRaceBrute" => BonusType::SpDrainWhenKillingRaceBrute(value.unwrap() as u16),
                "SpDrainWhenKillingRacePlant" => BonusType::SpDrainWhenKillingRacePlant(value.unwrap() as u16),
                "SpDrainWhenKillingRaceInsect" => BonusType::SpDrainWhenKillingRaceInsect(value.unwrap() as u16),
                "SpDrainWhenKillingRaceFish" => BonusType::SpDrainWhenKillingRaceFish(value.unwrap() as u16),
                "SpDrainWhenKillingRaceDemon" => BonusType::SpDrainWhenKillingRaceDemon(value.unwrap() as u16),
                "SpDrainWhenKillingRaceDemiHuman" => BonusType::SpDrainWhenKillingRaceDemiHuman(value.unwrap() as u16),
                "SpDrainWhenKillingRaceAngel" => BonusType::SpDrainWhenKillingRaceAngel(value.unwrap() as u16),
                "SpDrainWhenKillingRaceDragon" => BonusType::SpDrainWhenKillingRaceDragon(value.unwrap() as u16),
                "SpBurnOnTargetWhenAttackingPercentage" => BonusType::SpBurnOnTargetWhenAttackingPercentage(value.unwrap() as i8, value2.unwrap() as u16),
                // "percentage" => BonusType::percentage, chance
                "HpLossEveryMs" => BonusType::HpLossEveryMs(value.unwrap() as u16, value2.unwrap() as u16),
                //"amount" => BonusType::amount, every ms
                "HpRegenEveryMs" => BonusType::HpRegenEveryMs(value.unwrap() as u16, value2.unwrap() as u16),
                //"amount" => BonusType::amount, every ms
                "SpLossEveryMs" => BonusType::SpLossEveryMs(value.unwrap() as u16, value2.unwrap() as u16),
                //"amount" => BonusType::amount, every ms
                "SpRegenEveryMs" => BonusType::SpRegenEveryMs(value.unwrap() as u16, value2.unwrap() as u16),
                //"amount" => BonusType::amount, every ms
                "SkillIdDamagePercentage" => BonusType::SkillIdDamagePercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "EnableSkill" => BonusType::EnableSkillId(value.unwrap() as u32, value2.unwrap() as u8),
                _ => {
                    Err(serde::de::Error::custom(format!("Bonus not found with name {}", bonus)))?
                }
            };
            return Ok(BonusTypeWrapper(bonus));
        }
        Err(serde::de::Error::missing_field("bonus"))
    }
}
