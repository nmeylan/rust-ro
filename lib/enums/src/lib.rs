pub use enum_macro::*;
pub mod action;
pub mod class;
pub mod client_messages;
pub mod element;
pub mod item;
pub mod skill;
pub mod status;
pub mod unit;
pub mod weapon;
pub mod skills;
pub mod look;
pub mod vanish;
pub mod map;

pub trait EnumWithStringValue {
    fn try_from_string(value: &str) -> Result<Self, String> where Self: Sized;
    fn from_string(value: &str) -> Self;
    fn from_string_ignore_case(value: &str) -> Self;
    fn as_str(&self) -> &str;
}

pub trait EnumWithMaskValueU64 {
    fn from_flag(value: u64) -> Self;
    fn as_flag(&self) -> u64;
}

pub trait EnumWithNumberValue {
    fn from_value(value: usize) -> Self;
    fn try_from_value(value: usize) -> Result<Self, String> where Self: Sized;
    fn value(&self) -> usize;
}