use proc_macro2::Span;
use serde::{Deserialize, Serialize};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Expr, Lit, Meta, MetaNameValue, Token,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyAttribute {
    pub problem_id: String,
    pub epsilon: f64,
    pub time_limit: f64,
}

impl Parse for VerifyAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punc = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        let mut problem_id = None;
        let mut epsilon = 0.0;
        let mut time_limit = 10.0;
        for meta in punc.iter() {
            match meta {
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident();
                    match ident {
                        Some(ident) if ident == "problem_id" => {
                            problem_id = Some(parse_problem_id(nv)?)
                        }
                        Some(ident) if ident == "eps" => epsilon = parse_eps(nv)?,
                        Some(ident) if ident == "tl" => time_limit = parse_tl(nv)?,
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
            time_limit,
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
fn parse_tl(nv: &MetaNameValue) -> syn::Result<f64> {
    match &nv.value {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Float(litfloat) => litfloat.base10_parse(),
            _ => Err(Error::new(Span::call_site(), "tl must be float")),
        },
        _ => Err(Error::new(Span::call_site(), "tl is invalid")),
    }
}
