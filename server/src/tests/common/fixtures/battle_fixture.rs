use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use models::status::KnownSkill;

#[derive(Deserialize, GettersAll)]
pub struct BattleFixture {
    job: u32,
    base_level: u32,
    job_level: u32,
    str: u32,
    agi: u32,
    vit: u32,
    dex: u32,
    int: u32,
    luk: u32,
    weapon_type: u32,
    speed_potion: u32,
    weapon: u32,
    weapon_refinement: u32,
    weapon_card1: u32,
    weapon_card2: u32,
    weapon_card3: u32,
    weapon_card4: u32,
    headgear_upper: u32,
    headgear_upper_card: u32,
    headgear_middle: u32,
    headgear_middle_card: u32,
    headgear_lower: u32,
    shield: u32,
    shield_card: u32,
    body: u32,
    body_card: u32,
    shoulder: u32,
    shoulder_card: u32,
    shoes: u32,
    shoes_card: u32,
    accessory_left: u32,
    accessory_left_card: u32,
    accessory_right: u32,
    accessory_right_card: u32,
    passive_skills: Vec<PassiveSkill>,
    weapon_element: u32,
    supportive_skills: Vec<SupportiveSkill>,
    headgear_upper_refinement: u32,
    body_refinement: u32,
    shield_refinement: u32,
    shoulder_refinement: u32,
    shoes_refinement: u32,
    // skill_to_use: KnownSkill
}

impl BattleFixture {
    pub fn load(path: &str) -> Vec<Self> {
        let path = Path::new(path);
        if !path.exists() {
            panic!("fixture file does not exists at {}", path.to_str().unwrap());
        }
        let fixtures: Vec<BattleFixture> = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        fixtures
    }
}

#[derive(Deserialize, GettersAll)]
pub struct PassiveSkill {
    level: u32,
    skid: u32,
}

#[derive(Deserialize, GettersAll)]
pub struct SupportiveSkill {
    #[serde(rename = "skid", skip_serializing_if = "Option::is_none")]
    skid: Option<u32>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    value: u32,
}


#[derive(Deserialize, GettersAll)]
pub struct Expected {
    weapon_min_atk: u32,
    weapon_avg_atk: u32,
    weapon_max_atk: u32,
    base_atk: u32,
    hit_ratio: f32,
    critical_rate: f32,
    critical_damage_min: u32,
    critical_damage_max: u32,
    min_dmg: u32,
    avg_dmg: f32,
    max_dmg: u32,
    dps: f32,
}
