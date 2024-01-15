use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(AizuOnlineJudge)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let service = Ident::new("AizuOnlineJudge", Span::call_site());
    let implement = implement(&input.ident, &service);
    let fetch_testcases = fetch_testcases(&input.ident);
    let verify = verify(&input.ident);
    quote! {
        #implement
        #fetch_testcases
        #verify
    }
    .into()
}
fn implement(ident: &Ident, service: &Ident) -> proc_macro2::TokenStream {
    let md_name = LitStr::new(&format!("result_{ident}.md"), Span::call_site());
    quote! {
        #[cfg_attr(feature = "verify", doc = include_str!(#md_name))]
        impl ::verify::Verifiable for #ident {
            type SERVICE = ::verify::#service;
        }
    }
}
fn fetch_testcases(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name = Ident::new(&format!("fetch_testcases_{ident}"), Span::call_site());
    quote! {
        #[cfg_attr(feature = "fetch_testcases", test)]
        #[cfg_attr(feature = "fetch_testcases", ignore)]
        fn #fn_name() {
            <#ident as ::verify::Verifiable>::fetch_testcases();
        }
    }
}
fn verify(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name = Ident::new(&format!("verify_{ident}"), Span::call_site());
    quote! {
        #[cfg_attr(feature = "verify", test)]
        #[cfg_attr(feature = "fetch_testcases", ignore)]
        fn #fn_name() {
            let res = <#ident as ::verify::Verifiable>::verify();
            if let Ok(res) = res {
                res.output().expect("Failed to write result.");
                assert!(res.success);
            } else {
                panic!("Internal error: {}", #ident::PROBLEM_ID);
            }
        }
    }
}
