#![allow(dead_code)]

use r#enum::*;
#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq)]
pub enum  Element {
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