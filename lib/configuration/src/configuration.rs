use crate::serde_helper::*;
use accessor::{GettersAll, Setters, SettersAll};
use models::enums::class::JobName;
use models::enums::element::Element;
use models::enums::skill::{
    SkillCastTimeDelayType, SkillCopyType, SkillDamageFlags, SkillDamageType, SkillFlags,
    SkillState, SkillTargetType, SkillType, SkillUnitType,
};
use models::enums::unit::UnitTargetType;
use models::enums::weapon::{AmmoType, WeaponType};
use models::enums::EnumWithNumberValue;
use models::enums::{EnumWithMaskValueU64, EnumWithStringValue};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::{env, fs};
use models::enums::client_effect_icon::ClientEffectIcon;
use models::status_bonus::StatusBonusFlag;
use crate::bonus_type_wrapper::BonusTypeWrapper;

const DEFAULT_LOG_LEVEL: &str = "info";
const LOG_LEVELS: [&str; 4] = ["debug", "info", "warn", "error"];

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub game: GameConfig,
    pub database: DatabaseConfig,
    pub proxy: ProxyConfig,
    pub maps: MapConfig,
}

#[derive(Deserialize, Debug, Setters, Clone)]
pub struct ServerConfig {
    #[set]
    pub log_level: Option<String>,
    #[set]
    pub trace_packet: bool,
    #[set]
    pub log_exclude_pattern: Option<String>,
    pub accounts: Vec<u32>,
    pub port: u16,
    pub enable_visual_debugger: bool,
    pub packetver: u32,
}

#[derive(Deserialize, Debug, SettersAll, Clone)]
pub struct GameConfig {
    pub default_char_speed: u16,
    pub max_inventory: u16,
    pub max_base_level: u32,
    pub mob_density: f32,
    pub mob_move_frequency_when_player_around: f32,
    pub mob_move_frequency_when_no_player_around: f32,
    pub mob_spawn_refresh_frequency: f32,
    pub mob_action_refresh_frequency: f32,
    pub drop_rate: f32,
    pub drop_rate_mvp: f32,
    pub drop_rate_card: f32,
    pub base_exp_rate: f32,
    pub job_exp_rate: f32,
    pub max_stat_level: u16,
    pub mob_dropped_item_locked_to_owner_duration_in_secs: u16,
    pub player_dropped_item_locked_to_owner_duration_in_secs: u16,
    #[serde(skip)]
    pub status_point_rewards: Vec<StatusPointReward>,
    #[serde(skip)]
    pub status_point_raising_cost: Vec<StatusPointRaisingCost>,
    #[serde(skip)]
    pub exp_requirements: ExpRequirement,
}

#[derive(Deserialize, Default, Debug, SettersAll, Clone)]
pub struct ExpRequirement {
    pub base_next_level_requirement: BaseLevelRequirement,
    pub job_next_level_requirement: JobLevelRequirement,
}

#[derive(Deserialize, Default, Debug, SettersAll, Clone)]
pub struct BaseLevelRequirement {
    #[serde(rename = "Normal")]
    pub normal: Vec<u32>,
    #[serde(rename = "Transcendent")]
    pub transcendent: Vec<u32>,
}

#[derive(Deserialize, Default, Debug, SettersAll, Clone)]
pub struct JobLevelRequirement {
    #[serde(rename = "Novice")]
    pub novice: Vec<u32>,
    #[serde(rename = "FirstClass")]
    pub first_class: Vec<u32>,
    #[serde(rename = "SecondClass")]
    pub second_class: Vec<u32>,
    #[serde(rename = "TranscendedNovice")]
    pub transcended_novice: Vec<u32>,
    #[serde(rename = "TranscendedFirstClass")]
    pub transcended_first_class: Vec<u32>,
    #[serde(rename = "TranscendedSecondClass")]
    pub transcended_second_class: Vec<u32>,
    #[serde(rename = "TaekwonClass")]
    pub taekwon_class: Vec<u32>,
    #[serde(rename = "NinjaGunslingerClass")]
    pub gunslinger_class: Vec<u32>,
}

#[derive(Deserialize, Debug, SettersAll, Clone)]
pub struct StatusPointReward {
    pub level_min: u16,
    pub level_max: u16,
    pub reward: u16,
}

#[derive(Deserialize, Debug, SettersAll, Clone)]
pub struct StatusPointRaisingCost {
    pub level_min: u16,
    pub level_max: u16,
    pub raising_cost: u16,
}

#[derive(Deserialize, Debug, Setters, Clone)]
pub struct DatabaseConfig {
    pub db: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    #[set]
    pub password: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapConfig {
    pub cities: Vec<CityConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CityConfig {
    pub name: String,
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProxyConfig {
    pub remote_login_server_ip: String,
    pub remote_login_server_port: u16,
    pub remote_char_server_ip: String,
    pub remote_char_server_port: u16,
    pub local_char_server_port: u16,
    pub remote_map_server_ip: String,
    pub remote_map_server_port: u16,
    pub local_map_server_port: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone, GettersAll)]
pub struct InternalJobsConfig {
    level: HashMap<String, JobLevel>,
    pub jobs: BTreeMap<String, InternalJobConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone, GettersAll)]
pub struct JobLevel {
    #[serde(rename = "maxJobLevel")]
    max_job_level: u8,
    #[serde(rename = "minJobLevelToChangeJob")]
    min_job_level_to_change_job: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone, SettersAll, GettersAll)]
pub struct InternalJobConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    base_weight: Option<u32>,
    id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_sp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inherit_hp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_hp: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_sp: Option<Vec<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bonus_stats: Option<Vec<BTreeMap<String, i16>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_aspd: Option<BTreeMap<String, u32>>,
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
pub struct JobConfig {
    name: String,
    id: u32,
    base_weight: u32,
    base_hp: Vec<u32>,
    base_sp: Vec<u32>,
    base_aspd: BTreeMap<String, u32>,
    bonus_stats: Vec<BTreeMap<String, i16>>,
    job_level: JobLevel,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SkillsConfig {
    #[serde(rename = "skills", deserialize_with = "deserialize_skills")]
    pub skills: HashMap<u32, SkillConfig>,
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
#[allow(dead_code)]
pub struct SkillConfig {
    pub id: u32,
    pub name: String,
    pub description: String,
    #[serde(rename = "maxLevel")]
    max_level: u32,
    #[serde(
    rename = "type",
    deserialize_with = "deserialize_optional_string_enum",
    default
    )]
    skill_type: Option<SkillType>,
    #[serde(
    rename = "targetType",
    deserialize_with = "deserialize_string_enum",
    default = "SkillTargetType::default"
    )]
    target_type: SkillTargetType,
    #[serde(
    rename = "damageflags",
    deserialize_with = "deserialize_damage_flags",
    default
    )]
    damage_flags: Option<u64>,
    #[serde(
        rename = "bonusflags",
        deserialize_with = "deserialize_bonus_flags",
        default
    )]
    bonus_flags: Option<u64>,
    #[serde(
    rename = "flags",
    deserialize_with = "deserialize_skill_flags",
    default
    )]
    pub flags: Option<u64>,
    #[serde(default)]
    range: Option<i32>,
    #[serde(
    rename = "rangePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    range_per_level: Option<Vec<i32>>,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    damage_type: Option<SkillDamageType>,
    #[serde(rename = "hitCount", default)]
    hit_count: Option<i32>,
    #[serde(
    rename = "hitCountPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    hit_count_per_level: Option<Vec<i32>>,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    element: Option<Element>,
    element_per_level: Option<Vec<InternalSkillElement>>,
    #[serde(rename = "splashArea", default)]
    splash_area: Option<i32>,
    #[serde(
    rename = "splashAreaPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    splash_area_per_level: Option<Vec<i32>>,
    #[serde(rename = "activeInstance", default)]
    active_instance: Option<u32>,
    #[serde(
    rename = "activeInstancePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    active_instance_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "copyflags",
    deserialize_with = "deserialize_copy_flags",
    default
    )]
    copy_flags: Option<u64>,
    #[serde(rename = "castCancel", default)]
    cast_cancel: bool,
    #[serde(rename = "castDefenseReduction", default)]
    cast_defense_reduction: u32,
    #[serde(rename = "castTime", default)]
    cast_time: Option<u32>,
    #[serde(
    rename = "castTimePerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    cast_time_per_level: Option<Vec<u32>>,
    #[serde(rename = "afterCastActDelay", default)]
    after_cast_act_delay: Option<u32>,
    #[serde(
    rename = "afterCastActDelayPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    after_cast_act_delay_per_level: Option<Vec<u32>>,
    #[serde(rename = "afterCastActDelay", default)]
    after_cast_walk_delay: Option<u32>,
    #[serde(
    rename = "afterCastActDelayPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    after_cast_walk_delay_per_level: Option<Vec<u32>>,
    #[serde(rename = "duration1", default)]
    duration1: Option<u32>,
    #[serde(
    rename = "duration1PerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    duration1_per_level: Option<Vec<u32>>,
    #[serde(rename = "duration2", default)]
    duration2: Option<u32>,
    #[serde(
    rename = "duration2PerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    duration2_per_level: Option<Vec<u32>>,
    #[serde(rename = "cooldown", default)]
    cooldown: Option<u32>,
    #[serde(
    rename = "cooldownPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    cooldown_per_level: Option<Vec<u32>>,
    #[serde(rename = "fixedCastTime", default)]
    fixed_cast_time: Option<u32>,
    #[serde(
    rename = "fixedCastTimePerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    fixed_cast_time_per_level: Option<Vec<u32>>,
    #[serde(
    rename = "casttimeflags",
    deserialize_with = "deserialize_skill_cast_time_delay_flags",
    default
    )]
    cast_time_flags: Option<u64>,
    #[serde(
    rename = "castdelayflags",
    deserialize_with = "deserialize_skill_cast_time_delay_flags",
    default
    )]
    cast_delay_flags: Option<u64>,
    requires: Option<SkillRequirements>,
    #[serde(rename = "skiprequires")]
    skip_requires: Option<SkillSkipRequirements>,
    unit: Option<SkillUnit>,
    #[serde(rename = "dmgWaves", default)]
    dmg_waves: Option<u32>,

    #[serde(rename = "aoesize", default)]
    aoesize: Option<String>,
    #[serde(
    rename = "dmgAtkPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_atk_per_level: Option<Vec<f32>>,
    #[serde(rename = "dmgAtk", default)]
    dmg_atk: Option<f32>,
    #[serde(
    rename = "dmgMatkPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_matk_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "spLossPerSecondPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    sp_loss_per_second_per_level: Option<Vec<f32>>,
    #[serde(rename = "aoesplash", default)]
    aoesplash: Option<u32>,
    #[serde(
    rename = "knockbackPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    knockback_per_level: Option<Vec<u32>>,
    #[serde(rename = "knockback", default)]
    knockback: Option<u32>,
    #[serde(
    rename = "knockbackRangePerLevel",
    deserialize_with = "deserialize_tuples_range_u32",
    default
    )]
    knockback_range_per_level: Option<Vec<(u32, u32)>>,
    #[serde(
    rename = "trapHpPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    trap_hp_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "aoeactPerLevel",
    deserialize_with = "deserialize_tuples_string",
    default
    )]
    aoeact_per_level: Option<Vec<String>>,
    #[serde(rename = "dmgMatk", default)]
    dmg_matk: Option<f32>,
    #[serde(
    rename = "dmgWavesPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_waves_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgLvPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_lv_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "aoesizePerLevel",
    deserialize_with = "deserialize_tuples_string",
    default
    )]
    aoesize_per_level: Option<Vec<String>>,
    #[serde(
    rename = "craftPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    craft_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_per_level: Option<Vec<f32>>,
    #[serde(rename = "spLoss", default)]
    sp_loss: Option<f32>,
    #[serde(
    rename = "expPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    exp_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "skillDelayInSecPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    skill_delay_in_sec_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "healspPercentagePerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    healsp_percentage_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "dmgOuterPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_outer_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "healbasePerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    healbase_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "buyPricePercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    buy_price_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "healhpPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    healhp_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "healhpPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    healhp_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "dmgSPPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_s_p_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "maxLvPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    max_lv_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "plantHpPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    plant_hp_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "plantTypePerLevel",
    deserialize_with = "deserialize_tuples_string",
    default
    )]
    plant_type_per_level: Option<Vec<String>>,
    #[serde(
    rename = "healPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    heal_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "healPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    heal_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "stunDurationPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    stun_duration_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "refinePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    refine_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgDecayPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_decay_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "memosPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    memos_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "weightLimitPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    weight_limit_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "salePricePercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    sale_price_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "itemsPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    items_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "katarOffhandDmgPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    katar_offhand_dmg_percentage_per_level: Option<Vec<i32>>,

    #[serde(
    rename = "recursionPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    recursion_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "wallHpPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    wall_hp_per_level: Option<Vec<i32>>,

    #[serde(
    rename = "blindDurationPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    blind_duration_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "freezeDurationPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    freeze_duration_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "falconAtkPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    falcon_atk_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "mspdwallPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    mspdwall_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "autospellLimitPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    autospell_limit_per_level: Option<Vec<i32>>,
    #[serde(rename = "healhp", default)]
    healhp: Option<i32>,
    #[serde(
    rename = "hpmaxVitPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    hpmax_vit_per_level: Option<Vec<i32>>,
    #[serde(rename = "dmg", default)]
    dmg: Option<i32>,
    #[serde(
    rename = "dmgBowPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_bow_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "plantQuantityPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    plant_quantity_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "plantMovePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    plant_move_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "delaypartyInSecPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    delayparty_in_sec_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "delaypartyInSecPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    delayuser_in_sec_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "guardLvPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    guard_lv_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "playersPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    players_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "maxSpheresPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    max_spheres_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "spheresPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    spheres_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "disableDurationPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    disable_duration_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "spLossPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    sp_loss_percentage_per_level: Option<Vec<i32>>,

    #[serde(
    rename = "intuserIntpartnerPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    intuser_intpartner_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgSPPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_s_p_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "spRegenPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    sp_regen_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "hpLossPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    hp_loss_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "hpLossPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    hp_loss_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "delayPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    delay_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "dmgHPPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_h_p_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgSPChancePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_s_p_chance_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgIntPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    dmg_int_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "smdefPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    smdef_percentage_per_level: Option<Vec<i32>>,
    #[serde(rename = "skillFailureChance", default)]
    skill_failure_chance: Option<i32>,
    #[serde(rename = "snareDuration", default)]
    snare_duration: Option<i32>,
    #[serde(rename = "spRegenBonusRatio", default)]
    sp_regen_bonus_ratio: Option<f32>,
    #[serde(rename = "spRegenBonusPercentage", default)]
    sp_regen_bonus_percentage: Option<f32>,
    #[serde(
    rename = "distancePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    distance_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "mapPerLevel",
    deserialize_with = "deserialize_tuples_string",
    default
    )]
    map_per_level: Option<Vec<String>>,
    #[serde(
    rename = "weightPercentageLimitPercentagePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    weight_percentage_limit_percentage_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "spLossPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    sp_loss_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "maxRefinePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    max_refine_per_level: Option<Vec<i32>>,
    #[serde(
    rename = "dmgPistolPerLevel",
    deserialize_with = "deserialize_tuples_f32",
    default
    )]
    dmg_pistol_per_level: Option<Vec<f32>>,
    #[serde(
    rename = "dmgRangePerLevel",
    deserialize_with = "deserialize_tuples_range_f32",
    default
    )]
    dmg_range_per_level: Option<Vec<(f32, f32)>>,
    #[serde(
    rename = "bonusToSelf",
    default
    )]
    bonus_to_self: Vec<BonusPerLevel>,
    #[serde(
    rename = "bonusToTarget",
    default
    )]
    bonus_to_target: Vec<BonusPerLevel>,
    #[serde(
    rename = "bonusToParty",
    default
    )]
    bonus_to_party: Vec<BonusPerLevel>,
    #[serde(
        rename = "bonusToSelfBeforeHit",
        default
    )]
    bonus_to_self_before_hit: Vec<BonusPerLevel>,
    #[serde(
        rename = "bonusToTargetBeforeHit",
        default
    )]
    bonus_to_target_before_hit: Vec<BonusPerLevel>,
    #[serde(rename = "effectClientIcon", deserialize_with = "deserialize_optional_string_enum",default)]
    effect_client_icon: Option<ClientEffectIcon>,
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
pub struct BonusPerLevel {
    pub value: BonusTypeWrapper,
    pub level: Option<u8>,
}


#[derive(Deserialize, Debug, Clone, GettersAll)]
#[allow(dead_code)]
pub struct SkillRequirements {
    #[serde(rename = "hpcost", default)]
    hp_cost: Option<u32>,
    #[serde(
    rename = "hpcostPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    hp_cost_per_level: Option<Vec<u32>>,
    #[serde(rename = "spcost", default)]
    sp_cost: Option<u32>,
    #[serde(
    rename = "spcostPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    sp_cost_per_level: Option<Vec<u32>>,
    #[serde(rename = "hpratecost", default)]
    hp_rate_cost: Option<u32>,
    #[serde(
    rename = "hpratecostPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    hp_rate_cost_per_level: Option<Vec<u32>>,
    #[serde(rename = "spratecost", default)]
    sp_rate_cost: Option<u32>,
    #[serde(
    rename = "spratecostPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    sp_rate_cost_per_level: Option<Vec<u32>>,
    #[serde(rename = "zenycost", default)]
    zeny_cost: Option<u32>,
    #[serde(
    rename = "zenycostPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    zeny_cost_per_level: Option<Vec<u32>>,
    #[serde(
    rename = "weaponFlags",
    deserialize_with = "deserialize_weapon_flags",
    default
    )]
    weapon_flags: Option<u64>,
    #[serde(
    rename = "ammoFlags",
    deserialize_with = "deserialize_ammo_flags",
    default
    )]
    ammo_flags: Option<u64>,
    #[serde(rename = "ammoAmount", default)]
    ammo_amount: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    state: Option<SkillState>,
    #[serde(rename = "spiritSphereCost", default)]
    sphere_cost: Option<u32>,
    #[serde(
    rename = "spiritSphereCostPerLevel",
    deserialize_with = "deserialize_tuples_u32",
    default
    )]
    sphere_cost_per_level: Option<Vec<u32>>,
    #[serde(rename = "itemcost", default)]
    item_cost: Vec<InternalSkillItemCost>,
    #[serde(rename = "joblevel", default)]
    job_level: Option<u32>
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
#[allow(dead_code)]
pub struct SkillSkipRequirements {
    #[serde(rename = "itemcost", default)]
    item_cost: Option<InternalSkillSkipItemCost>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct SkillUnit {
    id: String,
    #[serde(rename = "alternateId", default)]
    alternate_id: Option<String>,
    #[serde(default)]
    layout: Option<i32>,
    #[serde(
    rename = "layoutPerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    layout_per_level: Option<Vec<i32>>,
    #[serde(default)]
    range: Option<i32>,
    #[serde(
    rename = "rangePerLevel",
    deserialize_with = "deserialize_tuples_i32",
    default
    )]
    range_per_level: Option<Vec<i32>>,
    interval: i32,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    target: Option<UnitTargetType>,
    #[serde(deserialize_with = "deserialize_skill_unit_flags", default)]
    flag: Option<u64>,
}

macro_rules! deserialize_tuples {
    ($function:ident, $type:ty, $max:expr) => {
        fn $function<'de, D>(deserializer: D) -> Result<Option<Vec<$type>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s: Vec<HashMap<String, $type>> = Deserialize::deserialize(deserializer)?;
            let mut res: Vec<$type> = vec![$max; s.len() + 1];
            for x in s.iter() {
                let (_, value) = x.iter().find(|(k, _)| k.as_str() != "level").unwrap();
                if *x.get("level").unwrap() as usize >= res.len() {
                    return Err(serde::de::Error::custom("Level is out of bounds"));
                }
                let _ = std::mem::replace(&mut res[*x.get("level").unwrap() as usize], *value);
            }
            Ok(Some(res))
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
struct RangeTuple<T> {
    level: usize,
    value: Vec<T>,
}

fn deserialize_tuples_range_u32<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<(u32, u32)>>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Vec<RangeTuple<u32>> = Deserialize::deserialize(deserializer)?;
    let mut res: Vec<(u32, u32)> = vec![(0, 0); s.len() + 1];
    for x in s.iter() {
        let _ = std::mem::replace(&mut res[x.level], (x.value[0], x.value[1]));
    }
    Ok(Some(res))
}

fn deserialize_tuples_range_f32<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<(f32, f32)>>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Vec<RangeTuple<f32>> = Deserialize::deserialize(deserializer)?;
    let mut res: Vec<(f32, f32)> = vec![(0.0, 0.0); s.len() + 1];
    for x in s.iter() {
        let _ = std::mem::replace(&mut res[x.level], (x.value[0], x.value[1]));
    }
    Ok(Some(res))
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
struct StringTuple {
    level: usize,
    value: String,
}

fn deserialize_tuples_string<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Vec<StringTuple> = Deserialize::deserialize(deserializer)?;
    let mut res: Vec<String> = vec![String::new(); s.len() + 1];
    for x in s.iter() {
        let _ = std::mem::replace(&mut res[x.level], x.value.clone());
    }
    Ok(Some(res))
}

deserialize_tuples!(deserialize_tuples_i32, i32, i32::MAX);
deserialize_tuples!(deserialize_tuples_u32, u32, u32::MAX);
deserialize_tuples!(deserialize_tuples_f32, f32, f32::MAX);

fn deserialize_skills<'de, D>(deserializer: D) -> Result<HashMap<u32, SkillConfig>, D::Error>
    where
        D: Deserializer<'de>,
{
    let skills: Vec<SkillConfig> = Deserialize::deserialize(deserializer)?;
    let mut skills_map: HashMap<u32, SkillConfig> = Default::default();
    skills.iter().for_each(|skill| {
        skills_map.insert(skill.id, skill.clone());
    });

    Ok(skills_map)
}

fn deserialize_damage_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillDamageFlags>(deserializer)
}

fn deserialize_copy_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillCopyType>(deserializer)
}

fn deserialize_skill_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillFlags>(deserializer)
}

fn deserialize_bonus_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, StatusBonusFlag>(deserializer)
}

fn deserialize_weapon_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, WeaponType>(deserializer)
}

fn deserialize_ammo_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, AmmoType>(deserializer)
}

fn deserialize_skill_cast_time_delay_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillCastTimeDelayType>(deserializer)
}

fn deserialize_skill_unit_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillUnitType>(deserializer)
}

fn deserialize_flags<'de, D, MaskEnum>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
        MaskEnum: EnumWithMaskValueU64 + EnumWithStringValue,
{
    let s: HashMap<String, bool> = Deserialize::deserialize(deserializer)?;
    let flags: u64 = s.iter().fold(0, |acc, (k, v)| {
        if *v {
            let mask_enum = MaskEnum::from_string_ignore_case(k);
            acc | mask_enum.as_flag()
        } else {
            acc
        }
    });
    Ok(Some(flags))
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
#[allow(dead_code)]
pub struct InternalSkillElement {
    level: u32,
    #[serde(deserialize_with = "deserialize_optional_string_enum")]
    element: Option<Element>,
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
#[allow(dead_code)]
pub struct InternalSkillItemCost {
    item: String,
    amount: u32,
    level: Option<u32>,
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
#[allow(dead_code)]
pub struct InternalSkillSkipItemCost {
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    state: Option<SkillState>,
}

#[derive(Deserialize, Default, Debug, SettersAll, Clone)]
struct InternalJobsSkillTreeConfig {
    jobs_tree: HashMap<String, InternalSkillsTreeConfig>,
}

#[derive(Deserialize, Default, Debug, SettersAll, Clone)]
struct InternalSkillsTreeConfig {
    tree: Option<Vec<SkillInTree>>,
    inherit: Option<Vec<String>>,
}

#[derive(Deserialize, Default, Debug, SettersAll, GettersAll, Clone)]
pub struct SkillInTree {
    name: String,
    #[serde(default, rename = "jobLevel")]
    job_level: u8,
    requires: Option<Vec<SkillTreeRequirement>>,
}

#[derive(Deserialize, Default, Debug, SettersAll, GettersAll, Clone)]
pub struct SkillTreeRequirement {
    name: String,
    level: u8,
}

#[derive(Deserialize, Default, Debug, SettersAll, GettersAll, Clone)]
pub struct JobSkillTree {
    name: String,
    tree: Vec<SkillInTree>,
    parent_skills: HashMap<String, Vec<SkillInTree>>,
}

impl Config {
    pub fn load(root: &str) -> Result<Config, String> {
        let path = Path::new(root).join(Path::new("config.json"));
        if !path.exists() {
            return Err(format!(
                "config.json file does not exists at {}",
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        let mut config: Config = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        match env::var("DATABASE_PASSWORD") {
            Ok(password) => config.database.set_password(Some(password)),
            Err(_) => {
                return Err("DATABASE_PASSWORD env is missing. please provide this env".to_string());
            }
        }

        if config.server.log_level.is_some() {
            let log_level = config.server.log_level.as_ref().unwrap();
            if !LOG_LEVELS.contains(&log_level.as_str()) {
                println!("Provided log level \"{}\" is not allowed. Valid values are {}, default to \"{}\"", log_level, LOG_LEVELS.join(", "), DEFAULT_LOG_LEVEL);
                config
                    .server
                    .set_log_level(Some(DEFAULT_LOG_LEVEL.to_string()));
            }
        } else {
            config
                .server
                .set_log_level(Some(DEFAULT_LOG_LEVEL.to_string()));
        }
        if config.server.log_exclude_pattern.is_none() {
            config
                .server
                .set_log_exclude_pattern(Some("none".to_string()));
        }
        let file_path = "./config/status_point_reward.json";
        Self::set_config_status_point_rewards(&mut config, file_path).unwrap();
        let file_path = "./config/status_point_raising_cost.json";
        Self::set_config_status_point_raising_cost(&mut config, file_path).unwrap();
        let file_path = "./config/exp.json";
        Self::set_exp_requirements(&mut config, file_path).unwrap();

        Ok(config)
    }

    pub fn set_config_status_point_rewards(
        config: &mut Config,
        file_path: &str,
    ) -> Result<(), String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!(
                "{} file does not exists at {}",
                file_path,
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        config.game.status_point_rewards =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(())
    }
    pub fn set_config_status_point_raising_cost(
        config: &mut Config,
        file_path: &str,
    ) -> Result<(), String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!(
                "{} file does not exists at {}",
                file_path,
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        config.game.status_point_raising_cost =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(())
    }
    pub fn set_exp_requirements(config: &mut Config, file_path: &str) -> Result<(), String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!(
                "{} file does not exists at {}",
                file_path,
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        config.game.exp_requirements =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(())
    }

    pub fn load_jobs_config(root: &str) -> Result<Vec<JobConfig>, String> {
        let path = Path::new(root).join("config/job.json");
        if !path.exists() {
            return Err(format!(
                "config/job.json file does not exists at {}",
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        let internal_configs: InternalJobsConfig =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

        let mut job_configs: Vec<JobConfig> = vec![];
        let default_values = internal_configs
            .jobs
            .get("default")
            .expect("Expect jobs.default config");
        for (name, config) in internal_configs.jobs.iter() {
            if name == "default" {
                continue;
            }
            let mut base_aspd = Self::resolve_inherited_config(
                name,
                config,
                &internal_configs,
                "base_aspd",
                |_conf| None,
                |conf| conf.base_aspd.clone(),
            )
                .unwrap_or_default();
            default_values
                .base_aspd
                .as_ref()
                .expect("Expect jobs.default to have base_aspd")
                .iter()
                .for_each(|(weapon, value)| {
                    base_aspd.entry(weapon.to_string()).or_insert(*value);
                });
            let job = JobName::from_value(
                config
                    .id
                    .unwrap_or_else(|| panic!("Expect job {name} to have id but found none"))
                    as usize,
            );
            let job_level = if job.is_novice() {
                internal_configs
                    .level
                    .get("novice")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_first_class() && job.is_rebirth() {
                internal_configs
                    .level
                    .get("firstClassHigh")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_second_class() && job.is_rebirth() {
                internal_configs
                    .level
                    .get("secondClassHigh")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_first_class() && !job.is_rebirth() {
                internal_configs
                    .level
                    .get("firstClass")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_second_class() && !job.is_rebirth() {
                internal_configs
                    .level
                    .get("secondClass")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_taekwon() {
                internal_configs
                    .level
                    .get("taekwon")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_gunslinger_ninja() {
                internal_configs
                    .level
                    .get("gunslingerNinja")
                    .expect("Expect level.novice config")
                    .clone()
            } else if job.is_supernovice() {
                internal_configs
                    .level
                    .get("superNovice")
                    .expect("Expect level.novice config")
                    .clone()
            } else {
                panic!(
                    "Can't find job level configuration for job {}::{}",
                    name,
                    job.value()
                );
            };
            job_configs.push(JobConfig {
                id: job.value() as u32,
                name: name.clone(),
                job_level,
                base_weight: Self::resolve_inherited_config(
                    name,
                    config,
                    &internal_configs,
                    "base_weight",
                    |_conf| None,
                    |conf| conf.base_weight,
                )
                    .or_else(|| {
                        Some(
                            default_values
                                .base_weight
                                .expect("Expect jobs.default to have base_weight"),
                        )
                    })
                    .unwrap(),
                base_hp: Self::resolve_inherited_config(
                    name,
                    config,
                    &internal_configs,
                    "inherit_hp",
                    |conf| conf.inherit_hp.as_ref(),
                    |conf| conf.base_hp.clone(),
                )
                    .unwrap_or_else(|| {
                        panic!("job config for class {name}: expected to find property base_hp")
                    }),
                base_sp: Self::resolve_inherited_config(
                    name,
                    config,
                    &internal_configs,
                    "inherit_sp",
                    |conf| conf.inherit_sp.as_ref(),
                    |conf| conf.base_sp.clone(),
                )
                    .unwrap_or_else(|| {
                        panic!("job config for class {name}: expected to find property base_sp")
                    }),
                bonus_stats: Self::resolve_inherited_config(
                    name,
                    config,
                    &internal_configs,
                    "bonus_stats",
                    |_conf| None,
                    |conf| conf.bonus_stats.clone(),
                ).unwrap_or_else(|| {
                    panic!("job config for class {name}: expected to find property bonus_stats")
                }),
                base_aspd,
            });
        }
        Ok(job_configs)
    }

    pub fn load_jobs_skill_tree(root: &str) -> Result<Vec<JobSkillTree>, String> {
        let path = Path::new(root).join("config/skill_tree.json");
        if !path.exists() {
            return Err(format!(
                "config/job.json file does not exists at {}",
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        let internal_jobs_skill_tree_config: InternalJobsSkillTreeConfig =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let mut jobs_skill_tree: Vec<JobSkillTree> = vec![];
        for (name, config) in internal_jobs_skill_tree_config.jobs_tree.iter() {
            jobs_skill_tree.push(JobSkillTree {
                name: name.to_string(),
                tree: config.tree.as_ref().unwrap_or(&vec![]).clone(),
                parent_skills: Default::default(),
            });
        }
        for (name, config) in internal_jobs_skill_tree_config.jobs_tree.iter() {
            if let Some(inherit) = &config.inherit {
                let parent_trees = jobs_skill_tree
                    .iter()
                    .filter(|job_tree| inherit.contains(&job_tree.name))
                    .cloned()
                    .collect::<Vec<JobSkillTree>>();
                let job_tree = jobs_skill_tree
                    .iter_mut()
                    .find(|job_tree| job_tree.name.eq(name))
                    .unwrap();
                for parent_tree in parent_trees {
                    job_tree
                        .parent_skills
                        .insert(parent_tree.name, parent_tree.tree);
                }
            }
        }
        Ok(jobs_skill_tree)
    }

    pub fn load_skills_config(root: &str) -> Result<HashMap<u32, SkillConfig>, String> {
        let path = Path::new(root).join("config/skill.json");
        if !path.exists() {
            return Err(format!(
                "config/skill.json file does not exists at {}",
                env::current_dir().unwrap().join(path).to_str().unwrap()
            ));
        }
        let internal_configs: SkillsConfig =
            serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(internal_configs.skills)
    }

    fn resolve_inherited_config<T, F1, F2>(
        name: &String,
        current_config: &InternalJobConfig,
        configs: &InternalJobsConfig,
        inherit_name: &str,
        inherited_property_fn: F1,
        defined_property_fn: F2,
    ) -> Option<T>
        where
            F1: Fn(&InternalJobConfig) -> Option<&String>,
            F2: Fn(&InternalJobConfig) -> Option<T>,
    {
        if defined_property_fn(current_config).is_some() {
            return defined_property_fn(current_config);
        }
        return if let Some(inherit) = current_config.inherit.as_ref() {
            let inherited_config = configs.jobs.get(inherit).unwrap_or_else(|| {
                panic!("job config for class {name}: inherit \"{inherit}\" was not found")
            });
            Self::resolve_inherited_config(
                name,
                inherited_config,
                configs,
                inherit_name,
                inherited_property_fn,
                defined_property_fn,
            )
        } else if let Some(inherit) = inherited_property_fn(current_config) {
            let inherited_config = configs.jobs.get(inherit).unwrap_or_else(|| {
                panic!("job config for class {name}: {inherit_name} \"{inherit}\" was not found")
            });
            Self::resolve_inherited_config(
                name,
                inherited_config,
                configs,
                inherit_name,
                inherited_property_fn,
                defined_property_fn,
            )
        } else {
            defined_property_fn(current_config)
        };
    }

    pub fn packetver(&self) -> u32 {
        self.server.packetver
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::configuration::SkillsConfig;

    #[test]
    fn test_deserialize_skill_config() {
        let json = fs::read_to_string("../../config/skill.json").unwrap();
        let mut config_deserializer = serde_json::Deserializer::from_str(&json);
        let result: Result<SkillsConfig, _> = serde_path_to_error::deserialize(&mut config_deserializer);
        match result {
            Err(err) => {
                let path = err.path().to_string();
                println!("Path in error {}", path);
                panic!("{}", err);
            }
            Ok(internal_configs) => {
                let skill = internal_configs.skills.get(&29).unwrap();
                assert!(!skill.bonus_to_target.is_empty());
            }
        }
    }
}