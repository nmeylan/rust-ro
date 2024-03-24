mod enum_with_number_value;
mod helper;
mod enum_with_string_value;
mod enum_with_eq;
mod enum_with_stackable;

extern crate proc_macro;

#[macro_use]
mod enum_with_mask;

use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Enum;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(WithNumberValue, attributes(value))]
pub fn with_number_value(input: TokenStream) -> TokenStream {
    crate::enum_with_number_value::with_number_value(input)
}


with_mask!(
    with_mask_value_u64,
    EnumWithMaskValueU64,
    WithMaskValueU64,
    u64,
    63
);
with_mask!(
    with_mask_value_u32,
    EnumWithMaskValueU32,
    WithMaskValueU32,
    u32,
    31
);
with_mask!(
    with_mask_value_u16,
    EnumWithMaskValueU16,
    WithMaskValueU16,
    u16,
    15
);
with_mask!(
    with_mask_value_u8,
    EnumWithMaskValueU8,
    WithMaskValueU8,
    u8,
    7
);

#[proc_macro_derive(
WithStringValue,
attributes(value_string)
)]
pub fn with_string_value(input: TokenStream) -> TokenStream {
    crate::enum_with_string_value::with_string_value(input)
}

#[proc_macro_derive(WithEq, attributes(value_comparison_offset))]
pub fn with_eq(input: TokenStream) -> TokenStream {
    crate::enum_with_eq::with_eq(input)
}


#[proc_macro_derive(WithStackable, attributes(skip_sum, value_offset))]
pub fn stackable_enum(input: TokenStream) -> TokenStream {
    crate::enum_with_stackable::stackable_enum(input)
}