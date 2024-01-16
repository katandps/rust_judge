use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(AizuOnlineJudge)]
pub fn derive_aizu_online_judge(input: TokenStream) -> TokenStream {
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
#[proc_macro_derive(LibraryChecker)]
pub fn derive_library_checker(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let service = Ident::new("LibraryChecker", Span::call_site());
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
        #[cfg_attr(feature = "verify_result", doc = include_str!(#md_name))]
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
        #[cfg_attr(coverage_nightly, coverage(off))]
        fn #fn_name() {
            <#ident as ::verify::Verifiable>::fetch_testcases();
        }
    }
}
fn verify(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name = Ident::new(&format!("verify_{ident}"), Span::call_site());
    let ident_str = ident.to_string();
    quote! {
        #[cfg_attr(feature = "verify", test)]
        #[cfg_attr(feature = "verify", ignore)]
        #[cfg_attr(coverage_nightly, coverage(off))]
        fn #fn_name() {
            let res = <#ident as ::verify::Verifiable>::verify();
            if let Ok(res) = res {
                <#ident as ::verify::Verifiable>::output(&res, ::std::file!(), &#ident_str).expect("Failed to write result.");
                assert!(res.success);
            } else {
                panic!("Internal error: {}", #ident::PROBLEM_ID);
            }
        }
    }
}
