use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rathena_script_lang_interpreter::lang::chunk::ClassFile;
use rathena_script_lang_interpreter::lang::compiler::DebugFlag;
use rathena_script_lang_interpreter::lang::error::CompilationError;
use rathena_script_lang_interpreter::lang::value::Value;
use rathena_script_lang_interpreter::util::scripts_compiler;
use crate::server::model::map_item::{MapItem, MapItemType, ToMapItem};
use crate::server::model::script::Script;


use crate::server::script::constant::load_constant;

// TODO add a conf for this
static SCRIPT_CONF_PATH: &str = "./npc/scripts_custom.conf";

pub struct ScriptLoader;
impl ScriptLoader {

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
        }).collect::<Vec<Script>>();
        for script in scripts {
            let map_name = script.map_name.clone();
            let entry = npcs_by_map.entry(map_name).or_insert(Default::default());
            entry.push(script);
        }
        (npcs_by_map, class_files, errors)
    }
}

impl ToMapItem for Script {
    fn to_map_item(&self) -> MapItem {
        MapItem::new(self.id, self.sprite as i16, MapItemType::Npc)
    }
}