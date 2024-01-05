use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, Lit, LitFloat, LitStr, Meta, MetaNameValue, Token,
};

#[derive(Debug)]
pub struct VerifyAttribute {
    problem_id: LitStr,
    eps: Option<LitFloat>,
}

impl Parse for VerifyAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punc = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut problem_id = None;
        let mut eps = None;
        for meta in punc.iter() {
            match meta {
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident();
                    match ident {
                        Some(ident) if ident == "problem_id" => {
                            problem_id = Some(parse_problem_id(nv)?)
                        }
                        Some(ident) if ident == "eps" => eps = Some(parse_eps(nv)?),
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
        Ok(VerifyAttribute { problem_id, eps })
    }
}

fn parse_problem_id(nv: &MetaNameValue) -> syn::Result<LitStr> {
    match &nv.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(litstr) => Ok(litstr.clone()),
            _ => Err(Error::new(Span::call_site(), "problem_id must be str")),
        },
        _ => Err(Error::new(Span::call_site(), "problem_id is invalid")),
    }
}
fn parse_eps(nv: &MetaNameValue) -> syn::Result<LitFloat> {
    dbg!(&nv.value);
    match &nv.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Float(litfloat) => Ok(litfloat.clone()),
            _ => Err(Error::new(Span::call_site(), "eps must be float")),
        },
        _ => Err(Error::new(Span::call_site(), "eps is invalid")),
    }
}
