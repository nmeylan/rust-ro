use enum_macro::WithStringValue;
use crate::EnumWithStringValue;

#[derive(WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Size {
    Small,
    Medium,
    Large
}

impl Default for Size {
    fn default() -> Self {
        Size::Medium
    }
}