use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_tick() -> u32 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs() as u32
}