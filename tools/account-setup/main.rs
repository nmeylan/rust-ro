use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use configuration::configuration::{Config, SkillsConfig};
use postgres::{Client, NoTls};
use postgres::types::{ToSql, Type};
use serde::Deserialize;
use serde_json::Value;
use models::enums::class::JobName;
use models::enums::{EnumWithNumberValue, EnumWithStringValue};
use models::enums::item::EquipmentLocation;

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
    item_id: i16,
    #[serde(skip_deserializing)]
    item_id_32: i32,
    location: Option<i32>,
    unique_id: Option<i64>,
}


fn main() {
    let mut replace_existing_char = true;
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
        panic!("tools/account-setup/characters.json file does not exists at {}", path.to_str().unwrap());
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


    let path = Path::new("./config/items.json");
    if !path.exists() {
        panic!("config/items.json file does not exists at {}", path.to_str().unwrap());
    }
    let json = fs::read_to_string(path).unwrap();
    let mut config_deserializer = serde_json::Deserializer::from_str(&json);
    let result: Result<Value, _> = serde_path_to_error::deserialize(&mut config_deserializer);
    let value = result.unwrap();
    let items = value.as_object().unwrap().get("items").unwrap().as_array().unwrap();

    let mut query_insert_char = "INSERT INTO \"char\" (account_id, char_id, class, max_hp, max_sp, agi, dex, str, int, vit, luk, base_level, job_level, save_y, last_y, save_x, last_x, last_map, save_map, hair_color, hair, char_num, name, clothes_color, skill_point, status_point, sex, zeny, hp, sp, body, weapon, shield, head_top, head_mid, head_bottom) VALUES ".to_string();
    let mut query_insert_inventory = "INSERT INTO \"inventory\" (char_id, nameid, amount, equip, identified, unique_id) VALUES ".to_string();
    let mut query_delete_char = "DELETE FROM \"char\" WHERE char_id = ANY($1);".to_string();
    let mut query_delete_inventory = "DELETE FROM \"inventory\" WHERE char_id = ANY($1);".to_string();
    const CHAR_FIELD_COUNT: usize = 36;
    const INVENTORY_FIELD_COUNT: usize = 6;

    let mut inventory_count = 0;
    for character in characters.iter_mut() {
        character.class = Some(JobName::from_string(&character.job).value() as i16);
        for (location, item) in character.equipments.iter_mut() {
            let it = items.iter().find(|it| it.get("id").unwrap().as_u64().unwrap() == item.item_id as u64).unwrap().as_object().unwrap();
            let location = it.get("location").unwrap().as_u64().unwrap();
            item.location = Some(location as i32);
            item.item_id_32 = item.item_id as i32;
            item.unique_id = Some((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() << 9) as i64 + item.item_id as i64);
        }
        inventory_count += character.equipments.len();
    }

    let mut char_params: Vec<&(dyn postgres::types::ToSql + Sync)> = Vec::with_capacity(CHAR_FIELD_COUNT * characters.len());
    let mut inventory_params: Vec<&(dyn postgres::types::ToSql + Sync)> = Vec::with_capacity(INVENTORY_FIELD_COUNT * inventory_count);
    let mut j = 0;
    for (i, character) in characters.iter().enumerate() {
        if i > 0 {
            query_insert_char.push_str(", ");
        }

        for (_, (location, equipment)) in character.equipments.iter().enumerate() {
            if j > 0 {
                query_insert_inventory.push_str(", ");
            }
            query_insert_inventory.push_str(&format!("({})", generate_placeholder(j, INVENTORY_FIELD_COUNT).join(", ")));

            inventory_params.extend_from_slice(&[
                &character.id,
                &equipment.item_id_32,
                &1_i16,
                equipment.location.as_ref().unwrap(),
                &true,
                equipment.unique_id.as_ref().unwrap(),
            ]);
            j += 1;
        }

        query_insert_char.push_str(&format!("({})", generate_placeholder(i, CHAR_FIELD_COUNT).join(", ")));

        // Add the parameters
        char_params.extend_from_slice(&[
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
            character.equipments.get("body").map_or(&0_i16, |e| &e.item_id),
            character.equipments.get("weapon").map_or(&0_i16, |e| &e.item_id),
            character.equipments.get("shield").map_or(&0_i16, |e| &e.item_id),
            character.equipments.get("head_top").map_or(&0_i16, |e| &e.item_id),
            character.equipments.get("head_mid").map_or(&0_i16, |e| &e.item_id),
            character.equipments.get("head_low").map_or(&0_i16, |e| &e.item_id),
        ]);
    }
    // println!("{}, params: {}", query, params.iter().map(|p| format!("{:?}", p)).collect::<Vec<String>>().join(","));
    let mut transaction = client.transaction().unwrap();
    if replace_existing_char {
        let vec = characters.iter()
            .map(|c| c.id)
            .collect::<Vec<i32>>();
        transaction.execute(query_delete_char.as_str(), &[&vec]).unwrap();
        transaction.execute(query_delete_inventory.as_str(), &[&vec]).unwrap();
    }
    transaction.execute(&query_insert_char, &char_params).unwrap();
    // println!("{}, params: {}", query_insert_inventory, inventory_params.iter().map(|p| format!("{:?}", p)).collect::<Vec<String>>().join(","));
    transaction.execute(&query_insert_inventory, &inventory_params).unwrap();
    transaction.commit().unwrap();
}

fn generate_placeholder(i: usize, field_count: usize) -> Vec<String> {
    (0..field_count)
        .map(|j| format!("${}", i * field_count + j + 1))
        .collect()
}
