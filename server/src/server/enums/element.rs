#![allow(dead_code)]

#[derive(r#enum::WithNumberValue, Debug, Copy, Clone, PartialEq)]
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