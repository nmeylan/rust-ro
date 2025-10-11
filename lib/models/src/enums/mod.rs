pub use enum_macro::*;

pub mod action;
pub mod bonus;
pub mod cell;
pub mod class;
pub mod client_effect_icon;
pub mod client_messages;
pub mod effect;
pub mod element;
pub mod item;
pub mod look;
pub mod map;
pub mod mob;
pub mod size;
pub mod skill;
pub mod skill_enums;
pub mod status;
pub mod trigger;
pub mod unit;
pub mod vanish;
pub mod weapon;

pub trait EnumWithStringValue {
    fn try_from_string(value: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn try_from_string_ignore_case(value: &str) -> Result<Self, String>
    where
        Self: Sized;
    fn from_string(value: &str) -> Self;
    fn from_string_ignore_case(value: &str) -> Self;
    fn as_str(&self) -> &str;
}

macro_rules! enum_with_mask_trait {
    ($trait_name:ident, $type:ty) => {
        pub trait $trait_name {
            fn from_flag(value: $type) -> Self;
            fn try_from_flag(value: $type) -> Result<Self, String>
            where
                Self: Sized;
            fn as_flag(&self) -> $type;
        }
    };
}
enum_with_mask_trait!(EnumWithMaskValueU64, u64);
enum_with_mask_trait!(EnumWithMaskValueU32, u32);
enum_with_mask_trait!(EnumWithMaskValueU16, u16);
enum_with_mask_trait!(EnumWithMaskValueU8, u8);

pub trait EnumWithNumberValue {
    fn from_value(value: usize) -> Self;
    fn try_from_value(value: usize) -> Result<Self, String>
    where
        Self: Sized;
    fn value(&self) -> usize;
}

pub trait EnumStackable<T: PartialEq> {
    fn get_value_sum(single_enum: &T, enums: &Vec<T>) -> T;
    fn merge_enums(enums: &Vec<T>) -> Vec<T> {
        let mut merged_enums: Vec<T> = Vec::with_capacity(enums.len());
        for single_enum in enums.iter() {
            if merged_enums.contains(single_enum) {
                continue;
            }
            merged_enums.push(Self::get_value_sum(single_enum, enums));
        }
        merged_enums
    }
    #[inline]
    fn get_enum<'a>(single_enum: &T, enums: &'a Vec<&T>) -> Option<&'a T> {
        enums.iter().find(|b| **b == single_enum).copied()
    }
    fn get_enum_value(single_enum: &T, enums: &Vec<&T>) -> Option<f32>;
}
