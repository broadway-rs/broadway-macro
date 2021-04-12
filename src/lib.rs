use syn::__private::Span;
use proc_macro::TokenStream;
use syn::{Ident};
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemTrait, AttributeArgs};

#[proc_macro_attribute]
pub fn role(attr: TokenStream, input: TokenStream) -> TokenStream{
    let og = input.clone();
    let args = parse_macro_input!(attr as AttributeArgs);
    // Get the key and actor types
    let mut key = None;
    let mut actor = None;
    for arg in args.iter(){
        if let syn::NestedMeta::Meta(meta) = arg{
            if let syn::Meta::NameValue(name_value) = meta{
                if let Some(path) = name_value.path.get_ident(){
                    match path{
                        key_path if *key_path == syn::Ident::new("Key", Span::call_site()) => key = Some(name_value.lit.clone()),
                        act_path if *act_path == syn::Ident::new("Actor", Span::call_site()) => actor = Some(name_value.lit.clone()),
                        _ => {},
                    }
                }
            }
        }
    }
    let key = key.unwrap();
    let actor = actor.unwrap();

    let input = parse_macro_input!(input as ItemTrait);

    let call_ident = format_ident!("{}Call", input.ident);
    let mut_call_ident = format_ident!("{}MutCall", input.ident);
    let reply_ident = format_ident!("{}Reply", input.ident);
    
    let final_trait_impl = quote!{impl<'a, #input.generics> Role for #input.ident <#input.generics> + 'a};
    let final_trait_types = quote!{
        type Actor = #actor;
        type Key = #key;

        type Calls = Call<#call_ident, #reply_ident>;
        type MutCalls = Call<#mut_call_ident, #reply_ident>;
    };
    (quote!{#final_trait_impl { #final_trait_types }}).into()
}