use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use futures::future::join_all;

pub struct NpcLoader {
    pub(crate) conf_file: File,
    pub(crate) parallel_execution: usize,
}

pub trait Npc {
    fn parse_npc(file: &File) -> Result<Vec<Self>, String> where Self: Sized;
    fn get_map_name(&self) -> String;
}

impl NpcLoader {
    pub async fn load_npc<T: Npc + Clone + Send + 'static>(&self) -> HashMap<String, Vec<T>> {
        let semaphore = Semaphore::new(self.parallel_execution);
        let reader = BufReader::new(&self.conf_file);
        let npcs_by_map = Arc::new(Mutex::new(HashMap::<String, Vec<T>>::new()));
        let mut futures : Vec<JoinHandle<()>> = Vec::new();
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
            futures.push(tokio::task::spawn_blocking(move || {
                let npc_script_file_res = File::open(Path::new(&npc_script_path));
                if npc_script_file_res.is_err() {
                    warn!("Not able to load npc script: {}, due to {}", npc_script_path, npc_script_file_res.err().unwrap());
                    return;
                }
                
                let npcs_result = T::parse_npc(&npc_script_file_res.unwrap());
                if npcs_result.is_err() {
                    error!("{}", npcs_result.err().unwrap());
                    return ;
                }
                let npcs = npcs_result.unwrap();
                for npc in npcs {
                    let mut res_guard = res.lock().unwrap();
                    let map_name = npc.get_map_name();
                    if res_guard.contains_key(&map_name) {
                        res_guard.get_mut(&map_name).unwrap().push(npc);
                    } else {
                        res_guard.insert(map_name, vec![npc]);
                    }
                }
            }));
        }
        join_all(futures).await;
        let guard = npcs_by_map.lock().unwrap();
        let mut res= HashMap::<String, Vec<T>>::new();
        guard.iter().for_each(|(k, v)| {
            res.insert(k.clone(), v.clone());
        });
        res
    }
}