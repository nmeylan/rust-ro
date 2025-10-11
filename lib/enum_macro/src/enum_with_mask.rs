macro_rules! with_mask {
    ($function:ident, $trait:ident, $macro:ident, $type:ty, $max_bits:expr) => {
        #[proc_macro_derive($macro, attributes(mask_value, mask_all))]
        pub fn $function(input: TokenStream) -> TokenStream {
            let input = parse_macro_input!(input as DeriveInput);
            let enum_name = &input.ident;
            fn count_number_of_1_bits(value: $type) -> usize {
                let mut k: usize = 0;
                // count number of 1 bits in value
                loop {
                    if k > $max_bits {
                        break;
                    }
                    if value & (1 << k) != 0 {
                        return k;
                    }
                    k += 1;
                }
                0
            }
            let res = if let Enum(enum_data) = &input.data {
                let mut i: usize = 0;
                let from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
                    let variant_name = variant.ident.clone();
                    let maybe_value = crate::helper::get_number_value::<$type>(variant, "mask_value");
                    let is_all_value = crate::helper::is_all_value(variant, "mask_all");
                    let j = if let Some(value) = maybe_value {
                        i = count_number_of_1_bits(value);
                        value
                    } else if is_all_value {
                        <$type>::MAX
                    } else {
                        1 << i
                    };

                    let res = quote! {
                        #j => #enum_name::#variant_name,
                    };
                    i += 1;
                    res
                });
                let mut i: usize = 0;
                let try_from_value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
                    let variant_name = variant.ident.clone();
                    let maybe_value = crate::helper::get_number_value::<$type>(variant, "mask_value");
                    let is_all_value = crate::helper::is_all_value(variant, "mask_all");
                    let j = if let Some(value) = maybe_value {
                        i = count_number_of_1_bits(value);
                        value
                    } else if is_all_value {
                        <$type>::MAX
                    } else {
                        1 << i
                    };

                    let res = quote! {
                        #j => Ok(#enum_name::#variant_name),
                    };
                    i += 1;
                    res
                });
                let mut i: usize = 0;
                let value_match_arms = enum_data.variants.iter().enumerate().map(|(_, variant)| {
                    let variant_name = variant.ident.clone();
                    let maybe_value = crate::helper::get_number_value::<$type>(variant, "mask_value");
                    let is_all_value = crate::helper::is_all_value(variant, "mask_all");
                    let j = if let Some(value) = maybe_value {
                        i = count_number_of_1_bits(value);
                        value
                    } else if is_all_value {
                        <$type>::MAX
                    } else {
                        1 << i
                    };
                    let res = quote! {
                        #enum_name::#variant_name => #j,
                    };
                    i += 1;
                    res
                });
                quote! {
                    impl $trait for #enum_name {
                        fn from_flag(value: $type) -> Self {
                            match value {
                                #(#from_value_match_arms)*
                                _ => panic!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value)
                            }
                        }
                        fn try_from_flag(value: $type) -> Result<Self, String> {
                            match value {
                                #(#try_from_value_match_arms)*
                                _ => Err(format!("Can't create enum_macro {} for value [{}]", stringify!(#enum_name), value))
                            }
                        }

                        fn as_flag(&self) -> $type {
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
    };
}
