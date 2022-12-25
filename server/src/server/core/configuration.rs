use std::{env, fs};
use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use accessor::Setters;

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
        }
    }
}