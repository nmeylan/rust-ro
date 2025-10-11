use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Field, Type, Variant};

pub(crate) fn get_number_value<T>(variant: &Variant, ident: &str) -> Option<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    variant.attrs.iter().find(|attr| attr.path().is_ident(ident)).and_then(|attr| {
        if let syn::Meta::NameValue(syn::MetaNameValue {
            eq_token: _,
            value: syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(s), .. }),
            ..
        }) = &attr.meta
        {
            return s.base10_parse::<T>().ok();
        }

        None
    })
}

pub(crate) fn is_all_value(variant: &Variant, ident: &str) -> bool {
    variant.attrs.iter().any(|attr| attr.path().is_ident(ident))
}

pub(crate) fn get_enum_string_value(variant: &Variant, ident: &str, force_lowercase: bool) -> Vec<TokenStream2> {
    let variant_name = variant.ident.clone();
    let string_value;
    let aliases = get_string_value(variant, ident)
        .iter()
        .filter(|maybe_value| maybe_value.is_some())
        .map(|maybe_value| {
            let mut val = maybe_value.clone().unwrap();
            if force_lowercase {
                val = val.to_lowercase()
            }
            quote! { #val }
        })
        .collect::<Vec<TokenStream2>>();
    if !aliases.is_empty() {
        return aliases;
    } else {
        if force_lowercase {
            let string_value = format!("{variant_name}").to_lowercase();
            return vec![quote! { #string_value }];
        }
        string_value = format!("{variant_name}");
    }
    vec![quote! { #string_value }]
}

pub(crate) fn get_string_value(variant: &Variant, ident: &str) -> Vec<Option<String>> {
    variant
        .attrs
        .iter()
        .filter_map(|attr| {
            if let syn::Meta::NameValue(syn::MetaNameValue {
                path,
                eq_token: _,
                value: syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }),
            }) = &attr.meta
            {
                if path.is_ident(ident) {
                    return Some(Some(s.value()));
                }
            }
            None
        })
        .collect()
}

pub(crate) fn field_type(field: &Field) -> Option<String> {
    match &field.ty {
        Type::Path(p) => Some(p.path.get_ident().unwrap().to_string()),
        _ => None,
    }
}

pub(crate) fn is_numeric(field: &Field) -> bool {
    match &field.ty {
        Type::Path(_) => {
            let is_numeric = match field_type(field).unwrap().as_str() {
                "u8" | "i8" | "u16" | "i16" | "i32" | "u32" | "i64" | "u64" | "i128" | "u128" => true,
                _ => false,
            };
            is_numeric
        }
        _ => false,
    }
}
