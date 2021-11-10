use std::{env, fs};
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
}

#[derive(Deserialize, Debug, Setters, Clone)]
pub struct ServerConfig {
    #[set]
    pub log_level: Option<String>,
    pub accounts: Vec<u32>,
    pub port: u16,
    pub enable_visual_debugger: bool
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

impl Config {
    pub fn load() -> Result<Config, String> {
        let path = Path::new("config.toml");
        if !path.exists() {
            return Err(format!("config.toml file does not exists at {}", path.to_str().unwrap()));
        }
        let mut config: Config = toml::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        match env::var("DATABASE_PASSWORD"){
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

        return Ok(config);
    }
}