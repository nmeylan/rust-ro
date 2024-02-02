#![allow(dead_code)]
use crate::enums::*;
#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Element {
    #[value = 0]
    Neutral,
    Water,
    Earth,
    Fire,
    Wind,
    Poison,
    Holy,
    Dark,
    Ghost,
    Undead,
    All,
    Max,
    Weapon,
    Endowed,
    Random,
}
