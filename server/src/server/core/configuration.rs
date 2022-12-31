use std::{env, fs};
use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Deserializer};

use accessor::Setters;
use r#enum::{EnumWithMaskValue, EnumWithStringValue};
use crate::server::enums::element::Element;
use crate::server::enums::skill::{SkillCastTimeDelayType, SkillCopyType, SkillDamageFlags, SkillDamageType, SkillFlags, SkillRequirement, SkillTargetType, SkillType, SkillUnitType};
use crate::server::enums::unit::UnitTargetType;
use crate::server::enums::weapon::{AmmoType, WeaponType};

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
    pub log_exclude_pattern: Option<String>,
    pub accounts: Vec<u32>,
    pub port: u16,
    pub enable_visual_debugger: bool,
    pub packetver: u32,
}

#[derive(Deserialize, Debug, Setters, Clone)]
pub struct GameConfig {
    #[set]
    pub default_char_speed: u16,
    #[set]
    pub max_inventory: u16,
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

#[derive(Deserialize, Debug, Clone)]
struct InternalJobsConfig {
    jobs: HashMap<String, InternalJobConfig>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalJobConfig {
    base_weight: Option<u32>,
    id: Option<u32>,
    base_exp_group: Option<String>,
    job_exp_group: Option<String>,
    inherit: Option<String>,
    inherit_sp: Option<String>,
    inherit_hp: Option<String>,
    base_hp: Option<Vec<u32>>,
    base_sp: Option<Vec<u32>>,
    base_aspd: Option<HashMap<String, u32>>,
}

#[derive(Deserialize, Debug, Clone, GettersAll)]
pub struct JobConfig {
    name: String,
    id: u32,
    base_weight: u32,
    base_exp_group: String,
    job_exp_group: String,
    base_hp: Vec<u32>,
    base_sp: Vec<u32>,
    base_aspd: HashMap<String, u32>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillsConfig {
    #[serde(rename = "skill")]
    skills: Vec<InternalSkillConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InternalSkillConfig {
    id: u32,
    name: String,
    description: String,
    #[serde(rename = "maxLevel")]
    max_level: u32,
    #[serde(rename = "type", deserialize_with = "deserialize_optional_enum", default)]
    skill_type: Option<SkillType>,
    #[serde(rename = "targetType", deserialize_with = "deserialize_enum", default = "SkillTargetType::default")]
    target_type: SkillTargetType,
    #[serde(rename = "damageflags", deserialize_with = "deserialize_damage_flags", default)]
    damage_flags: Option<u64>,
    #[serde(rename = "flags", deserialize_with = "deserialize_skill_flags", default)]
    flags: Option<u64>,
    #[serde(default)]
    range: Option<i32>,
    #[serde(rename = "rangePerLevel", default)]
    range_per_level: Option<Vec<InternalSkillRange>>,
    #[serde(deserialize_with = "deserialize_optional_enum", default)]
    hit: Option<SkillDamageType>,
    #[serde(rename = "hitCount", default)]
    hit_count: Option<i32>,
    #[serde(rename = "hitCountPerLevel", default)]
    hit_count_per_level: Option<Vec<InternalSkillHitCount>>,
    #[serde(deserialize_with = "deserialize_optional_enum", default)]
    element: Option<Element>,
    element_per_level: Option<Vec<InternalSkillElement>>,
    #[serde(rename = "splashArea", default)]
    splash_area: Option<i32>,
    #[serde(rename = "splashAreaPerLevel", default)]
    splash_area_per_level: Option<Vec<InternalSkillSplashArea>>,
    #[serde(rename = "activeInstance", default)]
    active_instance: Option<u32>,
    #[serde(rename = "activeInstancePerLevel", default)]
    active_instance_per_level: Option<Vec<InternalSkillActiveInstance>>,
    #[serde(rename = "knockback", default)]
    knockback: Option<u32>,
    #[serde(rename = "knockbackPerLevel", default)]
    knockback_per_level: Option<Vec<InternalSkillKnockback>>,
    #[serde(rename = "copyflags", deserialize_with = "deserialize_copy_flags", default)]
    copy_flags: Option<u64>,
    no_near_npc: Option<InternalSkillNoNearNPC>,
    #[serde(rename = "castCancel", default)]
    cast_cancel: bool,
    #[serde(rename = "castDefenseReduction", default)]
    cast_defense_reduction: u32,
    #[serde(rename = "castTime", default)]
    cast_time: Option<u32>,
    #[serde(rename = "castTimePerLevel", default)]
    cast_time_per_level: Option<Vec<InternalSkillCastTime>>,
    #[serde(rename = "afterCastActDelay", default)]
    after_cast_act_delay: Option<u32>,
    #[serde(rename = "afterCastActDelayPerLevel", default)]
    after_cast_act_delay_per_level: Option<Vec<InternalSkillAfterCastActDelay>>,
    #[serde(rename = "afterCastActDelay", default)]
    after_cast_walk_delay: Option<u32>,
    #[serde(rename = "afterCastActDelayPerLevel", default)]
    after_cast_walk_delay_per_level: Option<Vec<InternalSkillAfterCastWalkDelay>>,
    #[serde(rename = "duration1", default)]
    duration1: Option<u32>,
    #[serde(rename = "duration1PerLevel", default)]
    duration1_per_level: Option<Vec<InternalSkillDuration1>>,
    #[serde(rename = "duration2", default)]
    duration2: Option<u32>,
    #[serde(rename = "duration2PerLevel", default)]
    duration2_per_level: Option<Vec<InternalSkillDuration2>>,
    #[serde(rename = "cooldown", default)]
    cooldown: Option<u32>,
    #[serde(rename = "cooldownPerLevel", default)]
    cooldown_per_level: Option<Vec<InternalSkillCooldown>>,
    #[serde(rename = "fixedCastTime", default)]
    fixed_cast_time: Option<u32>,
    #[serde(rename = "fixedCastTimePerLevel", default)]
    fixed_cast_time_per_level: Option<Vec<InternalSkillFixedCastTime>>,
    #[serde(rename = "casttimeflags", deserialize_with = "deserialize_skill_cast_time_delay_flags", default)]
    cast_time_flags: Option<u64>,
    #[serde(rename = "castdelayflags", deserialize_with = "deserialize_skill_cast_time_delay_flags", default)]
    cast_delay_flags: Option<u64>,
    requires: Option<InternalSkillRequires>,
    unit: Option<InternalSkillUnit>,
}

fn deserialize_optional_enum<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where D: Deserializer<'de>,
          T: EnumWithStringValue {
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(Some(T::from_string_ignore_case(s)))
}

fn deserialize_enum<'de, D, T>(deserializer: D) -> Result<T, D::Error>
    where D: Deserializer<'de>,
          T: EnumWithStringValue {
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(T::from_string(s))
}

fn deserialize_damage_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, SkillDamageFlags>(deserializer)
}

fn deserialize_copy_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, SkillCopyType>(deserializer)
}

fn deserialize_skill_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, SkillFlags>(deserializer)
}

fn deserialize_weapon_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, WeaponType>(deserializer)
}

fn deserialize_ammo_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, AmmoType>(deserializer)
}

fn deserialize_target_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, UnitTargetType>(deserializer)
}

fn deserialize_skill_cast_time_delay_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, SkillCastTimeDelayType>(deserializer)
}

fn deserialize_flags<'de, D, MaskEnum>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de>,
          MaskEnum: EnumWithMaskValue + EnumWithStringValue,
{
    let s: HashMap<String, bool> = Deserialize::deserialize(deserializer)?;
    let flags: u64 = s.iter().fold(0, |acc, (k, v)| {
        if *v {
            let mask_enum = MaskEnum::from_string_ignore_case(k);
            acc | mask_enum.as_flag() as u64
        } else {
            acc
        }
    });
    Ok(Some(flags))
}

fn deserialize_skill_unit_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillUnitType>(deserializer)
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillRange {
    level: u32,
    #[serde(rename = "size")]
    range: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillHitCount {
    level: u32,
    count: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillElement {
    level: u32,
    #[serde(deserialize_with = "deserialize_optional_enum")]
    element: Option<Element>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillSplashArea {
    level: u32,
    area: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillActiveInstance {
    level: u32,
    #[serde(rename = "max")]
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillKnockback {
    level: u32,
    #[serde(rename = "amount")]
    knockback: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillNoNearNPC {
    level: u32,
    distance: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillCastTime {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillAfterCastActDelay {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillAfterCastWalkDelay {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillDuration1 {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillDuration2 {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillCooldown {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillFixedCastTime {
    level: u32,
    time: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillRequires {
    #[serde(rename = "hpcost", default)]
    hp_cost: Option<u32>,
    #[serde(rename = "hpcostPerLevel", default)]
    hp_cost_per_level: Option<Vec<InternalSkillHpCost>>,
    #[serde(rename = "spcost", default)]
    sp_cost: Option<u32>,
    #[serde(rename = "spcostPerLevel", default)]
    sp_cost_per_level: Option<Vec<InternalSkillSpCost>>,
    #[serde(rename = "hpratecost", default)]
    hp_rate_cost: Option<u32>,
    #[serde(rename = "hpratecostPerLevel", default)]
    hp_rate_cost_per_level: Option<Vec<InternalSkillHpRateCost>>,
    #[serde(rename = "spratecost", default)]
    sp_rate_cost: Option<u32>,
    #[serde(rename = "spratecostPerLevel", default)]
    sp_rate_cost_per_level: Option<Vec<InternalSkillSpRateCost>>,
    #[serde(rename = "zenycost", default)]
    zeny_cost: Option<u32>,
    #[serde(rename = "zenycostPerLevel", default)]
    zeny_cost_per_level: Option<Vec<InternalSkillZenyCost>>,
    #[serde(rename = "weaponFlags", deserialize_with = "deserialize_weapon_flags", default)]
    weapon_flags: Option<u64>,
    #[serde(rename = "ammoFlags", deserialize_with = "deserialize_ammo_flags", default)]
    ammo_flags: Option<u64>,
    ammo_amount: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_enum", default)]
    state: Option<SkillRequirement>,
    #[serde(rename = "spiritSphereCost", default)]
    sphere_cost: Option<u32>,
    #[serde(rename = "spiritSphereCostPerLevel", default)]
    sphere_cost_per_level: Option<Vec<InternalSkillSphereCost>>,
    #[serde(rename = "itemcost", default)]
    item_cost: Vec<InternalSkillItemCost>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillHpCost {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillSpCost {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillHpRateCost {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillSpRateCost {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillMaxHpTrigger {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillZenyCost {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillAmmoAmount {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillState {
    #[serde(deserialize_with = "deserialize_optional_enum")]
    state: Option<SkillRequirement>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillSphereCost {
    level: u32,
    amount: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillItemCost {
    item: String,
    amount: u32,
    level: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillUnit {
    id: String,
    #[serde(rename = "alternateId", default)]
    alternate_id: Option<String>,
    #[serde(default)]
    layout: Option<i32>,
    #[serde(rename = "layoutPerLevel", default)]
    layout_per_level: Option<Vec<InternalSkillUnitLayout>>,
    #[serde(default)]
    range: Option<i32>,
    #[serde(rename = "rangePerLevel", default)]
    range_per_level: Vec<InternalSkillUnitRange>,
    interval: i32,
    #[serde(deserialize_with = "deserialize_optional_enum", default)]
    target: Option<UnitTargetType>,
    #[serde(deserialize_with = "deserialize_skill_unit_flags", default)]
    flag: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillUnitLayout {
    level: u32,
    size: i32,
}

#[derive(Deserialize, Debug, Clone)]
struct InternalSkillUnitRange {
    level: u32,
    size: i32,
}

impl Config {
    pub fn load() -> Result<Config, String> {
        let path = Path::new("config.toml");
        if !path.exists() {
            return Err(format!("config.toml file does not exists at {}", path.to_str().unwrap()));
        }
        let mut config: Config = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        match env::var("DATABASE_PASSWORD") {
            Ok(password) => config.database.set_password(Some(password)),
            Err(_) => return Err("DATABASE_PASSWORD env is missing. please provide this env".to_string())
        }

        if config.server.log_level.is_some() {
            let log_level = config.server.log_level.as_ref().unwrap();
            if !LOG_LEVELS.contains(&log_level.as_str()) {
                println!("Provided log level \"{}\" is not allowed. Valid values are {}, default to \"{}\"", log_level, LOG_LEVELS.join(", "), DEFAULT_LOG_LEVEL);
                config.server.set_log_level(Some(DEFAULT_LOG_LEVEL.to_string()));
            }
        } else {
            config.server.set_log_level(Some(DEFAULT_LOG_LEVEL.to_string()));
        }
        if config.server.log_exclude_pattern.is_none() {
            config.server.set_log_exclude_pattern(Some("none".to_string()));
        }
        Ok(config)
    }

    pub fn load_jobs_config() -> Result<Vec<JobConfig>, String> {
        let path = Path::new("config/job.toml");
        if !path.exists() {
            return Err(format!("config/job.toml file does not exists at {}", path.to_str().unwrap()));
        }
        let internal_configs: InternalJobsConfig = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        let mut job_configs: Vec<JobConfig> = vec![];
        let default_values = internal_configs.jobs.get("default").expect("Expect jobs.default config");
        for (name, config) in internal_configs.jobs.iter() {
            if name == "default" {
                continue;
            }
            let mut base_aspd = Self::resolve_inherited_config(name, config, &internal_configs, "base_aspd", |_conf| None, |conf| conf.base_aspd.clone()).unwrap_or_default();
            default_values.base_aspd.as_ref().expect("Expect jobs.default to have base_aspd").iter().for_each(|(weapon, value)| { base_aspd.entry(weapon.to_string()).or_insert(*value); });
            job_configs.push(JobConfig {
                id: config.id.unwrap_or_else(|| panic!("Expect job {} to have id but found none", name)),
                name: name.clone(),
                base_exp_group: config.base_exp_group.as_ref().unwrap().clone(),
                job_exp_group: config.job_exp_group.as_ref().unwrap().clone(),
                base_weight: Self::resolve_inherited_config(name, config, &internal_configs, "base_weight", |_conf| None, |conf| conf.base_weight)
                    .or_else(|| Some(default_values.base_weight.expect("Expect jobs.default to have base_weight"))).unwrap(),
                base_hp: Self::resolve_inherited_config(name, config, &internal_configs, "inherit_hp", |conf| conf.inherit_hp.as_ref(), |conf| conf.base_hp.clone()).unwrap_or_else(|| panic!("job config for class {}: expected to find property base_hp", name)),
                base_sp: Self::resolve_inherited_config(name, config, &internal_configs, "inherit_sp", |conf| conf.inherit_sp.as_ref(), |conf| conf.base_sp.clone()).unwrap_or_else(|| panic!("job config for class {}: expected to find property base_sp", name)),
                base_aspd,
            });
        }
        Ok(job_configs)
    }

    pub fn load_skills_config() -> Result<Vec<InternalSkillConfig>, String> {
        let path = Path::new("config/skill.toml");
        if !path.exists() {
            return Err(format!("config/skill.toml file does not exists at {}", path.to_str().unwrap()));
        }
        let internal_configs: InternalSkillsConfig = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(internal_configs.skills)
    }

    fn resolve_inherited_config<T, F1, F2>(name: &String, current_config: &InternalJobConfig, configs: &InternalJobsConfig, inherit_name: &str, inherited_property_fn: F1, defined_property_fn: F2) -> Option<T>
        where
            F1: Fn(&InternalJobConfig) -> Option<&String>,
            F2: Fn(&InternalJobConfig) -> Option<T>
    {
        return if let Some(inherit) = current_config.inherit.as_ref() {
            let inherited_config = configs.jobs.get(inherit).unwrap_or_else(|| panic!("job config for class {}: inherit \"{}\" was not found", name, inherit));
            Self::resolve_inherited_config(name, inherited_config, configs, inherit_name, inherited_property_fn, defined_property_fn)
        } else if let Some(inherit) = inherited_property_fn(current_config) {
            let inherited_config = configs.jobs.get(inherit).unwrap_or_else(|| panic!("job config for class {}: {} \"{}\" was not found", name, inherit_name, inherit));
            Self::resolve_inherited_config(name, inherited_config, configs, inherit_name, inherited_property_fn, defined_property_fn)
        } else {
            defined_property_fn(current_config)
        };
    }
}