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
        quote!{#path}
    }
    else{
        quote!{compile_error!("No Actor argument given in pos=1!")}
    };
    let actor = if let Some(syn::NestedMeta::Meta(syn::Meta::Path(path))) = args.next(){
        quote!{#path}
    }
    else{
        quote!{compile_error!("No Actor argument given in pos=1!")}
    };

    let input = parse_macro_input!(input as ItemTrait);

    let trait_name = input.ident.clone();
    let trait_generics = input.generics;

    let call_ident = format_ident!("{}Call", input.ident);
    let mut_call_ident = format_ident!("{}MutCall", input.ident);
    let reply_ident = format_ident!("{}Reply", input.ident);
    
    let final_trait = quote!{
        impl<'a, #trait_generics> Role for #trait_name <#trait_generics> + 'a{
        type Actor = #actor ;
        type Key = #key ;

        type Calls = Call<#call_ident, #reply_ident>;
        type MutCalls = Call<#mut_call_ident, #reply_ident>;
    }};

    // Generate {}Call enum
    let call_names = input
        .items
        .iter()
        .filter_map(|item| if let syn::TraitItem::Method(method) = item{
            Some(method.sig.clone())
        }
        else{
            None
        })
        .filter(|item| if let Some(syn::FnArg::Receiver(rec)) = item.inputs.first(){
            if !rec.reference.is_none() && rec.mutability.is_none(){
                true
            }
            else{
                false
            }
        }
        else{
            false
        });

    let calls = call_names.clone()
        .fold(quote!{}, |stream, sig|{
            let variant = sig.ident;
            let variant_args = syn::punctuated::Punctuated::<syn::FnArg, syn::token::Comma>::from(sig.inputs.clone().into_iter().skip(1).collect());
            if variant_args.len() > 0{
                quote!{
                    #variant(#variant_args),
                    #stream
                }
            }
            else{
                quote!{
                    #variant,
                    #stream
                }
            }
        });

    let call_defs = call_names
        .fold(quote!{}, |stream, sig|{
            let variant = sig.ident;
            let variant_args = syn::punctuated::Punctuated::<syn::FnArg, syn::token::Comma>::from(sig.inputs.clone().into_iter().skip(1).collect());
            let variant_arg_names = syn::punctuated::Punctuated::<syn::Pat, syn::token::Comma>::from({
                sig
                    .inputs
                    .clone()
                    .into_iter()
                    .skip(1)
                    .filter_map(|fn_arg| 
                        if let syn::FnArg::Typed(pat) = fn_arg{
                            Some(*pat.pat.clone())
                        }
                        else{
                            None
                        }
                    )
                    .collect()
            });
            if variant_args.len() > 0{
                quote!{
                    #call_ident::#variant{#variant_args} => self.return_channel.send(#reply_ident::#variant(#actor::#variant(actor, #variant_arg_names).await)).await,
                    #stream
                }
            }
            else{
                quote!{
                    #call_ident::#variant => self.return_channel.send(#reply_ident::#variant(#actor::#variant(actor).await)).await,
                    #stream
                }
            }
        });

    let call_def = quote!{
        pub enum #call_ident{
            #calls
        }

        #[async_trait]
        impl Handler<#actor> for Call<#call_ident, #reply_ident>{
            async fn handle(self, actor: &#actor){
                match self.call{
                    #call_defs
                };
            }
        }
    };

    // Generate {}MutCall enum
    let mut_call_names = input
    .items
    .iter()
    .filter_map(|item| if let syn::TraitItem::Method(method) = item{
        Some(method.sig.clone())
    }
    else{
        None
    })
    .filter(|item| if let Some(syn::FnArg::Receiver(rec)) = item.inputs.first(){
        if !rec.reference.is_none() && !rec.mutability.is_none(){
            true
        }
        else{
            false
        }
    }
    else{
        false
    });

    let mut_calls = mut_call_names.clone()
        .fold(quote!{}, |stream, sig|{
            let variant = sig.ident;
            let variant_args = syn::punctuated::Punctuated::<syn::FnArg, syn::token::Comma>::from(sig.inputs.clone().into_iter().skip(1).collect());
            if variant_args.len() > 0{
                quote!{
                    #variant{#variant_args},
                    #stream
                }
            }
            else{
                quote!{
                    #variant,
                    #stream
                }
            }
        });

    let mut_call_defs = mut_call_names
        .fold(quote!{}, |stream, sig|{
            let variant = sig.ident;
            let variant_args = syn::punctuated::Punctuated::<syn::FnArg, syn::token::Comma>::from(sig.inputs.clone().into_iter().skip(1).collect());
            let variant_arg_names = syn::punctuated::Punctuated::<syn::Pat, syn::token::Comma>::from({
                sig
                    .inputs
                    .clone()
                    .into_iter()
                    .skip(1)
                    .filter_map(|fn_arg| 
                        if let syn::FnArg::Typed(pat) = fn_arg{
                            Some(*pat.pat.clone())
                        }
                        else{
                            None
                        }
                    )
                    .collect()
            });
            if variant_args.len() > 0{
                quote!{
                    #call_ident::#variant(#variant_args) => self.return_channel.send(#reply_ident::#variant(#actor::#variant(actor, #variant_arg_names).await)).await,
                    #stream
                }
            }
            else{
                quote!{
                    #call_ident::#variant => self.return_channel.send(#reply_ident::#variant(#actor::#variant(actor).await)).await,
                    #stream
                }
            }
        });

    let mut_call_def = quote!{
        pub enum #mut_call_ident{
            #mut_calls
        }

        #[async_trait]
        impl MutHandler<#actor> for Call<#mut_call_ident, #reply_ident>{
            async fn handle(self, actor: &mut #actor){
                match self.call{
                    #mut_call_defs
                };
            }
        }
    };
    

    //og.extend(TokenStream::from(quote!{#final_trait_impl { #final_trait_types }}));
    og.extend(TokenStream::from(final_trait));
    og.extend(TokenStream::from(call_def));
    og.extend(TokenStream::from(mut_call_def));
    og
}