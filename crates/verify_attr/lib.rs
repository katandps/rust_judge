use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use verify_core::attribute::VerifyAttribute;

#[proc_macro_attribute]
pub fn verify(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as VerifyAttribute);
    dbg!(attr);
    let ast = parse_macro_input!(item as ItemFn);
    let fn_name = ast.sig.ident.clone();
    let test_name = Ident::new(&format!("verify_{fn_name}"), Span::call_site());
    let test_fn = quote! {
        #[test]
        fn #test_name() {
            let res = ::std::panic::catch_unwind(|| {
                let (stdin, stdout) = (::std::io::stdin(), ::std::io::stdout());
                let (stdin_lock, stdout_lock) = (stdin.lock(), stdout.lock());
                #fn_name(stdin_lock, stdout_lock);
            });
            ::std::assert!(res.is_ok(), "{}", ::std::stringify!(#fn_name));
        }
    };
    quote! {
        #[allow(dead_code)]
        #ast
        #test_fn
    }
    .into()
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
