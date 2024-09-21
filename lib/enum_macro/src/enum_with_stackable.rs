use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use proc_macro2::{TokenStream as TokenStream2};
use syn::Data::Enum;
use syn::{DeriveInput, parse_macro_input};
use crate::helper::{field_type, get_number_value, is_numeric};

pub fn stackable_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    if let Enum(enum_data) = &input.data {
        let get_value_sum_arms = enum_data.variants.iter().filter(|variant| matches!(&variant.fields, syn::Fields::Unnamed(..))).map(|variant| {
            let variant_name = variant.ident.clone();
            if let syn::Fields::Unnamed(fields) = &variant.fields {
                let value_offset = if variant.attrs.iter().find(|attr| {
                    attr.path().is_ident("value_offset")
                }).is_some() {
                    get_number_value(variant, "value_offset").unwrap_or(1)
                } else if fields.unnamed.len() == 1 {
                    0
                } else {
                    1
                };
                if is_numeric(&fields.unnamed[value_offset]) {
                    let mut args1 = fields.unnamed.iter().map(|_| quote! {_}).collect::<Vec<TokenStream2>>();
                    args1[value_offset] = quote! {val};
                    quote! {#enum_name::#variant_name(#(#args1,)*) => Some(*val as f32), }
                } else {
                    quote! {}
                }
            } else {
                panic!("patterns `Fields::Named(_)` and `Fields::Unit` not covered")
            }
        });
        let get_value_sum_return_arms = enum_data.variants.iter().map(|variant| {
            let variant_name = variant.ident.clone();
            if let syn::Fields::Unnamed(fields) = &variant.fields {
                let value_offset = if variant.attrs.iter().find(|attr| {
                    attr.path().is_ident("value_offset")
                }).is_some() {
                    get_number_value(variant, "value_offset").unwrap_or(1)
                } else if fields.unnamed.len() == 1 {
                    0
                } else {
                    1
                };
                if is_numeric(&fields.unnamed[value_offset]) {
                    let mut args1 = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let v = Ident::new(format!("value{}", i).as_str(), Span::call_site());
                        quote! {#v}
                    }).collect::<Vec<TokenStream2>>();
                    let mut args2 = args1.clone();
                    let val_type = Ident::new(field_type(&fields.unnamed[value_offset]).unwrap().as_str(), Span::call_site());
                    args1[value_offset] = quote! {_};
                    args2[value_offset] = quote! {&val as #val_type};
                    quote! {#enum_name::#variant_name(#(#args1,)*) => #enum_name::#variant_name(#(*#args2,)*), }
                } else {
                    let args1 = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let v = Ident::new(format!("value{}", i).as_str(), Span::call_site());
                        quote! {#v}
                    }).collect::<Vec<TokenStream2>>();
                    quote! {#enum_name::#variant_name(#(#args1,)*) => #enum_name::#variant_name(#(*#args1,)*), }
                }

            } else {
                quote! {#enum_name::#variant_name => #enum_name::#variant_name, }
            }
        });
        let get_enum_value = enum_data.variants.iter().filter(|variant| matches!(&variant.fields, syn::Fields::Unnamed(..))).map(|variant| {
            let variant_name = variant.ident.clone();
            if let syn::Fields::Unnamed(fields) = &variant.fields {
                let value_offset = if variant.attrs.iter().find(|attr| {
                    attr.path().is_ident("value_offset")
                }).is_some() {
                    get_number_value(variant, "value_offset").unwrap_or(1)
                } else if fields.unnamed.len() == 1 {
                    0
                } else {
                    1
                };
                if is_numeric(&fields.unnamed[value_offset]) {
                    let mut args1 = fields.unnamed.iter().map(|_| {
                        quote! {_}
                    }).collect::<Vec<TokenStream2>>();
                    args1[value_offset] = quote! {val};
                    quote! {#enum_name::#variant_name(#(#args1,)*) => *val as f32, }
                } else {
                    quote!{}
                }

            } else {
                panic!("patterns `Fields::Named(_)` and `Fields::Unit` not covered")
            }
        });
        TokenStream::from(quote! {
            impl EnumStackable<#enum_name> for #enum_name {
                fn get_value_sum(single_enum: &#enum_name, enums: &Vec<#enum_name>) -> #enum_name {
                    let val: f32 = enums.into_iter().filter_map(|e|
                        if e == single_enum {
                            match e {
                                #(#get_value_sum_arms)*
                                _ => None
                            }
                        } else {
                            None
                        }
                    ).sum();
                    match single_enum {
                        #(#get_value_sum_return_arms)*
                    }
                }
                fn get_enum_value<'a>(single_enum: &#enum_name, enums: &'a Vec<&#enum_name>) -> Option<f32> {
                    Self::get_enum(single_enum, enums).map(|b| match b {
                         #(#get_enum_value)*
                        _ => 0.0
                    })
                }
            }
        })
    } else {
        TokenStream::from(quote! {})
    }
}