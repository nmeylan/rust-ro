use std::{env, fs};
use std::path::Path;
use serde::{Deserialize};

#[derive(Deserialize, GettersAll, Debug)]
pub struct BattleFixture {
    job: String,
    base_level: u32,
    job_level: u32,
    str: u16,
    agi: u16,
    vit: u16,
    dex: u16,
    int: u16,
    luk: u16,
    #[serde(default)]
    speed_potion: Option<u32>,
    #[serde(default)]
    weapon: Option<String>,
    #[serde(default)]
    weapon_refinement: u32,
    #[serde(default)]
    weapon_card1: Option<String>,
    #[serde(default)]
    weapon_card2: Option<String>,
    #[serde(default)]
    weapon_card3: Option<String>,
    #[serde(default)]
    weapon_card4: Option<String>,
    #[serde(default)]
    headgear_upper:Option<String>,
    #[serde(default)]
    headgear_upper_card: Option<String>,
    #[serde(default)]
    headgear_middle: Option<String>,
    #[serde(default)]
    headgear_middle_card: Option<String>,
    #[serde(default)]
    headgear_lower: Option<String>,
    #[serde(default)]
    shield: Option<String>,
    #[serde(default)]
    shield_card: Option<String>,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    body_card: Option<String>,
    #[serde(default)]
    shoulder: Option<String>,
    #[serde(default)]
    shoulder_card: Option<String>,
    #[serde(default)]
    shoes: Option<String>,
    #[serde(default)]
    shoes_card: Option<String>,
    #[serde(default)]
    accessory_left: Option<String>,
    #[serde(default)]
    accessory_left_card: Option<String>,
    #[serde(default)]
    accessory_right: Option<String>,
    #[serde(default)]
    accessory_right_card: Option<String>,
    #[serde(default)]
    passive_skills: Vec<SkillLevel>,
    #[serde(default)]
    weapon_element: Option<u32>,
    #[serde(default)]
    supportive_skills: Vec<SupportiveSkill>,
    #[serde(default)]
    headgear_upper_refinement: Option<u32>,
    #[serde(default)]
    body_refinement: Option<u32>,
    #[serde(default)]
    shield_refinement: Option<u32>,
    #[serde(default)]
    shoulder_refinement: Option<u32>,
    #[serde(default)]
    shoes_refinement: Option<u32>,
    #[serde(default)]
    ammo: Option<String>,
    skill_to_use: SkillLevel,
    expected: Expected,
    target: String
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
}

#[derive(Deserialize, GettersAll, Debug)]
pub struct SkillLevel {
    level: u8,
    skid: u32,
}

#[derive(Deserialize, GettersAll, Debug)]
pub struct SupportiveSkill {
    #[serde(rename = "skid", skip_serializing_if = "Option::is_none")]
    skid: Option<u32>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    value: u32,
}


#[derive(Deserialize, GettersAll, Debug)]
pub struct Expected {
    weapon_min_atk: u32,
    weapon_avg_atk: f32,
    weapon_max_atk: u32,
    base_atk: u32,
    hit_ratio: f32,
    #[serde(default)]
    critical_rate: Option<f32>,
    #[serde(default)]
    critical_damage_min: Option<u32>,
    #[serde(default)]
    critical_damage_max: Option<u32>,
    min_dmg: u32,
    avg_dmg: f32,
    max_dmg: u32,
    dps: f32,
}
