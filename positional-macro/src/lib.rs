use proc_macro2::TokenStream;
use quote::quote;
use syn::{LitInt, parse_macro_input};

#[proc_macro]
pub fn peano(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LitInt);
    match peano_impl(input) {
        Ok(res) => res,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

fn peano_impl(input: LitInt) -> syn::Result<TokenStream> {
    let int = input.base10_parse::<u16>()?;
    let mut res = quote! { ::positional::Zero };

    for _ in 0..int {
        res = quote! {
            ::positional::peano::Successor<#res>
        };
    }

    Ok(res)
}
