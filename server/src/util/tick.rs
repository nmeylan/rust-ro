#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_tick_client() -> u32 {
    get_tick() as u32
}
pub fn get_tick() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn get_current_time() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

pub fn delayed_tick(delay: u128, tick_rate: u128) -> usize {
    let index = (delay as f64 / tick_rate as f64).round() as usize;
    index
}