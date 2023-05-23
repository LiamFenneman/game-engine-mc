use proc_macro::TokenStream;
use quote::quote;
use syn::*;

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
