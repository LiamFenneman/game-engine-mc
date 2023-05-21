extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[proc_macro_attribute]
pub fn dbg_timer(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as proc_macro2::TokenStream);
    quote! {
        let dbg_time_start = std::time::Instant::now();
        #input
        tracing::debug!("elapsed: {:?}", dbg_time_start.elapsed());
    }
    .into()
}

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

struct FromTrnsInput {
    left: Ident,
    _for: Token![for],
    right: Ident,
}

impl syn::parse::Parse for FromTrnsInput {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let left = input.parse()?;
        let _for = input.parse()?;
        let right = input.parse()?;
        Ok(Self { left, _for, right })
    }
}

#[proc_macro]
pub fn impl_from_trns(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FromTrnsInput);
    let (left, right) = (input.left, input.right);
    quote! {
        impl From<#left> for Transformation {
            fn from(t: #left) -> Self {
                return Self::#right(t);
            }
        }
    }
    .into()
}
