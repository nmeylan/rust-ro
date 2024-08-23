use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Enum;
use syn::{DeriveInput, parse_macro_input};
use crate::helper::get_number_value;

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
        let try_from_value_match_arms =
            enum_data.variants.iter().enumerate().map(|(_, variant)| {
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
                        _ => panic!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value)
                    }
                }
                fn try_from_value(value: usize) -> Result<Self, String> {
                    match value {
                        #(#try_from_value_match_arms)*
                        _ => panic!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value)
                    }
                }

                fn value(&self) -> usize {
                    match self {
                        #(#value_match_arms)*
                        _ => panic!("Value can't be found for enum_macro {}", stringify!(#enum_name))
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    TokenStream::from(res)
}