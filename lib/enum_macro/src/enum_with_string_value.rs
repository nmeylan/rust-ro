use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Enum;
use syn::{DeriveInput, parse_macro_input};
use crate::helper::{get_enum_string_value};

pub fn with_string_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let res = if let Enum(enum_data) = &input.data {
        let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let string_value = get_enum_string_value(variant, "value_string", false);

            let res = if string_value.len() > 1 {
                quote! {
                    #(#string_value )|* => #enum_name::#variant_name,
                }
            } else {
                quote! {
                    #(#string_value)* => #enum_name::#variant_name,
                }
            };
            res
        });
        let try_from_value_match_arms =
            enum_data.variants.iter().enumerate().map(|(_, variant)| {
                let variant_name = variant.ident.clone();
                let string_value = get_enum_string_value(variant, "value_string", false);

                let res = if string_value.len() > 1 {
                    quote! {
                        #(#string_value )|* => Ok(#enum_name::#variant_name),
                    }
                } else {
                    quote! {
                        #(#string_value)* => Ok(#enum_name::#variant_name),
                    }
                };
                res
            });
        let from_value_ignore_case_match_arms =
            enum_data.variants.iter().enumerate().map(|(_, variant)| {
                let variant_name = variant.ident.clone();
                let string_value = get_enum_string_value(variant, "value_string", true);

                let res = if string_value.len() > 1 {
                    quote! {
                        #(#string_value )|* => #enum_name::#variant_name,
                    }
                } else {
                    quote! {
                        #(#string_value)* => #enum_name::#variant_name,
                    }
                };

                res
            });
        let value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
            let variant_name = variant.ident.clone();
            let string_value = get_enum_string_value(variant, "value_string", false);
            let res = if string_value.len() > 1 {
                let string_value = string_value.iter().map(|v| v.to_string().replace("\"", "")).collect::<Vec<String>>().join("|");
                quote! {
                       #enum_name::#variant_name => #string_value,
                    }
            } else {
                quote! {
                       #enum_name::#variant_name => #(#string_value)*,
                    }
            };

            res
        });
        quote! {
            impl EnumWithStringValue for #enum_name {
                fn from_string(value: &str) -> Self {
                    match value {
                        #(#from_value_match_arms)*
                        _ => panic!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value)
                    }
                }
                fn try_from_string(value: &str) -> Result<Self, String> {
                    match value {
                        #(#try_from_value_match_arms)*
                        _ => Err(format!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value))
                    }
                }
                fn from_string_ignore_case(value: &str) -> Self {
                    match value.to_string().to_lowercase().as_str() {
                        #(#from_value_ignore_case_match_arms)*
                        _ => panic!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value)
                    }
                }

                fn as_str(&self) -> &str {
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