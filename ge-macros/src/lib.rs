extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, format_ident};
use syn::*;

struct AttrTokens {
    file: Ident,
}

impl syn::parse::Parse for AttrTokens {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let file = input.parse()?;
        Ok(AttrTokens { file })
    }
}

#[proc_macro_attribute]
pub fn config(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttrTokens);
    let item = parse_macro_input!(item as DeriveInput);

    let const_name = format_ident!("{}_CONFIG", attr.file.to_string().to_uppercase());
    let file_name = format!("{}.toml", attr.file);
    let ty = format_ident!("{}", item.ident);

    quote! {
        #item

        #[allow(clippy::declare_interior_mutable_const)]
        pub const #const_name: std::cell::LazyCell<#ty> = std::cell::LazyCell::new(|| {
            let file = std::path::Path::new("config").join(#file_name);
            let binding = std::fs::read_to_string(file).unwrap();
            return toml::from_str(&binding).unwrap();
        });
    }
    .into()
}
