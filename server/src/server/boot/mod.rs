use futures::future::join_all;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use configuration::configuration::Config;
use crate::server::model::Npc;

pub mod mob_spawn_loader;
pub mod script_loader;
pub mod warps_loader;
pub mod map_loader;

pub struct NpcLoader {
    pub(crate) conf_file: File,
    pub(crate) root_path: String,
    pub(crate) parallel_execution: usize,
}

pub trait NpcLoaderTrait<NpcKind: Npc> {
    fn parse_npc(file: &File, config: &'static Config) -> Result<Vec<NpcKind>, String>;
}

impl NpcLoader {
    pub async fn load_npc<NpcKind: Npc + Clone + Send + 'static, T: NpcLoaderTrait<NpcKind>>(&self, config: &'static Config) -> HashMap<String, Vec<NpcKind>> {
        let semaphore = Semaphore::new(self.parallel_execution);
        let reader = BufReader::new(&self.conf_file);
        let npcs_by_map = Arc::new(Mutex::new(HashMap::<String, Vec<NpcKind>>::new()));
        let mut futures: Vec<JoinHandle<()>> = Vec::new();
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

            let _ = semaphore.acquire().await.unwrap();
            let res = npcs_by_map.clone();
            let config_root_path = self.root_path.clone();
            futures.push(tokio::task::spawn_blocking(move || {
                let npc_script_file_res = File::open(Path::new(config_root_path.as_str()).join(npc_script_path.clone()));
                if npc_script_file_res.is_err() {
                    warn!(
                        "Not able to load boot script: {}, due to {}",
                        npc_script_path,
                        npc_script_file_res.err().unwrap()
                    );
                    return;
                }

                let npcs_result = T::parse_npc(&npc_script_file_res.unwrap(), config);
                if npcs_result.is_err() {
                    error!("{}", npcs_result.err().unwrap());
                    return;
                }
                let npcs = npcs_result.unwrap();
                for npc in npcs {
                    let mut res_guard = res.lock().unwrap();
                    let map_name = npc.get_map_name();
                    let entry = res_guard.entry(map_name).or_insert(Default::default());
                    entry.push(npc);
                }
            }));
        }
        join_all(futures).await;
        let guard = npcs_by_map.lock().unwrap();
        let mut res = HashMap::<String, Vec<NpcKind>>::new();
        guard.iter().for_each(|(k, v)| {
            res.insert(k.clone(), v.clone());
        });
        res
    }
}
