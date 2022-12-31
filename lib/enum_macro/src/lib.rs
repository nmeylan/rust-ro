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
            impl #enum_name {
                pub fn from_value(value: usize) -> Self {
                    match value {
                        #(#from_value_match_arms)*
                        _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                    }
                }

                pub fn value(&self) -> usize {
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

#[proc_macro_derive(WithMaskValue, attributes(mask_value))]
pub fn with_mask_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let res = if let Enum(enum_data) = &input.data {
        let mut i: usize = 0;
        let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_number_value::<u64>(variant, "mask_value");
            let j = if let Some(value) = maybe_value {
                i = count_number_of_1_bits(value);
                value
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
            let maybe_value = get_number_value::<u64>(variant, "mask_value");
            let j = if let Some(value) = maybe_value {
                i = count_number_of_1_bits(value);
                value
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
            impl EnumWithMaskValue for #enum_name {
                fn from_flag(value: u64) -> Self {
                    match value {
                        #(#from_value_match_arms)*
                        _ => panic!("Can't create enum_macro #enum_name for value {}", value)
                    }
                }

                fn as_flag(&self) -> u64 {
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

fn count_number_of_1_bits(value: u64) -> usize {
    let mut k: usize = 0;
    // count number of 1 bits in value
    loop {
        if k > 63 {
            break;
        }
        if value & (1 << k) != 0 {
            return k;
        }
        k += 1;
    }
    0
}

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