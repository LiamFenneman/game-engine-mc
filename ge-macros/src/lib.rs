use proc_macro::TokenStream;

extern crate proc_macro;

mod timer;
mod trns;

#[proc_macro_attribute]
pub fn dbg_timer(attr: TokenStream, item: TokenStream) -> TokenStream {
    timer::dbg_timer(attr, item)
}

#[proc_macro]
pub fn dbg_time(input: TokenStream) -> TokenStream {
    timer::dbg_time(input)
}

#[proc_macro]
pub fn impl_from_trns(input: TokenStream) -> TokenStream {
    trns::impl_from_trns(input)
}
