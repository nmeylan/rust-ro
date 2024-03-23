use std::{env, fs};
use std::fmt::Formatter;
use std::path::Path;

use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, SeqAccess, Visitor};
use models::enums::element::Element;
use configuration::serde_helper::deserialize_number_enum;
use crate::models::enums::EnumWithNumberValue;
use models::enums::bonus::BonusType;
use models::enums::mob::MobRace;
use models::enums::status::StatusEffect;
use models::enums::mob::MobClass;
use models::enums::size::Size;
use models::enums::mob::MobGroup;


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
    #[serde(rename = "ammo")]
    ammo: Option<String>,
    #[serde(rename = "ammoId")]
    ammo_id: Option<u32>,
    #[serde(rename = "targetName")]
    target: String,
    #[serde(rename = "targetId")]
    target_id: u32,
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
    #[serde(default, rename = "critATK")]
    crit_atk: Vec<i16>,
    #[serde(default, rename = "battleCritAtk")]
    battle_crit_atk: Vec<i16>,
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
    min_dmg: i32,
    #[serde(rename = "avgDmg")]
    avg_dmg: f32,
    #[serde(rename = "maxDmg")]
    max_dmg: i32,
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
        if let Some(e) = self.equipments.weapon().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.weapon_left().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.accessory1().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.accessory2().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.upper_headgear().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.middle_headgear().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.lower_headgear().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.body().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.shoulder().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.shoes().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
        if let Some(e) = self.equipments.shield().as_ref().filter(|e| e.item_id() > 0) { equipments.push(e) }
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
        BonusTypeWrapper(BonusType::DisableHpRegen)
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
                "Maxhp" => BonusType::Maxhp(value.unwrap()),
                "Maxsp" => BonusType::Maxsp(value.unwrap()),
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
                "PhysicalDamageAgainstSizeSmallPercentage" => BonusType::PhysicalDamageAgainstSizePercentage(Size::Small, value.unwrap() as i8),
                "PhysicalDamageAgainstSizeMediumPercentage" => BonusType::PhysicalDamageAgainstSizePercentage(Size::Medium, value.unwrap() as i8),
                "PhysicalDamageAgainstSizeLargePercentage" => BonusType::PhysicalDamageAgainstSizePercentage(Size::Large, value.unwrap() as i8),
                "MagicalDamageAgainstSizeSmallPercentage" => BonusType::MagicalDamageAgainstSizePercentage(Size::Small, value.unwrap() as i8),
                "MagicalDamageAgainstSizeMediumPercentage" => BonusType::MagicalDamageAgainstSizePercentage(Size::Medium, value.unwrap() as i8),
                "MagicalDamageAgainstSizeLargePercentage" => BonusType::MagicalDamageAgainstSizePercentage(Size::Large, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceFormlessPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceUndeadPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceBrutePercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "PhysicalDamageAgainstRacePlantPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceInsectPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceFishPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceDemonPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceDemiHumanPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceAngelPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "PhysicalDamageAgainstRaceDragonPercentage" => BonusType::PhysicalDamageAgainstRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "MagicalDamageAgainstRaceFormlessPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "MagicalDamageAgainstRaceUndeadPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "MagicalDamageAgainstRaceBrutePercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "MagicalDamageAgainstRacePlantPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "MagicalDamageAgainstRaceInsectPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "MagicalDamageAgainstRaceFishPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "MagicalDamageAgainstRaceDemonPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "MagicalDamageAgainstRaceDemiHumanPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "MagicalDamageAgainstRaceAngelPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "MagicalDamageAgainstRaceDragonPercentage" => BonusType::MagicalDamageAgainstRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "PhysicalDamageAgainstElementNeutralPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Neutral, value.unwrap() as i8),
                "PhysicalDamageAgainstElementWaterPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Water, value.unwrap() as i8),
                "PhysicalDamageAgainstElementEarthPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Earth, value.unwrap() as i8),
                "PhysicalDamageAgainstElementFirePercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Fire, value.unwrap() as i8),
                "PhysicalDamageAgainstElementWindPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Wind, value.unwrap() as i8),
                "PhysicalDamageAgainstElementPoisonPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Poison, value.unwrap() as i8),
                "PhysicalDamageAgainstElementHolyPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Holy, value.unwrap() as i8),
                "PhysicalDamageAgainstElementDarkPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Dark, value.unwrap() as i8),
                "PhysicalDamageAgainstElementGhostPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Ghost, value.unwrap() as i8),
                "PhysicalDamageAgainstElementUndeadPercentage" => BonusType::PhysicalDamageAgainstElementPercentage(Element::Undead, value.unwrap() as i8),
                "DamageAgainstMobGroupGoblinPercentage" => BonusType::DamageAgainstMobGroupPercentage(MobGroup::Goblin, value.unwrap() as i8),
                "DamageAgainstMobGroupKoboldPercentage" => BonusType::DamageAgainstMobGroupPercentage(MobGroup::Kobold, value.unwrap() as i8),
                "DamageAgainstMobGroupOrcPercentage" => BonusType::DamageAgainstMobGroupPercentage(MobGroup::Orc, value.unwrap() as i8),
                "DamageAgainstMobGroupGolemPercentage" => BonusType::DamageAgainstMobGroupPercentage(MobGroup::Golem, value.unwrap() as i8),
                "DamageAgainstMobGroupGuardianPercentage" => BonusType::DamageAgainstMobGroupPercentage(MobGroup::Guardian, value.unwrap() as i8),
                "CriticalAgainstRaceFormlessPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "CriticalAgainstRaceUndeadPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "CriticalAgainstRaceBrutePercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "CriticalAgainstRacePlantPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "CriticalAgainstRaceInsectPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "CriticalAgainstRaceFishPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "CriticalAgainstRaceDemonPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "CriticalAgainstRaceDemiHumanPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "CriticalAgainstRaceAngelPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "CriticalAgainstRaceDragonPercentage" => BonusType::CriticalAgainstRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "ChanceToInflictStatusPoisonOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Poison, value.unwrap()as f32),
                "ChanceToInflictStatusStunOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Stun, value.unwrap()as f32),
                "ChanceToInflictStatusFreezeOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Freeze, value.unwrap()as f32),
                "ChanceToInflictStatusCurseOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Curse, value.unwrap()as f32),
                "ChanceToInflictStatusBlindOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Blind, value.unwrap()as f32),
                "ChanceToInflictStatusSleepOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Sleep, value.unwrap()as f32),
                "ChanceToInflictStatusSilenceOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Silence, value.unwrap()as f32),
                "ChanceToInflictStatusBurningOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Burning, value.unwrap()as f32),
                "ChanceToInflictStatusChaosOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Chaos, value.unwrap()as f32),
                "ChanceToInflictStatusBleedingOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Bleeding, value.unwrap()as f32),
                "ChanceToInflictStatusStoneOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Stone, value.unwrap()as f32),
                "ChanceToInflictStatusWeaponBreakOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::WeaponBreak, value.unwrap()as f32),
                "ChanceToInflictStatusArmorBreakOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::ArmorBreak, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackPercentage" => BonusType::ChanceToInflictStatusOnAttackPercentage(StatusEffect::Coma, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackOnBossClassPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(MobClass::Boss, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackOnNormalClassPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(MobClass::Normal, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackOnGuardianClassPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnClassPercentage(MobClass::Guardian, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceFormlessPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Formless, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceUndeadPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Undead, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceBrutePercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Brute, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRacePlantPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Plant, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceInsectPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Insect, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceFishPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Fish, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceDemonPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Demon, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceDemiHumanPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::DemiHuman, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceAngelPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Angel, value.unwrap()as f32),
                "ChanceToInflictStatusComaOnAttackRaceDragonPercentage" => BonusType::ChanceToInflictStatusComaOnAttackOnRacePercentage(MobRace::Dragon, value.unwrap()as f32),
                "ChanceToInflictStatusPoisonToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Poison, value.unwrap()as f32),
                "ChanceToInflictStatusStunToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Stun, value.unwrap()as f32),
                "ChanceToInflictStatusFreezeToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Freeze, value.unwrap()as f32),
                "ChanceToInflictStatusCurseToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Curse, value.unwrap()as f32),
                "ChanceToInflictStatusBlindToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Blind, value.unwrap()as f32),
                "ChanceToInflictStatusSleepToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Sleep, value.unwrap()as f32),
                "ChanceToInflictStatusSilenceToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Silence, value.unwrap()as f32),
                "ChanceToInflictStatusBurningToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Burning, value.unwrap()as f32),
                "ChanceToInflictStatusChaosToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Chaos, value.unwrap()as f32),
                "ChanceToInflictStatusBleedingToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Bleeding, value.unwrap()as f32),
                "ChanceToInflictStatusStoneToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Stone, value.unwrap()as f32),
                "ChanceToInflictStatusWeaponBreakToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::WeaponBreak, value.unwrap()as f32),
                "ChanceToInflictStatusArmorBreakToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::ArmorBreak, value.unwrap()as f32),
                "ChanceToInflictStatusComaToSelfOnAttackPercentage" => BonusType::ChanceToInflictStatusToSelfOnAttackPercentage(StatusEffect::Coma, value.unwrap()as f32),
                "ChanceToInflictStatusPoisonWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Poison, value.unwrap()as f32),
                "ChanceToInflictStatusStunWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Stun, value.unwrap()as f32),
                "ChanceToInflictStatusFreezeWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Freeze, value.unwrap()as f32),
                "ChanceToInflictStatusCurseWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Curse, value.unwrap()as f32),
                "ChanceToInflictStatusBlindWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Blind, value.unwrap()as f32),
                "ChanceToInflictStatusSleepWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Sleep, value.unwrap()as f32),
                "ChanceToInflictStatusSilenceWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Silence, value.unwrap()as f32),
                "ChanceToInflictStatusBurningWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Burning, value.unwrap()as f32),
                "ChanceToInflictStatusChaosWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Chaos, value.unwrap()as f32),
                "ChanceToInflictStatusBleedingWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Bleeding, value.unwrap()as f32),
                "ChanceToInflictStatusStoneWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Stone, value.unwrap()as f32),
                "ChanceToInflictStatusWeaponBreakWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::WeaponBreak, value.unwrap()as f32),
                "ChanceToInflictStatusArmorBreakWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::ArmorBreak, value.unwrap()as f32),
                "ChanceToInflictStatusComaWhenHitPercentage" => BonusType::ChanceToInflictStatusWhenHitPercentage(StatusEffect::Coma, value.unwrap()as f32),
                "ResistanceToStatusPoisonPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Poison, value.unwrap() as i8 as f32),
                "ResistanceToStatusStunPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Stun, value.unwrap() as f32),
                "ResistanceToStatusFreezePercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Freeze, value.unwrap() as f32),
                "ResistanceToStatusCursePercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Curse, value.unwrap() as f32),
                "ResistanceToStatusBurningPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Burning, value.unwrap() as f32),
                "ResistanceToStatusBlindPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Blind, value.unwrap() as f32),
                "ResistanceToStatusSleepPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Sleep, value.unwrap() as f32),
                "ResistanceToStatusSilencePercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Silence, value.unwrap() as f32),
                "ResistanceToStatusChaosPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Chaos, value.unwrap() as f32),
                "ResistanceToStatusBleedingPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Bleeding, value.unwrap() as f32),
                "ResistanceToStatusStonePercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::Stone, value.unwrap() as f32),
                "ResistanceToStatusWeaponBreakPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::WeaponBreak, value.unwrap() as f32),
                "ResistanceToStatusArmorBreakPercentage" => BonusType::ResistanceToStatusPercentage(StatusEffect::ArmorBreak, value.unwrap() as f32),
                "BreakArmorPercentage" => BonusType::BreakArmorPercentage(value.unwrap() as i8),
                "BreakWeaponPercentage" => BonusType::BreakWeaponPercentage(value.unwrap() as i8),
                "ClassChangePercentageOnHit" => BonusType::ClassChangePercentageOnHit(value.unwrap() as i8),
                "LongRangeCriticalChance" => BonusType::LongRangeCriticalChance(value.unwrap() as i8),
                // Only when attacking with ranged weapon
                "IncreaseDamageAgainstClassBossBaseOnDef" => BonusType::IncreaseDamageAgainstClassBaseOnDef(MobClass::Boss),
                "IncreaseDamageAgainstClassNormalBaseOnDef" => BonusType::IncreaseDamageAgainstClassBaseOnDef(MobClass::Normal),
                "IncreaseDamageAgainstClassGuardianBaseOnDef" => BonusType::IncreaseDamageAgainstClassBaseOnDef(MobClass::Guardian),
                "PhysicalDamageAgainstClassBossPercentage" => BonusType::PhysicalDamageAgainstClassPercentage(MobClass::Boss, value.unwrap() as i8),
                "PhysicalDamageAgainstClassNormalPercentage" => BonusType::PhysicalDamageAgainstClassPercentage(MobClass::Normal, value.unwrap() as i8),
                "PhysicalDamageAgainstClassGuardianPercentage" => BonusType::PhysicalDamageAgainstClassPercentage(MobClass::Guardian, value.unwrap() as i8),
                "PhysicalDamageAgainstMobIdPercentage" => BonusType::PhysicalDamageAgainstMobIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "ResistanceDamageFromClassBossPercentage" => BonusType::ResistanceDamageFromClassPercentage(MobClass::Boss, value.unwrap() as i8),
                "ResistanceDamageFromClassNormalPercentage" => BonusType::ResistanceDamageFromClassPercentage(MobClass::Normal, value.unwrap() as i8),
                "ResistanceDamageFromClassGuardianPercentage" => BonusType::ResistanceDamageFromClassPercentage(MobClass::Guardian, value.unwrap() as i8),
                "ResistanceDamageFromElementNeutralPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Neutral, value.unwrap() as i8),
                "ResistanceDamageFromElementWaterPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Water, value.unwrap() as i8),
                "ResistanceDamageFromElementEarthPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Earth, value.unwrap() as i8),
                "ResistanceDamageFromElementFirePercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Fire, value.unwrap() as i8),
                "ResistanceDamageFromElementWindPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Wind, value.unwrap() as i8),
                "ResistanceDamageFromElementPoisonPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Poison, value.unwrap() as i8),
                "ResistanceDamageFromElementHolyPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Holy, value.unwrap() as i8),
                "ResistanceDamageFromElementDarkPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Dark, value.unwrap() as i8),
                "ResistanceDamageFromElementGhostPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Ghost, value.unwrap() as i8),
                "ResistanceDamageFromElementUndeadPercentage" => BonusType::ResistanceDamageFromElementPercentage(Element::Undead, value.unwrap() as i8),
                "ResistanceDamageFromRaceFormlessPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "ResistanceDamageFromRaceUndeadPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "ResistanceDamageFromRaceBrutePercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "ResistanceDamageFromRacePlantPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "ResistanceDamageFromRaceInsectPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "ResistanceDamageFromRaceFishPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "ResistanceDamageFromRaceDemonPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "ResistanceDamageFromRaceDemiHumanPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "ResistanceDamageFromRaceAngelPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "ResistanceDamageFromRaceDragonPercentage" => BonusType::ResistanceDamageFromRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "ResistanceDamageFromSizeSmallPercentage" => BonusType::ResistanceDamageFromSizePercentage(Size::Small, value.unwrap() as i8),
                "ResistanceDamageFromSizeMediumPercentage" => BonusType::ResistanceDamageFromSizePercentage(Size::Medium, value.unwrap() as i8),
                "ResistanceDamageFromSizeLargePercentage" => BonusType::ResistanceDamageFromSizePercentage(Size::Large, value.unwrap() as i8),

                "SkillDelayIncDecPercentage" => BonusType::SkillDelayIncDecPercentage(value.unwrap() as i8),
                "DoubleAttackChancePercentage" => BonusType::DoubleAttackChancePercentage(value.unwrap() as i8),
                "HealSkillPercentage" => BonusType::HealSkillPercentage(value.unwrap() as i8),
                "HealSkillIdPercentage" => BonusType::HealSkillIdPercentage(value.unwrap() as u32, value2.unwrap() as i8),
                "IgnoreDefClassNormal" => BonusType::IgnoreDefClass(MobClass::Normal),
                "IgnoreDefClassBoss" => BonusType::IgnoreDefClass(MobClass::Boss),
                "IgnoreDefClassGuardian" => BonusType::IgnoreDefClass(MobClass::Guardian),
                "IgnoreDefRaceAngel" => BonusType::IgnoreDefRace(MobRace::Angel),
                "IgnoreDefRaceBrute" => BonusType::IgnoreDefRace(MobRace::Brute),
                "IgnoreDefRaceDemiHuman" => BonusType::IgnoreDefRace(MobRace::DemiHuman),
                "IgnoreDefRaceDemon" => BonusType::IgnoreDefRace(MobRace::Demon),
                "IgnoreDefRaceDragon" => BonusType::IgnoreDefRace(MobRace::Dragon),
                "IgnoreDefRaceFish" => BonusType::IgnoreDefRace(MobRace::Fish),
                "IgnoreDefRaceFormless" => BonusType::IgnoreDefRace(MobRace::Formless),
                "IgnoreDefRaceInsect" => BonusType::IgnoreDefRace(MobRace::Insect),
                "IgnoreDefRacePlant" => BonusType::IgnoreDefRace(MobRace::Plant),
                "IgnoreDefRacePlayerHuman" => BonusType::IgnoreDefRace(MobRace::DemiHuman),
                "IgnoreDefRacePlayerDoram" => BonusType::IgnoreDefRace(MobRace::DemiHuman),
                "IgnoreDefRaceUndead" => BonusType::IgnoreDefRace(MobRace::Undead),
                "IgnoreDefRaceFormlessPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "IgnoreDefRaceUndeadPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "IgnoreDefRaceBrutePercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "IgnoreDefRacePlantPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "IgnoreDefRaceInsectPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "IgnoreDefRaceFishPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "IgnoreDefRaceDemonPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "IgnoreDefRaceDemiHumanPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "IgnoreDefRaceAngelPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "IgnoreDefRaceDragonPercentage" => BonusType::IgnoreDefRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "IgnoreMDefRaceFormlessPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "IgnoreMDefRaceUndeadPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "IgnoreMDefRaceBrutePercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "IgnoreMDefRacePlantPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "IgnoreMDefRaceInsectPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "IgnoreMDefRaceFishPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "IgnoreMDefRaceDemonPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "IgnoreMDefRaceDemiHumanPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "IgnoreMDefRaceAngelPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "IgnoreMDefRaceDragonPercentage" => BonusType::IgnoreMDefRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "IgnoreMDefClassNormalPercentage" => BonusType::IgnoreMDefClassPercentage(MobClass::Normal, value.unwrap() as i8),
                "IgnoreMDefClassBossPercentage" => BonusType::IgnoreMDefClassPercentage(MobClass::Boss, value.unwrap() as i8),
                "IgnoreMDefClassGuardianPercentage" => BonusType::IgnoreMDefClassPercentage(MobClass::Guardian, value.unwrap() as i8),
                "ResistanceRangeAttackPercentage" => BonusType::ResistanceRangeAttackPercentage(value.unwrap() as i8),
                "DamageRangedAtkPercentage" => BonusType::DamageRangedAtkPercentage(value.unwrap() as i8),
                "ResistanceMagicAttackPercentage" => BonusType::ResistanceMagicAttackPercentage(value.unwrap() as i8),
                "MagicAttackReflectChancePercentage" => BonusType::MagicAttackReflectChancePercentage(value.unwrap() as i8),
                "MeleeAttackReflectChancePercentage" => BonusType::PhysicalAttackReflectChancePercentage(value.unwrap() as i8),
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
                "GainExpWhenKillingRaceFormlessPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Formless, value.unwrap() as i8),
                "GainExpWhenKillingRaceUndeadPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Undead, value.unwrap() as i8),
                "GainExpWhenKillingRaceBrutePercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Brute, value.unwrap() as i8),
                "GainExpWhenKillingRacePlantPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Plant, value.unwrap() as i8),
                "GainExpWhenKillingRaceInsectPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Insect, value.unwrap() as i8),
                "GainExpWhenKillingRaceFishPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Fish, value.unwrap() as i8),
                "GainExpWhenKillingRaceDemonPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Demon, value.unwrap() as i8),
                "GainExpWhenKillingRaceDemiHumanPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::DemiHuman, value.unwrap() as i8),
                "GainExpWhenKillingRaceAngelPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Angel, value.unwrap() as i8),
                "GainExpWhenKillingRaceDragonPercentage" => BonusType::GainExpWhenKillingRacePercentage(MobRace::Dragon, value.unwrap() as i8),
                "GainZenyWhenKillingMonster" => BonusType::GainZenyWhenKillingMonster(value.unwrap() as u16, value2.unwrap() as i8),
                // "zeny" => BonusType::zeny, chance
                "HpDrainWhenAttackingPercentage" => BonusType::HpDrainWhenAttackingPercentage(value.unwrap() as i8, value2.unwrap() as i8),
                // hp "percentage" => BonusType::percentage, chance
                "SpDrainWhenAttackingPercentage" => BonusType::SpDrainWhenAttackingPercentage(value.unwrap() as i8, value2.unwrap() as i8),
                // sp "percentage" => BonusType::percentage, chance
                "SpDrainWhenAttackingRaceFormless" => BonusType::SpDrainWhenAttackingRace(MobRace::Formless, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceUndead" => BonusType::SpDrainWhenAttackingRace(MobRace::Undead, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceBrute" => BonusType::SpDrainWhenAttackingRace(MobRace::Brute, value.unwrap() as u16),
                "SpDrainWhenAttackingRacePlant" => BonusType::SpDrainWhenAttackingRace(MobRace::Plant, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceInsect" => BonusType::SpDrainWhenAttackingRace(MobRace::Insect, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceFish" => BonusType::SpDrainWhenAttackingRace(MobRace::Fish, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceDemon" => BonusType::SpDrainWhenAttackingRace(MobRace::Demon, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceDemiHuman" => BonusType::SpDrainWhenAttackingRace(MobRace::DemiHuman, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceAngel" => BonusType::SpDrainWhenAttackingRace(MobRace::Angel, value.unwrap() as u16),
                "SpDrainWhenAttackingRaceDragon" => BonusType::SpDrainWhenAttackingRace(MobRace::Dragon, value.unwrap() as u16),
                "SpDrainWhenKillingRaceFormless" => BonusType::SpDrainWhenKillingRace(MobRace::Formless, value.unwrap() as u16),
                "SpDrainWhenKillingRaceUndead" => BonusType::SpDrainWhenKillingRace(MobRace::Undead, value.unwrap() as u16),
                "SpDrainWhenKillingRaceBrute" => BonusType::SpDrainWhenKillingRace(MobRace::Brute, value.unwrap() as u16),
                "SpDrainWhenKillingRacePlant" => BonusType::SpDrainWhenKillingRace(MobRace::Plant, value.unwrap() as u16),
                "SpDrainWhenKillingRaceInsect" => BonusType::SpDrainWhenKillingRace(MobRace::Insect, value.unwrap() as u16),
                "SpDrainWhenKillingRaceFish" => BonusType::SpDrainWhenKillingRace(MobRace::Fish, value.unwrap() as u16),
                "SpDrainWhenKillingRaceDemon" => BonusType::SpDrainWhenKillingRace(MobRace::Demon, value.unwrap() as u16),
                "SpDrainWhenKillingRaceDemiHuman" => BonusType::SpDrainWhenKillingRace(MobRace::DemiHuman, value.unwrap() as u16),
                "SpDrainWhenKillingRaceAngel" => BonusType::SpDrainWhenKillingRace(MobRace::Angel, value.unwrap() as u16),
                "SpDrainWhenKillingRaceDragon" => BonusType::SpDrainWhenKillingRace(MobRace::Dragon, value.unwrap() as u16),
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
