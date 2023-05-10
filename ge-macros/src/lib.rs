extern crate proc_macro;

use nom::bytes::complete::{take_till, take_while};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::Path;
use syn::*;

struct ConfigTokens {
    file: Ident,
    _sep: Token![.],
    field: Ident,
    _as: Token![as],
    ty: Type,
}

impl syn::parse::Parse for ConfigTokens {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let file = input.parse()?;
        let _sep = input.parse()?;
        let field = input.parse()?;
        let _as = input.parse()?;
        let ty = input.parse()?;

        Ok(ConfigTokens {
            file,
            _sep,
            field,
            _as,
            ty,
        })
    }
}

fn parse(s: &str) -> nom::IResult<&str, &str> {
    let (i, _) = take_till(|c| c == '=')(s)?;
    let (i, o) = take_while(|c: char| c == '=' || c == ' ')(i)?;
    Ok((i, o))
}

#[proc_macro]
pub fn load_config(tokens: TokenStream) -> TokenStream {
    // get the field and file names from token stream
    let tokens = parse_macro_input!(tokens as ConfigTokens);
    let field = tokens.field.to_string();
    let file = tokens.file.to_string();

    // load file and get the value
    let binding = std::fs::read_to_string(Path::new("config").join(format!("{file}.cfg"))).unwrap();
    let line = binding.lines().find(|l| l.starts_with(&field)).unwrap();
    let (value, _) = parse(line).unwrap();
    let value = value.trim();

    // make field name uppercase and generate const
    let fun = Ident::new(&field, Span::call_site());
    let str = Ident::new(&format!("__CONFIG_{}_{}__", file.to_uppercase(), field.to_uppercase()), Span::call_site());
    let ty = tokens.ty;

    dbg!(&file);
    quote! {
        const #str: &'static str = #value;
        pub fn #fun() -> #ty {
            return #str.parse::<#ty>().unwrap();
        }
    }
    .into()
}
