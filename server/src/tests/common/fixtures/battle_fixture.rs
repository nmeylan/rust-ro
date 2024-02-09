use std::{env, fs};
use std::fmt::Formatter;
use std::path::Path;
use std::ptr::eq;
use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, SeqAccess, Visitor};
use crate::tests::common::character_helper::equip_item_from_id;

#[derive(Deserialize, GettersAll, Debug)]
pub struct BattleFixture {
    #[serde(rename = "_id")]
    id: String,
    job: String,
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
    matk_max: u16
}
#[derive(GettersAll, Debug)]
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

#[derive(Deserialize, GettersAll, Debug)]
pub struct SkillLevel {
    level: u8,
    skid: u32,
}

#[derive(Deserialize, GettersAll, Debug)]
pub struct Equipment {
    #[serde(rename = "itemId")]
    item_id: i32,
    refinement: u8,
    #[serde(default)]
    cards: Vec<Card>,
}

#[derive(Deserialize, GettersAll, Debug)]
pub struct Card {
    #[serde(rename = "itemId")]
    item_id: u32,
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
