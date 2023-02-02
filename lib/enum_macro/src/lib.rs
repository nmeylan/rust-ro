extern crate proc_macro;
use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Variant};
use syn::Data::Enum;

#[proc_macro_derive(WithNumberValue, attributes(value))]
pub fn with_number_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let res = if let Enum(enum_data) = &input.data {
        let mut j: usize = 1;
        let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_number_value::<usize>(variant, "value");
            if let Some(value) = maybe_value {
                j = value;
            }
            let res = quote! {
                #j => #enum_name::#variant_name,
            };
            j += 1;
            res
        });
        let mut j: usize = 1;
        let try_from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_number_value::<usize>(variant, "value");
            if let Some(value) = maybe_value {
                j = value;
            }
            let res = quote! {
                #j => Ok(#enum_name::#variant_name),
            };
            j += 1;
            res
        });
        let mut j: usize = 1;
        let value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_number_value::<usize>(variant, "value");
            if let Some(value) = maybe_value {
                j = value;
            }
            let res = quote! {
                #enum_name::#variant_name => #j,
            };
            j += 1;
            res
        });
        quote! {
            impl EnumWithNumberValue for #enum_name {
                fn from_value(value: usize) -> Self {
                    match value {
                        #(#from_value_match_arms)*
                        _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                    }
                }
                fn try_from_value(value: usize) -> Result<Self, String> {
                    match value {
                        #(#try_from_value_match_arms)*
                        _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                    }
                }

                fn value(&self) -> usize {
                    match self {
                        #(#value_match_arms)*
                        _ => panic!("Value can't be found for enum_macro #enum_name")
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    TokenStream::from(res)
}

macro_rules! with_mask {
    ($function:ident, $trait:ident, $macro:ident, $type:ty, $max_bits:expr) => {

        #[proc_macro_derive($macro, attributes(mask_value, mask_all))]
        pub fn $function(input: TokenStream) -> TokenStream {
            let input = parse_macro_input!(input as DeriveInput);
            let enum_name = &input.ident;
            fn count_number_of_1_bits(value: $type) -> usize {
                let mut k: usize = 0;
                // count number of 1 bits in value
                loop {
                    if k > $max_bits {
                        break;
                    }
                    if value & (1 << k) != 0 {
                        return k;
                    }
                    k += 1;
                }
                0
            }
            let res = if let Enum(enum_data) = &input.data {
                let mut i: usize = 0;
                let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
                    let variant_name = variant.ident.clone();
                    let maybe_value = get_number_value::<$type>(variant, "mask_value");
                    let is_all_value = is_all_value(variant, "mask_all");
                    let j = if let Some(value) = maybe_value {
                        i = count_number_of_1_bits(value);
                        value
                    } else if is_all_value {
                        <$type>::MAX
                    } else {
                        1 << i
                    };

                    let res = quote! {
                        #j => #enum_name::#variant_name,
                    };
                    i += 1;
                    res
                });
                let mut i: usize = 0;
                let value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
                    let variant_name = variant.ident.clone();
                    let maybe_value = get_number_value::<$type>(variant, "mask_value");
                    let is_all_value = is_all_value(variant, "mask_all");
                    let j = if let Some(value) = maybe_value {
                        i = count_number_of_1_bits(value);
                        value
                    } else if is_all_value {
                        <$type>::MAX
                    } else {
                        1 << i
                    };
                    let res = quote! {
                        #enum_name::#variant_name => #j,
                    };
                    i += 1;
                    res
                });
                quote! {
                    impl $trait for #enum_name {
                        fn from_flag(value: $type) -> Self {
                            match value {
                                #(#from_value_match_arms)*
                                _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                            }
                        }

                        fn as_flag(&self) -> $type {
                            match self {
                                #(#value_match_arms)*
                                _ => panic!("Value can't be found for enum_macro #enum_name")
                            }
                        }
                    }
                }
            } else {
                quote! {}
            };
            TokenStream::from(res)
        }
    }
}

with_mask!(with_mask_value_u64, EnumWithMaskValueU64, WithMaskValueU64, u64, 63);
with_mask!(with_mask_value_u32, EnumWithMaskValueU32, WithMaskValueU32, u32, 31);
with_mask!(with_mask_value_u16, EnumWithMaskValueU16, WithMaskValueU16, u16, 15);
with_mask!(with_mask_value_u8, EnumWithMaskValueU8, WithMaskValueU8, u8, 7);




fn get_number_value<T>(variant: &Variant, ident: &str) -> Option<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    let maybe_value = variant.attrs.iter().find(|attr| attr.path.is_ident(ident)).map(|attr| match attr.parse_meta().unwrap() {
        syn::Meta::NameValue(syn::MetaNameValue {
                                 lit: syn::Lit::Int(s),
                                 ..
                             }) => s.to_string().parse::<T>().unwrap(),
        _ => panic!("malformed attribute syntax"),
    });
    maybe_value
}

fn is_all_value(variant: &Variant, ident: &str) -> bool {
    variant.attrs.iter().any(|attr| attr.path.is_ident(ident))
}


#[proc_macro_derive(WithStringValue, attributes(with_string_value_uppercase, with_string_value_lowercase, value_string))]
pub fn with_string_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let mut uppercase = false;
    let mut lowercase = false;
    input.attrs.iter().for_each(|attr| {
        if attr.path.is_ident("with_string_value_uppercase") {
            uppercase = true;
        }
        if attr.path.is_ident("with_string_value_lowercase") {
            lowercase = true;
        }
    });
    let res = if let Enum(enum_data) = &input.data {
        let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_string_value(variant);
            let string_value = if let Some(value) = maybe_value {
                value
            } else if uppercase {
                format!("{}", variant_name).to_uppercase()
            } else if lowercase {
                format!("{}", variant_name).to_lowercase()
            } else {
                format!("{}", variant_name)
            };
            let res = quote! {
                #string_value => #enum_name::#variant_name,
            };
            res
        });
        let try_from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_string_value(variant);
            let string_value = if let Some(value) = maybe_value {
                value
            } else if uppercase {
                format!("{}", variant_name).to_uppercase()
            } else if lowercase {
                format!("{}", variant_name).to_lowercase()
            } else {
                format!("{}", variant_name)
            };
            let res = quote! {
                #string_value => Ok(#enum_name::#variant_name),
            };
            res
        });
        let from_value_ignore_case_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_string_value(variant);
            let string_value = if let Some(value) = maybe_value {
                value.to_lowercase()
            } else {
                format!("{}", variant_name).to_lowercase()
            };
            let res = quote! {
                #string_value => #enum_name::#variant_name,
            };
            res
        });
        let value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_string_value(variant);
            let string_value = if let Some(value) = maybe_value {
                value
            } else if uppercase {
                format!("{}", variant_name).to_uppercase()
            } else if lowercase {
                format!("{}", variant_name).to_lowercase()
            } else {
                format!("{}", variant_name)
            };
            let res = quote! {
                #enum_name::#variant_name => #string_value,
            };
            res
        });
        quote! {
            impl EnumWithStringValue for #enum_name {
                fn from_string(value: &str) -> Self {
                    match value {
                        #(#from_value_match_arms)*
                        _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                    }
                }
                fn try_from_string(value: &str) -> Result<Self, String> {
                    match value {
                        #(#try_from_value_match_arms)*
                        _ => Err(format!("Can't create enum_macro #enum_name for value {}", value))
                    }
                }
                fn from_string_ignore_case(value: &str) -> Self {
                    match value.to_string().to_lowercase().as_str() {
                        #(#from_value_ignore_case_match_arms)*
                        _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                    }
                }

                fn as_str(&self) -> &str {
                    match self {
                        #(#value_match_arms)*
                        _ => panic!("Value can't be found for enum_macro #enum_name")
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    TokenStream::from(res)
}

fn get_string_value(variant: &Variant) -> Option<String> {
    let maybe_value = variant.attrs.iter().find(|attr| attr.path.is_ident("value_string")).map(|attr| match attr.parse_meta().unwrap() {
        syn::Meta::NameValue(syn::MetaNameValue {
                                 lit: syn::Lit::Str(s),
                                 ..
                             }) => s.value(),
        _ => panic!("malformed attribute syntax"),
    });
    maybe_value
}