use std::{env, fs};
use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Deserializer};
use accessor::Setters;
use enums::{EnumWithMaskValueU64, EnumWithStringValue};
use enums::element::Element;
use enums::skill::{SkillCastTimeDelayType, SkillCopyType, SkillDamageFlags, SkillDamageType, SkillFlags, SkillRequirement, SkillTargetType, SkillType, SkillUnitType};
use enums::unit::UnitTargetType;
use enums::weapon::{AmmoType, WeaponType};
use crate::util::serde_helper::*;

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

#[derive(Deserialize, Debug, SettersAll, Clone)]
pub struct GameConfig {
    pub default_char_speed: u16,
    pub max_inventory: u16,
    pub max_base_level: u32,
    pub max_job_level: u32,
    pub mob_density: f32,
    pub drop_rate: f32,
    pub drop_rate_mvp: f32,
    pub drop_rate_card: f32,
    pub mob_dropped_item_locked_to_owner_duration_in_secs: u16,
    pub player_dropped_item_locked_to_owner_duration_in_secs: u16,
    #[serde(skip)]
    pub status_point_rewards: Vec<StatusPointReward>,
    #[serde(skip)]
    pub status_point_raising_cost: Vec<StatusPointRaisingCost>,
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
struct SkillsConfig {
    #[serde(rename = "skills", deserialize_with = "deserialize_skills")]
    skills: HashMap<u32, SkillConfig>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct SkillConfig {
    pub(crate) id: u32,
    pub(crate) name: String,
    description: String,
    #[serde(rename = "maxLevel")]
    max_level: u32,
    #[serde(rename = "type", deserialize_with = "deserialize_optional_string_enum", default)]
    skill_type: Option<SkillType>,
    #[serde(rename = "targetType", deserialize_with = "deserialize_string_enum", default = "SkillTargetType::default")]
    target_type: SkillTargetType,
    #[serde(rename = "damageflags", deserialize_with = "deserialize_damage_flags", default)]
    damage_flags: Option<u64>,
    #[serde(rename = "flags", deserialize_with = "deserialize_skill_flags", default)]
    flags: Option<u64>,
    #[serde(default)]
    range: Option<i32>,
    #[serde(rename = "rangePerLevel", deserialize_with =  "deserialize_tuples", default)]
    range_per_level: Option<Vec<(u32, i32)>>,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    hit: Option<SkillDamageType>,
    #[serde(rename = "hitCount", default)]
    hit_count: Option<i32>,
    #[serde(rename = "hitCountPerLevel", deserialize_with =  "deserialize_tuples", default)]
    hit_count_per_level: Option<Vec<(u32, i32)>>,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    element: Option<Element>,
    element_per_level: Option<Vec<InternalSkillElement>>,
    #[serde(rename = "splashArea", default)]
    splash_area: Option<i32>,
    #[serde(rename = "splashAreaPerLevel", deserialize_with =  "deserialize_tuples", default)]
    splash_area_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "activeInstance", default)]
    active_instance: Option<u32>,
    #[serde(rename = "activeInstancePerLevel", deserialize_with =  "deserialize_tuples", default)]
    active_instance_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "knockback", default)]
    knockback: Option<u32>,
    #[serde(rename = "knockbackPerLevel", deserialize_with =  "deserialize_tuples", default)]
    knockback_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "copyflags", deserialize_with = "deserialize_copy_flags", default)]
    copy_flags: Option<u64>,
    #[serde(rename = "castCancel", default)]
    cast_cancel: bool,
    #[serde(rename = "castDefenseReduction", default)]
    cast_defense_reduction: u32,
    #[serde(rename = "castTime", default)]
    cast_time: Option<u32>,
    #[serde(rename = "castTimePerLevel", deserialize_with =  "deserialize_tuples", default)]
    cast_time_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "afterCastActDelay", default)]
    after_cast_act_delay: Option<u32>,
    #[serde(rename = "afterCastActDelayPerLevel", deserialize_with =  "deserialize_tuples", default)]
    after_cast_act_delay_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "afterCastActDelay", default)]
    after_cast_walk_delay: Option<u32>,
    #[serde(rename = "afterCastActDelayPerLevel", deserialize_with =  "deserialize_tuples", default)]
    after_cast_walk_delay_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "duration1", default)]
    duration1: Option<u32>,
    #[serde(rename = "duration1PerLevel", deserialize_with =  "deserialize_tuples", default)]
    duration1_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "duration2", default)]
    duration2: Option<u32>,
    #[serde(rename = "duration2PerLevel", deserialize_with =  "deserialize_tuples", default)]
    duration2_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "cooldown", default)]
    cooldown: Option<u32>,
    #[serde(rename = "cooldownPerLevel", deserialize_with =  "deserialize_tuples", default)]
    cooldown_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "fixedCastTime", default)]
    fixed_cast_time: Option<u32>,
    #[serde(rename = "fixedCastTimePerLevel", deserialize_with =  "deserialize_tuples", default)]
    fixed_cast_time_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "casttimeflags", deserialize_with = "deserialize_skill_cast_time_delay_flags", default)]
    cast_time_flags: Option<u64>,
    #[serde(rename = "castdelayflags", deserialize_with = "deserialize_skill_cast_time_delay_flags", default)]
    cast_delay_flags: Option<u64>,
    requires: Option<SkillRequirements>,
    unit: Option<SkillUnit>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct SkillRequirements {
    #[serde(rename = "hpcost", default)]
    hp_cost: Option<u32>,
    #[serde(rename = "hpcostPerLevel", deserialize_with = "deserialize_tuples", default)]
    hp_cost_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "spcost", default)]
    sp_cost: Option<u32>,
    #[serde(rename = "spcostPerLevel", deserialize_with = "deserialize_tuples", default)]
    sp_cost_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "hpratecost", default)]
    hp_rate_cost: Option<u32>,
    #[serde(rename = "hpratecostPerLevel", deserialize_with = "deserialize_tuples", default)]
    hp_rate_cost_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "spratecost", default)]
    sp_rate_cost: Option<u32>,
    #[serde(rename = "spratecostPerLevel", deserialize_with = "deserialize_tuples", default)]
    sp_rate_cost_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "zenycost", default)]
    zeny_cost: Option<u32>,
    #[serde(rename = "zenycostPerLevel", deserialize_with = "deserialize_tuples", default)]
    zeny_cost_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "weaponFlags", deserialize_with = "deserialize_weapon_flags", default)]
    weapon_flags: Option<u64>,
    #[serde(rename = "ammoFlags", deserialize_with = "deserialize_ammo_flags", default)]
    ammo_flags: Option<u64>,
    #[serde(rename = "ammoFlags", default)]
    ammo_amount: Option<u32>,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    state: Option<SkillRequirement>,
    #[serde(rename = "spiritSphereCost", default)]
    sphere_cost: Option<u32>,
    #[serde(rename = "spiritSphereCostPerLevel", deserialize_with = "deserialize_tuples", default)]
    sphere_cost_per_level: Option<Vec<(u32, i32)>>,
    #[serde(rename = "itemcost", default)]
    item_cost: Vec<InternalSkillItemCost>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct SkillUnit {
    id: String,
    #[serde(rename = "alternateId", default)]
    alternate_id: Option<String>,
    #[serde(default)]
    layout: Option<i32>,
    #[serde(rename = "layoutPerLevel", deserialize_with = "deserialize_tuples", default)]
    layout_per_level: Option<Vec<(u32, i32)>>,
    #[serde(default)]
    range: Option<i32>,
    #[serde(rename = "rangePerLevel", deserialize_with = "deserialize_tuples", default)]
    range_per_level: Option<Vec<(u32, i32)>>,
    interval: i32,
    #[serde(deserialize_with = "deserialize_optional_string_enum", default)]
    target: Option<UnitTargetType>,
    #[serde(deserialize_with = "deserialize_skill_unit_flags", default)]
    flag: Option<u64>,
}

fn deserialize_tuples<'de, D>(deserializer: D) -> Result<Option<Vec<(u32, i32)>>, D::Error>
    where D: Deserializer<'de> {
    let s: Vec<HashMap<String, i32>> = Deserialize::deserialize(deserializer)?;
    let res = s.iter().map(|x| {
        let (_, value) = x.iter().find(|(k, _)| k.as_str() != "level").unwrap();
        (*x.get("level").unwrap() as u32, *value as i32)
    }).collect::<Vec<(u32, i32)>>();
    Ok(Some(res))
}
fn deserialize_skills<'de, D>(deserializer: D) -> Result<HashMap<u32, SkillConfig>, D::Error>
    where D: Deserializer<'de> {
    let skills: Vec<SkillConfig> = Deserialize::deserialize(deserializer)?;
    let mut skills_map: HashMap<u32, SkillConfig> = Default::default();
    skills.iter().for_each(|skill| {
        skills_map.insert(skill.id, skill.clone());
    });

    Ok(skills_map)
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

fn deserialize_skill_cast_time_delay_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de> {
    deserialize_flags::<_, SkillCastTimeDelayType>(deserializer)
}

fn deserialize_skill_unit_flags<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error> where D: Deserializer<'de>,
{
    deserialize_flags::<_, SkillUnitType>(deserializer)
}

fn deserialize_flags<'de, D, MaskEnum>(deserializer: D) -> Result<Option<u64>, D::Error>
    where D: Deserializer<'de>,
          MaskEnum: EnumWithMaskValueU64 + EnumWithStringValue,
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

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct InternalSkillElement {
    level: u32,
    #[serde(deserialize_with = "deserialize_optional_string_enum")]
    element: Option<Element>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
struct InternalSkillItemCost {
    item: String,
    amount: u32,
    level: Option<u32>,
}

impl Config {
    pub fn load() -> Result<Config, String> {
        let path = Path::new("config.json");
        if !path.exists() {
            return Err(format!("config.json file does not exists at {}", env::current_dir().unwrap().join(path).to_str().unwrap()));
        }
        let mut config: Config = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
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
        let file_path = "./config/status_point_reward.json";
        Self::set_config_status_point_rewards(&mut config, file_path).unwrap();
        let file_path = "./config/status_point_raising_cost.json";
        Self::set_config_status_point_raising_cost(&mut config, file_path).unwrap();


        Ok(config)
    }

    pub fn set_config_status_point_rewards(config: &mut Config, file_path: &str) -> Result<(), String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!("{} file does not exists at {}", file_path, env::current_dir().unwrap().join(path).to_str().unwrap()));
        }
        config.game.status_point_rewards = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(())
    }
    pub fn set_config_status_point_raising_cost(config: &mut Config, file_path: &str) -> Result<(), String> {
        let path = Path::new(file_path);
        if !path.exists() {
            return Err(format!("{} file does not exists at {}", file_path, env::current_dir().unwrap().join(path).to_str().unwrap()));
        }
        config.game.status_point_raising_cost = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        Ok(())
    }

    pub fn load_jobs_config(root: &str) -> Result<Vec<JobConfig>, String> {
        let path = Path::new(root).join("config/job.json");
        if !path.exists() {
            return Err(format!("config/job.json file does not exists at {}", env::current_dir().unwrap().join(path).to_str().unwrap()));
        }
        let internal_configs: InternalJobsConfig = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
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

    pub fn load_skills_config(root: &str) -> Result<HashMap<u32, SkillConfig>, String> {
        let path = Path::new(root).join("config/skill.json");
        if !path.exists() {
            return Err(format!("config/skill.json file does not exists at {}", env::current_dir().unwrap().join(path).to_str().unwrap()));
        }
        let internal_configs: SkillsConfig = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
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

    pub fn packetver(&self) -> u32 {
        self.server.packetver
    }

}