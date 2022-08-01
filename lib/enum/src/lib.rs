extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, DataEnum};
use syn::Data::Enum;

#[proc_macro_derive(WithNumberValue)]
pub fn with_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let res = if let Enum(enum_data) = &input.data {
        let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(i, variant)| {
            let j = i + 1;
            quote! {
                #j => #enum_name::#variant,
            }
        });
        let value_match_arms = enum_data.variants.iter().enumerate().map(|(i, variant)| {
            let j = i + 1;
            quote! {
                #enum_name::#variant => #j,
            }
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