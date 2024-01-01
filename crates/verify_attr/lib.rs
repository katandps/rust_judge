use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use verify_core::attribute::Attribute;

#[proc_macro_attribute]
pub fn verify(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _attr = parse_macro_input!(attr as Attribute);
    let ast = parse_macro_input!(item as ItemFn);
    let fn_name = ast.sig.ident.clone();
    let test_name = Ident::new(&format!("verify_{fn_name}"), Span::call_site());
    let test_fn = quote! {
        #[test]
        fn #test_name() {
            let (stdin, stdout) = (::std::io::stdin(), ::std::io::stdout());
            let (stdin_lock, stdout_lock) = (stdin.lock(), stdout.lock());
            #fn_name(stdin_lock, stdout_lock);
        }
    };
    quote! {
        #[allow(dead_code)]
        #ast
        #test_fn
    }
    .into()
}
