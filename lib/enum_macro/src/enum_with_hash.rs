use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Enum;
use syn::{DeriveInput, parse_macro_input};
use proc_macro2::{Ident, Span};

pub fn with_hash(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    if let Enum(enum_data) = &input.data {
        let arms = enum_data.variants.iter().enumerate().map(|(j, variant)| {
            let variant_name = variant.ident.clone();
            let base_hash = format!("{}_{}", enum_name, j);
            match &variant.fields {
                syn::Fields::Unnamed(fields) => {
                    let args1 = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        Ident::new(format!("arg{}", i).as_str(), Span::call_site())
                    }).collect::<Vec<Ident>>();

                    quote! {#enum_name::#variant_name(#(#args1)*) => {#base_hash.hash(state); #(#args1.hash(state);)*} }
                }
                syn::Fields::Unit => {
                    quote! { #enum_name::#variant_name => {#base_hash.hash(state);}}
                }
                syn::Fields::Named(_) => {
                    // Handling named fields if necessary
                    panic!("Named fields are not supported in this macro")
                }
            }
        });
        TokenStream::from(quote! {
            impl std::hash::Hash for #enum_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                    match self {
                        #(#arms)*
                    }
                }
            }
        })
    } else {
        TokenStream::from(quote! {})
    }
}
