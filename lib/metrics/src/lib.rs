use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, ItemFn};
use syn::parse::Parse;

#[proc_macro_attribute]
pub fn elapsed(_args: TokenStream, function_def: TokenStream) -> TokenStream {
    let mut item: syn::Item = syn::parse(function_def.clone()).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn")
    };
    let ItemFn { attrs, vis, sig, block } = fn_item;
    let function_body = block.clone();

    let new_function_def = quote! {
        #(#attrs)* #vis #sig {
            let start_for_elapsed_macro = std::time::Instant::now();
            let mut wrapped_func = || #function_body;
            let res = wrapped_func();
            elapsed!(start_for_elapsed_macro);
            res
        }
    };
    TokenStream::from(new_function_def)
}