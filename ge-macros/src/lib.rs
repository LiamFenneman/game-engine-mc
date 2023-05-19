extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro]
pub fn dbg_time(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as proc_macro2::TokenStream);
    quote! {
        let dbg_time_start = std::time::Instant::now();
        #input
        tracing::debug!("elapsed: {:?}", dbg_time_start.elapsed());
    }
    .into()
}
