use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, AttributeArgs};

#[proc_macro_attribute]
pub fn role(attr: TokenStream, input: TokenStream) -> Result<TokenStream, BroadwayMacroError>{
    let og = input.clone();
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut key = None;
    let mut actor = None;
    for arg in arg.iter(){
        if let syn::NestedMeta::Meta(meta) = arg{
            if let sync::Meta::NameValue(name_value) = meta{
                
            }
        }
    }
    let input = parse_macro_input!(input as ItemTrait);
}

pub enum BroadwayMacroError{
    MissingKey,
    MissingActor,
}