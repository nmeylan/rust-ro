use std::collections::HashMap;
use std::fs;
use std::path::Path;
use configuration::configuration::{Config, SkillsConfig};
use postgres::{Client, NoTls};
use postgres::types::{ToSql, Type};
use serde::Deserialize;
use models::enums::class::JobName;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};

#[derive(Debug, Deserialize)]
struct Character {
    id: i32,
    name: String,
    job: String,
    class: Option<i16>,
    char_num: i16,
    hair: i16,
    hair_color: i16,
    clothes_color: i16,
    max_hp: i32,
    max_sp: i32,
    agi: i16,
    dex: i16,
    str: i16,
    vit: i16,
    int: i16,
    luk: i16,
    base_level: i32,
    job_level: i32,
    equipments: HashMap<String, Equipment>,
    save_y: i16,
    save_x: i16,
    save_map: String,
    last_y: i16,
    last_x: i16,
    last_map: String,
    status_point: i16,
    skill_point: i16,
}
#[derive(Debug, Deserialize)]
struct Equipment {
    name: String,
    #[serde(rename = "itemId")]
    item_id: i32,
}


fn main() {
    let mut replace_existing_char = false;
    let mut account_id = 2000001;
    let mut char_num_start = 0;
    let mut sex = "M".to_string();
    let mut zeny = 11127525_i32;

    let config = Config::load("").unwrap();
    let mut client = Client::connect(
        format!(
            "host={} port={} user={} password={} dbname={}",
            config.database.host,
            config.database.port,
            config.database.username,
            config.database.password.unwrap(),
            config.database.db
        )
            .as_str(),
        NoTls,
    );
    let mut client = client.unwrap();
    let path = Path::new("./tools/account-setup/characters.json");
    if !path.exists() {
        panic!("config/skill.json file does not exists at {}", path.to_str().unwrap());
    }

    let json = fs::read_to_string(path).unwrap();
    let mut config_deserializer = serde_json::Deserializer::from_str(&json);
    let result: Result<Vec<Character>, _> = serde_path_to_error::deserialize(&mut config_deserializer);
    match result {
        Err(err) => {
            let path = err.path().to_string();
            println!("Path in error {}", path);
            panic!("{}", err);
        }
        _ => {}
    }
    let mut characters = result.unwrap();

    let mut query = "INSERT INTO \"char\" (account_id, char_id, class, max_hp, max_sp, agi, dex, str, int, vit, luk, base_level, job_level, save_y, last_y, save_x, last_x, last_map, save_map, hair_color, hair, char_num, name, clothes_color, skill_point, status_point, sex, zeny, hp, sp) VALUES ".to_string();
    const FIELD_COUNT: usize = 30;
    let mut params: Vec<&(dyn postgres::types::ToSql + Sync)> = Vec::with_capacity(FIELD_COUNT * characters.len());
    for character in characters.iter_mut() {
        character.class = Some(JobName::from_string(&character.job).value() as i16);
    }

    for (i, character) in characters.iter().enumerate() {
        if i > 0 {
            query.push_str(", ");
        }

        // Generate placeholders dynamically
        let placeholders: Vec<String> = (0..FIELD_COUNT)
            .map(|j| format!("${}", i * FIELD_COUNT + j + 1))
            .collect();
        query.push_str(&format!("({})", placeholders.join(", ")));

        // Add the parameters
        params.extend_from_slice(&[
            &account_id,
            &character.id,
            &character.class,
            &character.max_hp,
            &character.max_sp,
            &character.agi,
            &character.dex,
            &character.str,
            &character.int,
            &character.vit,
            &character.luk,
            &character.base_level,
            &character.job_level,
            &character.save_y,
            &character.last_y,
            &character.save_x,
            &character.last_x,
            &character.last_map,
            &character.save_map,
            &character.hair_color,
            &character.hair,
            &character.char_num,
            &character.name,
            &character.clothes_color,
            &character.skill_point,
            &character.status_point,
            &sex,
            &zeny,
            &character.max_hp,
            &character.max_sp,
        ]);
    }
    // println!("{}, params: {}", query, params.iter().map(|p| format!("{:?}", p)).collect::<Vec<String>>().join(","));

    client.execute(&query, &params).unwrap();
}
