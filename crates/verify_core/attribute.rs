use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, Lit, Meta, MetaNameValue, Token,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyAttribute {
    pub problem_id: String,
    pub epsilon: Option<f64>,
    pub time_limit_ms: u64,
}

impl Parse for VerifyAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punc = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut problem_id = None;
        let mut epsilon = None;
        let mut time_limit_ms = 10000;
        for meta in punc.iter() {
            match meta {
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident();
                    match ident {
                        Some(ident) if ident == "problem_id" => {
                            problem_id = Some(parse_problem_id(nv)?)
                        }
                        Some(ident) if ident == "eps" => epsilon = Some(parse_eps(nv)?),
                        Some(ident) if ident == "tl" => time_limit_ms = parse_tl(nv)?,
                        _ => {
                            return Err(Error::new(
                                Span::call_site(),
                                format!("unknown variable: {:?}", ident),
                            ))
                        }
                    }
                }
                Meta::List(_list) => return Err(Error::new(Span::call_site(), "unknown format")),
                Meta::Path(_path) => return Err(Error::new(Span::call_site(), "unknown format")),
            }
        }
        let Some(problem_id) = problem_id else {
            return Err(Error::new(Span::call_site(), "problem_id is not specified"));
        };
        Ok(VerifyAttribute {
            problem_id,
            epsilon,
            time_limit_ms,
        })
    }
}

fn parse_problem_id(nv: &MetaNameValue) -> syn::Result<String> {
    match &nv.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(litstr) => Ok(litstr.value()),
            _ => Err(Error::new(Span::call_site(), "problem_id must be str")),
        },
        _ => Err(Error::new(Span::call_site(), "problem_id is invalid")),
    }
}
fn parse_eps(nv: &MetaNameValue) -> syn::Result<f64> {
    match &nv.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Float(litfloat) => litfloat.base10_parse(),
            _ => Err(Error::new(Span::call_site(), "eps must be float")),
        },
        _ => Err(Error::new(Span::call_site(), "eps is invalid")),
    }
}
fn parse_tl(nv: &MetaNameValue) -> syn::Result<u64> {
    match &nv.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(litint) => litint.base10_parse(),
            _ => Err(Error::new(Span::call_site(), "tl must be float")),
        },
        _ => Err(Error::new(Span::call_site(), "tl is invalid")),
    }
}

impl ToTokens for VerifyAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let problem_id = self.problem_id.clone();
        let epsilon = self.epsilon;
        let time_limit_ms = self.time_limit_ms;
        quote!(
            ::verify::VerifyAttribute {
                problem_id: #problem_id.to_string(),
                epsilon: #epsilon,
                time_limit_ms: #time_limit_ms
            }
        )
        .to_tokens(tokens)
    }
}
