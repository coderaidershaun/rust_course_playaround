extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {

    // Parse the input function
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);

    // Create string representation of function
    let function_str: String = format!("{}", input_fn.to_token_stream());

    // Define a new function with the same signature as the input function
    let fn_ident: proc_macro2::Ident = input_fn.sig.ident;
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = input_fn.sig.inputs;
    let fn_generics: syn::Generics = input_fn.sig.generics;

    // Generate output function
    let output: proc_macro2::TokenStream = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}
