use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::time::Instant;
use rathena_script_lang_interpreter::lang::script_visitor;
use crate::MapItem;
use crate::server::enums::map_item::MapItemType;
use crate::server::npc::npc::{Npc, NpcLoader};


static PARALLEL_EXECUTIONS: usize = 1;
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
    x_size: u16,
    y_size: u16,
}

impl MapItem for Script {
    fn id(&self) -> u32 {
        self.id
    }

    fn client_item_class(&self) -> i16 {
        self.sprite as i16
    }
    fn object_type(&self) -> i16 {
        MapItemType::NPC.value()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
        self.y
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Npc for Script {
    fn parse_npc(file: &File) -> Result<Vec<Self>, String> where Self: Sized {
        let scripts = script_visitor::visit(file);
        Ok(scripts.iter().map(|s| Script {
            id: 0,
            map_name: s.map.clone(),
            name: s.name.clone(),
            sprite: s.sprite as u16,
            x: s.x_pos as u16,
            y: s.y_pos as u16,
            dir: s.dir as u16,
            x_size: s.x_size as u16,
            y_size: s.y_size as u16
        }).collect::<Vec<Self>>())
    }

    fn get_map_name(&self) -> String {
        self.map_name.clone()
    }
}

impl Script {
    pub async fn load_script() -> HashMap<String, Vec<Script>> {
        let start = Instant::now();
        let npc_loader = NpcLoader {
            conf_file: File::open(Path::new(SCRIPT_CONF_PATH)).unwrap(),
            parallel_execution: PARALLEL_EXECUTIONS,
        };
        let scripts = npc_loader.load_npc::<Script>().await;
        info!("load {} scripts in {} secs", scripts.iter().fold(0, |memo, curr| memo + curr.1.len()), start.elapsed().as_millis() as f32 / 1000.0);
        scripts
    }
}