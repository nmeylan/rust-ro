use crate::EnumWithStringValue;
use enum_macro::WithStringValue;

#[derive(WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
#[derive(Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}


