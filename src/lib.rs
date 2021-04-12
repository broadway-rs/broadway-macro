use syn::__private::Span;
use proc_macro::TokenStream;
use syn::{Ident};
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemTrait, AttributeArgs};

#[proc_macro_attribute]
pub fn role(attr: TokenStream, input: TokenStream) -> TokenStream{
    let mut og = input.clone();
    let args = parse_macro_input!(attr as AttributeArgs);
    let mut args = args.iter();
    // Get the key and actor types
    let key = if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = args.next(){
        Some(path)
    }
    else{
        None
    };
    let actor = if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = args.next(){
        Some(path)
    }
    else{
        None
    };

    let input = parse_macro_input!(input as ItemTrait);

    let call_ident = format_ident!("{}Call", input.ident);
    let mut_call_ident = format_ident!("{}MutCall", input.ident);
    let reply_ident = format_ident!("{}Reply", input.ident);
    
    let final_trait_impl = quote!{impl<'a, #input.generics> Role for #input.ident <#input.generics> + 'a};
    let final_trait_types = quote!{
        type Actor = #actor.map_or_else(|s| TokenStream::from(s), TokenStream::from(quote!{compile_error!("No Actor argument given in pos=1!")}));
        type Key = #key.map_or_else(|s| TokenStream::from(s), TokenStream::from(quote!{compile_error!("No Key argument given in pos=0!")}));

        type Calls = Call<#call_ident, #reply_ident>;
        type MutCalls = Call<#mut_call_ident, #reply_ident>;
    };
    og.extend(TokenStream::from(quote!{#final_trait_impl { #final_trait_types }}));
    og
}