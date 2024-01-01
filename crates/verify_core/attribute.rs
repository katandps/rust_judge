pub mod arg;
pub mod name;
pub mod no_whitespace_lit_str;

use arg::Arg;
use quote::ToTokens;
use syn::{parse::Parse, punctuated::Punctuated, token::Comma};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Attribute {
    pub args: Punctuated<Arg, Comma>,
}

impl Parse for Attribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.args.to_tokens(tokens);
    }
}
