use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct BattleFixture {
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
}

#[derive(Serialize, Deserialize)]
struct PassiveSkill {
    level: u32,
    skid: u32,
}

#[derive(Serialize, Deserialize)]
struct SupportiveSkill {
    #[serde(rename = "skid", skip_serializing_if = "Option::is_none")]
    skid: Option<u32>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    value: u32,
}