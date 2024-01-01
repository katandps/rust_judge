use super::name::Name;

use proc_macro2::Ident;
use quote::ToTokens;
use syn::{ext::IdentExt, parse::Parse, LitStr};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub enum Arg {
    Name(Name),
}

impl Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            input.parse().map(Self::Name)
        } else if lookahead.peek(Ident::peek_any) {
            let token: Ident = input.parse()?;
            match token.to_string().as_str() {
                "name" => Name::parse_after_token(token, input).map(Self::Name),
                _ => Err(input.error("expected `name` | `eps`")),
            }
        } else {
            Err(input.error("expected `name` | `eps`"))
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Arg::Name(arg) => arg.to_tokens(tokens),
        }
    }
}
