pub use enum_macro::*;

pub trait EnumWithStringValue {
    fn from_string(value: &str) -> Self;
    fn from_string_ignore_case(value: &str) -> Self;
    fn as_str(&self) -> &str;
}

pub trait EnumWithMaskValue {
    fn from_flag(value: u64) -> Self;
    fn as_flag(&self) -> u64;
}