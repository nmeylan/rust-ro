use std::{fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use lazy_static::lazy_static;
use std::path::Path;

lazy_static! {
    pub static ref PACKETS_DB: HashMap<String, String> = {
    let mut packets : HashMap<String, String> = HashMap::new();
    let file = File::open(Path::new("src/server/packets_name")).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        if line.is_ok() {
            let line_fragments: Vec<String> =line.unwrap().split(",").map(|e| e.to_string()).collect();
            packets.insert(line_fragments.get(0).unwrap().to_string(), line_fragments.get(1).unwrap().to_string());
        }
    }
    packets
    };
}