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
            let maybe_value = get_number_value(variant);
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
            let maybe_value = get_number_value(variant);
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
                        _ => panic!("Can't create enum #enum_name for value {}", value)
                    }
                }

                pub fn value(&self) -> usize {
                    match self {
                        #(#value_match_arms)*
                        _ => panic!("Value can't be found for enum #enum_name")
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    TokenStream::from(res)
}

fn get_number_value(variant: &Variant) -> Option<usize> {
    let maybe_value = variant.attrs.iter().find(|attr| attr.path.is_ident("value")).map(|attr| match attr.parse_meta().unwrap() {
        syn::Meta::NameValue(syn::MetaNameValue {
                                 lit: syn::Lit::Int(s),
                                 ..
                             }) => s.to_string().parse::<usize>().unwrap(),
        _ => panic!("malformed attribute syntax"),
    });
    maybe_value
}


#[proc_macro_derive(WithStringValue, attributes(with_string_value_uppercase, value))]
pub fn with_string_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let mut uppercase = false;
    input.attrs.iter().for_each(|attr| {
        if attr.path.is_ident("with_string_value_uppercase") {
            uppercase = true;
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
            } else {
                format!("{}", variant_name)
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
            } else {
                format!("{}", variant_name)
            };
            let res = quote! {
                #enum_name::#variant_name => #string_value.to_string(),
            };
            res
        });
        quote! {
            impl #enum_name {
                pub fn from_string(value: &str) -> Self {
                    match value {
                        #(#from_value_match_arms)*
                        _ => panic!("Can't create enum #enum_name for value {}", value)
                    }
                }

                pub fn to_string(&self) -> String {
                    match self {
                        #(#value_match_arms)*
                        _ => panic!("Value can't be found for enum #enum_name")
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
    let maybe_value = variant.attrs.iter().find(|attr| attr.path.is_ident("str")).map(|attr| match attr.parse_meta().unwrap() {
        syn::Meta::NameValue(syn::MetaNameValue {
                                 lit: syn::Lit::Int(s),
                                 ..
                             }) => s.to_string(),
        _ => panic!("malformed attribute syntax"),
    });
    maybe_value
}