use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Data::Enum;
use syn::{DeriveInput, parse_macro_input};

use crate::helper::{get_number_value, is_numeric};

pub fn with_eq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    if let Enum(enum_data) = &input.data {
        let arms = enum_data.variants.iter().map(|variant| {
            let variant_name = variant.ident.clone();
            match &variant.fields {
                syn::Fields::Unnamed(fields) => {
                    let variant_offset = if variant.attrs.iter().find(|attr| {
                        attr.path().is_ident("value_comparison_offset")
                    }).is_some() {
                        get_number_value(variant, "value_comparison_offset").unwrap_or(0)
                    } else {
                        0
                    };
                    let mut args1 = fields.unnamed.iter().map(|_| quote! {_}).collect::<Vec<TokenStream2>>();
                    let mut args2 = fields.unnamed.iter().map(|_| quote! {_}).collect::<Vec<TokenStream2>>();
                    let mut should_deref = false;
                    if args1.len() > 1 {
                        args1[variant_offset] = quote! {variant1};
                        args2[variant_offset] = quote! {variant2};
                        should_deref = is_numeric(&fields.unnamed[variant_offset]);
                    }
                    // let field_types = fields.unnamed.iter().map(|field| &field.ty);
                    if fields.unnamed.len() > 1 {
                        if should_deref {
                            quote! {(#enum_name::#variant_name(#(#args1,)*), #enum_name::#variant_name(#(#args2,)*)) => *variant1 == *variant2, }
                        } else {
                            quote! {(#enum_name::#variant_name(#(#args1,)*), #enum_name::#variant_name(#(#args2,)*)) => variant1 == variant2, }
                        }
                    } else {
                        quote! {(#enum_name::#variant_name(#(#args1)*), #enum_name::#variant_name(#(#args2)*)) => true, }
                    }
                }
                syn::Fields::Unit => {
                    quote! {
                    (#enum_name::#variant_name, #enum_name::#variant_name) => true,
                }
                }
                syn::Fields::Named(_) => {
                    // Handling named fields if necessary
                    panic!("Named fields are not supported in this macro")
                }
            }
        });
        TokenStream::from(quote! {
            impl PartialEq for #enum_name {
            fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                     #(#arms)*
                        _ => false
                    }
                }
            }
        })
    } else {
        TokenStream::from(quote! {})
    }
}
