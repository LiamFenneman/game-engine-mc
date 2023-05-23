use proc_macro::TokenStream;
use quote::quote;
use syn::*;

pub fn dbg_timer(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as proc_macro2::TokenStream);
    quote! {
        let dbg_time_start = std::time::Instant::now();
        #input
        tracing::debug!("elapsed: {:?}", dbg_time_start.elapsed());
    }
    .into()
}

pub fn dbg_time(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as proc_macro2::TokenStream);
    quote! {
        let dbg_time_start = std::time::Instant::now();
        #input
        tracing::debug!("elapsed: {:?}", dbg_time_start.elapsed());
    }
    .into()
}
