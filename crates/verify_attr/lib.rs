use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(AizuOnlineJudge)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fetch_testcases = fetch_testcases(&input.ident);
    let verify = verify(&input.ident);
    quote! {
        #fetch_testcases
        #verify
    }
    .into()
}
fn fetch_testcases(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name = Ident::new(&format!("fetch_testcases_{ident}"), Span::call_site());
    quote! {
        #[test]
        #[ignore]
        #[cfg(feature = "fetch_testcases")]
        fn #fn_name() {
            #ident::fetch_testcases();
        }
    }
}
fn verify(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name = Ident::new(&format!("verify_{ident}"), Span::call_site());
    let md = LitStr::new(&format!("{ident}.md"), Span::call_site());
    quote! {
        #[test]
        #[ignore]
        #[cfg_attr(feature = "verify_doc", doc = include_str!(#md))]
        #[cfg(feature = "verify")]
        pub fn #fn_name() {
            let res = #ident::verify();
            if let Ok(res) = res {
                res.output().expect("Failed to write result.");
                assert!(res.success);
            } else {
                panic!("Internal error: {}", #ident::PROBLEM_ID);
            }
        }
    }
}
