use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Error, Ident, LitInt, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro]
pub fn peano(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as LitInt);
    match peano_impl(input) {
        Ok(res) => res,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

fn peano_impl(input: LitInt) -> Result<TokenStream> {
    let n = input.base10_parse::<u16>()?;
    Ok(peano_type(n))
}

fn peano_type(n: u16) -> TokenStream {
    let mut res = quote! { ::positional::PeanoZero };

    for _ in 0..n {
        res = quote! {
            ::positional::PeanoSucc<#res>
        };
    }

    res
}

struct EncodingInput {
    ident: Ident,
    _comma: Token![,],
    digits: LitStr,
}

impl Parse for EncodingInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(EncodingInput {
            ident: input.parse()?,
            _comma: input.parse()?,
            digits: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn define_encoding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as EncodingInput);
    match encoding_impl(input) {
        Ok(res) => res,
        Err(err) => err.into_compile_error(),
    }
    .into()
}

/// Canonical identifier for the given character.
fn char_ident(c: char) -> Ident {
    format_ident!("Digit{:X}", c as u32)
}

fn encoding_impl(input: EncodingInput) -> Result<TokenStream> {
    let ident = input.ident;

    let digits = input.digits.value();

    let radix_type = peano_type(
        digits
            .chars()
            .count()
            .try_into()
            .map_err(|_| Error::new(input.digits.span(), "radix is too large"))?,
    );
    let radix_def = quote!(pub type Radix = #radix_type;);

    let digit_defs = digits
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let ident = char_ident(c);
            let typ = peano_type(
                i.try_into()
                    .expect("<= radix, which is known to be in range"),
            );
            quote!(pub type #ident = #typ;)
        })
        .collect::<TokenStream>();

    Ok(quote! {
        mod #ident {
            #radix_def
            #digit_defs
        }
    })
}

struct NumberInput {
    encoding: Ident,
    _comma: Token![,],
    digits: LitStr,
}

impl Parse for NumberInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(NumberInput {
            encoding: input.parse()?,
            _comma: input.parse()?,
            digits: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn number(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as NumberInput);
    number_impl(input).into()
}

fn number_impl(input: NumberInput) -> TokenStream {
    let encoding = input.encoding;

    let mut res = quote!(::positional::Term<#encoding::Radix>);

    for c in input.digits.value().chars().rev() {
        let digit = char_ident(c);
        res = quote! {
            ::positional::Seq<
                #encoding::Radix,
                #encoding::#digit,
                #res,
            >
        };
    }

    res
}
