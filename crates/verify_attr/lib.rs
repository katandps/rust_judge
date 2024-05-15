use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[proc_macro_derive(AizuOnlineJudge)]
pub fn derive_aizu_online_judge(input: TokenStream) -> TokenStream {
    derive(input, Ident::new("AizuOnlineJudge", Span::call_site()))
}
#[proc_macro_derive(LibraryChecker)]
pub fn derive_library_checker(input: TokenStream) -> TokenStream {
    derive(input, Ident::new("LibraryChecker", Span::call_site()))
}
#[proc_macro_derive(Yukicoder)]
pub fn derive_yukicoder(input: TokenStream) -> TokenStream {
    derive(input, Ident::new("Yukicoder", Span::call_site()))
}
#[proc_macro_derive(AtCoder)]
pub fn derive_atcoder(input: TokenStream) -> TokenStream {
    derive(input, Ident::new("AtCoder", Span::call_site()))
}
fn derive(input: TokenStream, service: Ident) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let implement = implement(&input.ident, &service);
    // 問題情報をファイルに出力する
    // cliでtestcaseをfetchするようにする
    // verifyはファイルを読み込んで行う
    let save_metadata = save_metadata(&input.ident);
    let fetch_testcases = fetch_testcases(&input.ident);
    let verify = verify(&input.ident);
    quote! {
        #save_metadata
        #implement
        #fetch_testcases
        #verify
    }
    .into()
}

fn save_metadata(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name: Ident = Ident::new(&format!("save_metadata_{ident}"), Span::call_site());
    quote! {
        #[cfg_attr(feature = "save_metadata", test)]
        #[cfg_attr(feature = "save_metadata", ignore)]
        #[cfg_attr(coverage_nightly, coverage(off))]
        fn #fn_name() -> anyhow::Result<()>{
            <#ident as ::verify::Verifiable>::save_metadata()
        }
    }
}

fn implement(ident: &Ident, service: &Ident) -> proc_macro2::TokenStream {
    let md_name = LitStr::new(&format!("result_{ident}.md"), Span::call_site());
    quote! {
        #[cfg_attr(feature = "verify_result", doc = include_str!(#md_name))]
        #[cfg_attr(coverage_nightly, coverage(off))]
        impl ::verify::Verifiable for #ident {
            type SERVICE = ::verify::#service;
        }
    }
}
fn fetch_testcases(ident: &Ident) -> proc_macro2::TokenStream {
    let fn_name: Ident = Ident::new(&format!("fetch_testcases_{ident}"), Span::call_site());
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
            match res {
                Ok(res) => <#ident as ::verify::Verifiable>::output(&res, ::std::file!(), &#ident_str).expect("Failed to write result."),
                Err(e) => panic!("Internal error: {}: {}", #ident::PROBLEM_ID, e),
            }
        }
    }
}
