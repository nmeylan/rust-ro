
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use configuration::configuration::Config;
use crate::server::boot::{NpcLoader, NpcLoaderTrait};
use crate::server::model::warp::Warp;

static PARALLEL_EXECUTIONS: usize = 100; // TODO add a conf for this
static WARP_ROOT_PATH: &str = "./config/npc";
static WARP_CONF_FILE: &str = "scripts_warps.conf";


pub struct WarpLoader;
impl NpcLoaderTrait<Warp> for WarpLoader {
    fn parse_npc(file: &File, _config: &'static Config) -> Result<Vec<Warp>, String> {
        let reader = BufReader::new(file);
        let mut warps = Vec::<Warp>::new();
        for line in reader.lines() {
            let mut warp = Warp::new();
            if line.is_err() {
                break;
            }
            let line = line.unwrap();
            if line.starts_with("//") || !line.contains("\twarp\t") {
                continue;
            }
            // A warp "boot" definition is as below
            // anthell01,253,32,0	warp	ant01	2,1,anthell02,34,263
            let line_fragment = line.split('\t').collect::<Vec<&str>>();
            let source_information = line_fragment[0];
            let name = line_fragment[2];
            let warp_and_destination_information = line_fragment[3];
            let source_information_split = source_information.split(',').collect::<Vec<&str>>();
            let warp_and_destination_information_split = warp_and_destination_information.split(',').collect::<Vec<&str>>();
            warp.set_name(name.to_string());
            warp.set_map_name(source_information_split[0].to_string());
            warp.set_x(source_information_split[1].parse::<u16>().unwrap());
            warp.set_y(source_information_split[2].parse::<u16>().unwrap());
            warp.set_x_size(warp_and_destination_information_split[0].parse::<u16>().unwrap());
            warp.set_y_size(warp_and_destination_information_split[1].parse::<u16>().unwrap());
            warp.set_dest_map_name(warp_and_destination_information_split[2].to_string());
            warp.set_to_x(warp_and_destination_information_split[3].parse::<u16>().unwrap());
            warp.set_to_y(warp_and_destination_information_split[4].parse::<u16>().unwrap());
            warps.push(warp);
        }
        Ok(warps)
    }

}

impl WarpLoader {
    pub async fn load_warps(config: &'static Config) -> HashMap<String, Vec<Warp>> {
        let start = Instant::now();
        let npc_loader = NpcLoader {
            conf_file: File::open(Path::new(WARP_ROOT_PATH).join(WARP_CONF_FILE)).unwrap(),
            root_path: WARP_ROOT_PATH.to_string(),
            parallel_execution: PARALLEL_EXECUTIONS,
        };
        let warps = npc_loader.load_npc::<Warp, WarpLoader>(config).await;
        info!("load {} warps in {}ms", warps.iter().fold(0, |memo, curr| memo + curr.1.len()), start.elapsed().as_millis());
        warps
    }
}