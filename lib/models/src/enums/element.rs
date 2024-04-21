#![allow(dead_code)]

use std::fmt::{Display, Formatter};
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
    AllElement,
    Max,
    Weapon,
    Endowed,
    Random,
    Ammo,
}

impl Default for Element {
    fn default() -> Self {
        Element::Neutral
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}