use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use tokio::runtime::Runtime;

use crate::repository::model::mob_model::MobModel;
use crate::server::model::configuration::Config;
use crate::server::boot::{Npc, NpcLoader, NpcLoaderTrait};
use crate::server::model::mob_spawn::{MobSpawn, MobType};


static PARALLEL_EXECUTIONS: usize = 1;
// TODO add a conf for this
static MOB_CONF_PATH: &str = "./npc/scripts_monsters.conf";

pub struct MobSpawnLoader;
impl NpcLoaderTrait<MobSpawn> for MobSpawnLoader {
    fn parse_npc(file: &File, config: &'static Config) -> Result<Vec<MobSpawn>, String> {
        let reader = BufReader::new(file);
        let mut mob_spawns = Vec::<MobSpawn>::new();
        let mut i = 0_u32;
        for line in reader.lines() {
            let mut mob_spawn = MobSpawn::default();
            if line.is_err() {
                break;
            }
            let line = line.unwrap();
            if line.trim().starts_with("//") || line.trim().is_empty() || !(line.contains("monster") || line.contains("script")) {
                continue;
            }
            if line.contains("script") {
                debug!("{}. Mob spawn of type \"script\" is currently not supported", line);
                continue;
            }
            // A mob "npc" definition is as below
            //
            // <map name>,<x>,<y>,{<xs>,<ys>}	monster	<mob name>{,<mob level>}	<mob id>,<amount>,{<delay1>,<delay2>,<event>,<mob size>,<mob ai>}
            let line_fragment = line.split('\t').collect::<Vec<&str>>();
            if line_fragment.len() < 4 {
                continue;
            }
            let spawn_location_info = line_fragment[0].split(',').collect::<Vec<&str>>();
            if spawn_location_info.len() < 3 {
                return Err(format!("{}: {:?}: spawn_location_info.len() < 3", line, spawn_location_info));
            }
            mob_spawn.set_map_name(spawn_location_info[0].to_string());
            mob_spawn.set_x(spawn_location_info[1].to_string().parse::<u16>().unwrap());
            mob_spawn.set_y(spawn_location_info[2].to_string().parse::<u16>().unwrap());
            // If mob will spawn in a given range (e.g: seal only spawn on beach)
            if spawn_location_info.len() >= 5 {
                mob_spawn.set_x_size(spawn_location_info[3].to_string().parse::<u16>().unwrap());
                mob_spawn.set_y_size(spawn_location_info[4].to_string().parse::<u16>().unwrap());
            }
            mob_spawn.set_mob_type(MobType::from(line_fragment[1].trim()));
            let mob_info = line_fragment[2].split(',').collect::<Vec<&str>>();
            mob_spawn.set_name(mob_info[0].to_string());
            if mob_info.len() >= 2 {
                mob_spawn.set_level(mob_info[1].parse::<u16>().unwrap());
            }
            let spawn_info = line_fragment[3].split(',').collect::<Vec<&str>>();
            let result = spawn_info[0].parse::<i16>();
            if result.is_err() {
                return Err(format!("{}: {} {}", line, spawn_info[0], result.err().unwrap()));
            }
            mob_spawn.set_mob_id(result.unwrap());
            mob_spawn.set_to_spawn_amount((spawn_info[1].parse::<i16>().unwrap() as f32 * config.game.mob_density) as i16);
            if mob_info.len() >= 3 {
                mob_spawn.set_fixed_delay_in_ms(spawn_info[2].parse::<u32>().unwrap());
            }
            if mob_info.len() >= 4 {
                mob_spawn.set_random_variance_delay_in_ms(spawn_info[3].parse::<u32>().unwrap());
            }
            mob_spawn.set_id(i);
            mob_spawns.push(mob_spawn);
            i += 1;
        }
        Ok(mob_spawns)
    }
}

impl MobSpawnLoader {
    pub fn load_mob_spawns(config: &'static Config, mobs: HashMap<u32, MobModel>) -> JoinHandle<HashMap<String, Vec<MobSpawn>>> {
        thread::spawn(move ||{
            let runtime = Runtime::new().unwrap();
            let start = Instant::now();
            let npc_loader = NpcLoader {
                conf_file: File::open(Path::new(MOB_CONF_PATH)).unwrap(),
                parallel_execution: PARALLEL_EXECUTIONS,
            };
            let mut mob_spawns = runtime.block_on(async { npc_loader.load_npc::<MobSpawn, MobSpawnLoader>(config).await });
            mob_spawns.iter_mut().for_each(|(_, spawns)| {
                spawns.iter_mut().for_each(|mob_spawn| {
                    let mob_info = mobs.get(&(mob_spawn.mob_id as u32)).unwrap_or_else(|| panic!("Can't find mob information for mob id {}", mob_spawn.mob_id));
                    mob_spawn.set_info(mob_info.clone());
                });
            });

            info!("load {} mob spawns in {} secs", mob_spawns.iter().fold(0, |memo, curr| memo + curr.1.len()), start.elapsed().as_millis() as f32 / 1000.0);
            mob_spawns
        })
    }
}