use enum_macro::{WithNumberValue, WithStringValue};

use crate::enums::{EnumWithNumberValue, EnumWithStringValue};

#[derive(WithStringValue, WithNumberValue, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Size {
    #[value = 0]
    Small,
    #[default]
    Medium,
    Large,
    All,
}
