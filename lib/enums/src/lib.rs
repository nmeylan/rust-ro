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

pub trait EnumWithStringValue {
    fn from_string(value: &str) -> Self;
    fn from_string_ignore_case(value: &str) -> Self;
    fn as_str(&self) -> &str;
}

pub trait EnumWithMaskValue {
    fn from_flag(value: u64) -> Self;
    fn as_flag(&self) -> u64;
}