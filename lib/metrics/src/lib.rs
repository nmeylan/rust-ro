use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[proc_macro_attribute]
pub fn elapsed(_args: TokenStream, function_def: TokenStream) -> TokenStream {
    let mut item = syn::parse(function_def).unwrap();
    let fn_item = match &mut item {
        syn::Item::Fn(fn_item) => fn_item,
        _ => panic!("expected fn"),
    };
    let ItemFn { attrs, vis, sig, block } = fn_item;
    let function_body = block.clone();
    let fn_name = sig.ident.clone();
    let log_ns = format!("{fn_name} tooks {{}}ns");
    let log_us = format!("{fn_name} tooks {{}}µs");
    let log_ms = format!("{fn_name} tooks {{}}ms");
    let new_function_def = quote! {
        #(#attrs)* #vis #sig {
            let start_for_elapsed_macro = std::time::Instant::now();
            let mut wrapped_func = || #function_body;
            let res = wrapped_func();
            let elapsed = start_for_elapsed_macro.elapsed().as_nanos();
            if elapsed < 1000 {
                info!(#log_ns, elapsed);
            } else if elapsed < 1000 * 1000 {
                info!(#log_us, elapsed as f64 / 1000.0);
            } else {
                info!(#log_ms, elapsed as f64 / 1000.0 / 1000.0);
            }
            res
        }
    };
    TokenStream::from(new_function_def)
}
