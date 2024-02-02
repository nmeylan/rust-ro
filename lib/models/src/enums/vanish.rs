#![allow(dead_code)]
use crate::enums::*;

#[derive(WithNumberValue, Debug, Copy, Clone)]
pub enum VanishType {
    #[value = 0]
    OutOfSight,
    Die,
    Loggout,
    Teleport,
    TrickDead,
}
