use super::no_whitespace_lit_str::NoWhitespaceLitStr;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    token,
};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct Name {
    pub name_token: Option<(Ident, token::Eq)>,
    pub name: NoWhitespaceLitStr,
}

impl Name {
    pub fn parse_after_token(token: Ident, input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_token: Some((token, input.parse()?)),
            name: input.parse()?,
        })
    }
}

impl Parse for Name {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name_token: None,
            name: input.parse()?,
        })
    }
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some((name_token, eq)) = &self.name_token {
            name_token.to_tokens(tokens);
            eq.to_tokens(tokens);
        }
        self.name.to_tokens(tokens);
    }
}
