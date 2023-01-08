#![allow(dead_code)]
use crate::*;

#[derive(WithNumberValue, Debug, Copy, Clone)]
pub enum LookType {
    #[value = 0]
    Job,
    Hair,
    Weapon,
    HeadBottom,
    HeadTop,
    HeadMid,
    HairColor,
    ClothesColor,
    Shield,
    Shoes,
    Robe,
    Body,
}