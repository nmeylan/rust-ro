extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::__private::Span;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type, parse_macro_input};

#[proc_macro_derive(Setters, attributes(set))]
pub fn setters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let generated_setters = fields
        .iter()
        .filter(|field| field.attrs.iter().any(|attr| attr.path().is_ident("set")))
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let function_name = Ident::new(format!("set_{field_name}").as_str(), Span::call_site());
            let ty = field.ty.clone();
            quote! {
                pub fn #function_name(&mut self, #field_name: #ty) {
                    self.#field_name = #field_name;
                }
            }
        });

    let res = quote! {
        impl #struct_name {
            #(#generated_setters)*
        }
    };
    TokenStream::from(res)
}
#[proc_macro_derive(SettersAll)]
pub fn setters_all_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let generated_setters = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let function_name = Ident::new(&format!("set_{field_name}"), Span::call_site());
        let ty = field.ty.clone();
        quote! {
            pub fn #function_name(&mut self, #field_name: #ty) {
                self.#field_name = #field_name;
            }
        }
    });

    let res = quote! {
        impl #struct_name {
            #(#generated_setters)*
        }
    };
    TokenStream::from(res)
}
#[proc_macro_derive(GettersAll)]
pub fn getters_all_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let generated_setters = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let function_name = Ident::new(format!("{field_name}").as_str(), Span::call_site());
        let ty = field.ty.clone();
        if is_a_number(&ty) {
            quote! {
                pub fn #function_name(&self) -> #ty {
                    self.#field_name
                }
            }
        } else {
            quote! {
                pub fn #function_name(&self) -> &#ty {
                    &self.#field_name
                }
            }
        }
    });

    let res = quote! {
        impl #struct_name {
            #(#generated_setters)*
        }
    };
    TokenStream::from(res)
}

fn is_a_number(field_type: &Type) -> bool {
    match field_type {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                match segment.ident.to_string().as_str() {
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "f32" | "f64" => {
                        true
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
