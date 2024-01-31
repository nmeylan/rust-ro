use crate::{EnumWithStringValue, EnumWithNumberValue};
use enum_macro::{WithNumberValue, WithStringValue};

#[derive(WithStringValue, WithNumberValue, Debug, Copy, Clone, PartialEq, Eq)]
#[derive(Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
    All,
}


