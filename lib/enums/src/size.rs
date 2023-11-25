use crate::EnumWithStringValue;
use enum_macro::WithStringValue;

#[derive(WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Default for Size {
    fn default() -> Self {
        Size::Medium
    }
}
