use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use verify_core::{
    attribute::VerifyAttribute,
    save_verify_info,
    service::{AizuOnlineJudge, Service},
};

#[proc_macro_attribute]
pub fn aizu_online_judge(attr: TokenStream, item: TokenStream) -> TokenStream {
    verify_attr::<AizuOnlineJudge>(attr, item)
}
// #[proc_macro_attribute]
// pub fn library_checker(attr: TokenStream, item: TokenStream) -> TokenStream {
//     verify_attr("LibraryChecker", attr, item)
// }
// #[proc_macro_attribute]
// pub fn atcoder(attr: TokenStream, item: TokenStream) -> TokenStream {
//     verify_attr("AtCoder", attr, item)
// }

// #[proc_macro_attribute]
// pub fn yuki_coder(attr: TokenStream, item: TokenStream) -> TokenStream {
//     verify_attr("YukiCoder", attr, item)
// }

fn verify_attr<S: Service>(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parse_macro_input!(attr as VerifyAttribute) {
        attr => {
            // dbg!(std::module_path!());
            let ast = parse_macro_input!(item as ItemFn);
            save_verify_info::<S>(&attr)
                .expect(format!("Failed to save verify info: {}", attr.problem_id).as_str());
            let fn_name = ast.sig.ident.clone();
            let verify_name = Ident::new(&format!("verify_{fn_name}"), Span::call_site());
            let verify = S::build(attr, fn_name);
            let test_fn = quote! {
                #[test]
                #[ignore]
                fn #verify_name() {
                    #verify;
                }
            };
            quote! {
                #[allow(dead_code)]
                #ast
                #test_fn
            }
            .into()
        }
    }
}

#[allow(unused)]
fn get_out_dir() -> Option<String> {
    let mut args = std::env::args();
    // Then we loop through them all, and find the value of "out-dir"
    let mut out_dir = None;
    while let Some(arg) = args.next() {
        if arg == "--out-dir" {
            out_dir = args.next();
        }
    }
    out_dir
}
