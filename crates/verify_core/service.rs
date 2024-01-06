use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{LitFloat, LitStr};

pub trait Service {
    fn build(problem_id: LitStr, eps: LitFloat, tl: LitFloat, f: Ident) -> TokenStream;

    fn run(f: Ident) -> TokenStream {
        quote! {
            let case = "12345".as_bytes();
            let mut buf = Vec::new();
            #f(case, &mut buf);
        }
    }
}

pub struct AizuOnlineJudge;

impl Service for AizuOnlineJudge {
    fn build(_problem_id: LitStr, _eps: LitFloat, _tl: LitFloat, f: Ident) -> TokenStream {
        // dbg!(problem_id, eps, tl, &f);
        quote! {
            let case = "12345".as_bytes();
            let mut buf = Vec::new();
            #f(case, &mut buf);
        }
    }
}

// pub struct LibraryChecker;

// impl LibraryChecker {
//     fn download_testcases(_problem_id: &str) {
//         todo!()
//     }
//     fn verify(_attr: VerifyAttribute, _f: &dyn Fn() -> ()) -> VerifyResult {
//         todo!()
//     }
// }
// pub struct AtCoder;

// impl Service for AtCoder {
//     fn download_testcases(_problem_id: &str) {
//         todo!()
//     }
//     fn verify(_attr: VerifyAttribute, _f: &dyn Fn() -> ()) -> VerifyResult {
//         todo!()
//     }
// }
// pub struct YukiCoder;

// impl Service for YukiCoder {
//     fn download_testcases(_problem_id: &str) {
//         todo!()
//     }
//     fn verify(_attr: VerifyAttribute, _f: &dyn Fn() -> ()) -> VerifyResult {
//         todo!()
//     }
// }
