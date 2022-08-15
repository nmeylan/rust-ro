extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Variant};
use syn::Data::Enum;

#[proc_macro_derive(WithNumberValue, attributes(value))]
pub fn with_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let res = if let Enum(enum_data) = &input.data {
        let mut j: usize = 1;
        let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(i, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_value(variant);
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
        let value_match_arms = enum_data.variants.iter().enumerate().map(|(i, variant)| {
            let variant_name = variant.ident.clone();
            let maybe_value = get_value(variant);
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

fn get_value(variant: &Variant) -> Option<usize> {
    let maybe_value = variant.attrs.iter().find(|attr| attr.path.is_ident("value")).map(|attr| match attr.parse_meta().unwrap() {
        syn::Meta::NameValue(syn::MetaNameValue {
                                 path,
                                 lit: syn::Lit::Int(s),
                                 ..
                             }) => s.to_string().parse::<usize>().unwrap(),
        _ => panic!("malformed attribute syntax"),
    });
    maybe_value
}