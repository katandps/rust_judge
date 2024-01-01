use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Error, LitStr,
};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
pub struct NoWhitespaceLitStr {
    pub litstr: LitStr,
}

impl Parse for NoWhitespaceLitStr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let litstr: LitStr = input.parse()?;
        if litstr.value().contains(char::is_whitespace) {
            return Err(Error::new_spanned(
                litstr,
                "string literal should not contain whitespace",
            ));
        }
        Ok(Self { litstr })
    }
}

impl ToTokens for NoWhitespaceLitStr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.litstr.to_tokens(tokens)
    }
}
