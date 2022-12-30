use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::DebugFlag;
use rathena_script_lang_interpreter::lang::error::CompilationError;
use rathena_script_lang_interpreter::lang::value::Value;
use rathena_script_lang_interpreter::util::scripts_compiler;
use crate::server::core::map_item::{MapItem, MapItemType};
use crate::server::map_item::ToMapItem;


use crate::server::script::constant::load_constant;

// TODO add a conf for this
static SCRIPT_CONF_PATH: &str = "./npc/scripts_custom.conf";

#[derive(Setters, Clone, Debug)]
pub struct Script {
    #[set]
    id: u32,
    map_name: String,
    name: String,
    sprite: u16,
    x: u16,
    y: u16,
    dir: u16,
    #[allow(dead_code)]
    x_size: u16,
    #[allow(dead_code)]
    y_size: u16,
    pub class_name: String,
    pub class_reference: u64,
    pub constructor_args: Vec<Value>,
    #[set]
    pub instance_reference: u64,
}

impl Script {
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn dir(&self) -> u16 {
        self.dir
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn load_scripts() -> (HashMap::<String, Vec<Script>>, Vec<ClassFile>, HashMap::<String, Vec<CompilationError>>) {
        let mut npcs_by_map = HashMap::<String, Vec<Script>>::new();
        let conf_file = File::open(Path::new(SCRIPT_CONF_PATH)).unwrap();
        let reader = BufReader::new(&conf_file);
        let mut paths = Vec::<String>::new();
        for line in reader.lines() {
            if line.is_err() {
                break;
            }
            let mut line = line.unwrap();
            if !line.starts_with("npc:") {
                continue;
            }
            line = line.replace("npc: ", "");
            let npc_script_path = line.trim().to_string();
            paths.push(npc_script_path);
        }
        let compilation_result = scripts_compiler::compile(paths, "native_functions_list.txt", DebugFlag::None.value());
        let (scripts, class_files, errors) = compilation_result;
        let scripts = scripts.iter().map(|s| {
            let sprite = if let Ok(sprite_id) = s.sprite.parse::<u16>() {
                sprite_id
            } else {
                let mut sprite_id = 100;
                if let Some(constant) = load_constant(&s.sprite) {
                    if constant.is_number() {
                        sprite_id = constant.number_value().unwrap() as u16;
                    }
                }
                sprite_id
            };
            Script {
                id: 0,
                map_name: s.map.clone(),
                name: s.name.clone(),
                sprite,
                x: s.x_pos as u16,
                y: s.y_pos as u16,
                dir: s.dir as u16,
                x_size: s.x_size as u16,
                y_size: s.y_size as u16,
                class_name: s.class_name.clone(),
                class_reference: s.class_reference,
                constructor_args: s.constructor_args.clone(),
                instance_reference: 0,
            }
        }).collect::<Vec<Self>>();
        for script in scripts {
            let map_name = script.map_name.clone();
            if npcs_by_map.contains_key(&map_name) {
                npcs_by_map.get_mut(&map_name).unwrap().push(script);
            } else {
                npcs_by_map.insert(map_name, vec![script]);
            }
        }
        (npcs_by_map, class_files, errors)
    }
}

impl ToMapItem for Script {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.sprite as i16, MapItemType::Npc)
    }
}