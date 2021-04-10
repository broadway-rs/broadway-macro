use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, AttributeArgs};

#[proc_macro_attribute]
pub fn role(attr: TokenStream, input: TokenStream) -> TokenStream{
    let og = input.clone();
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(input as ItemTrait);
    og
}