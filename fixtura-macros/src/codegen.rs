use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{ItemFn, Signature};

use crate::parser::parse_args;

pub fn expand(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let args = match parse_args(&sig.inputs) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let bindings = args.iter().map(|arg| {
        let ident = &arg.ident;
        let ty = &arg.ty;
        quote_spanned! { arg.span =>
            let #ident: #ty = ::fake::Faker.fake();
        }
    });

    // Filter out #[test] to avoid duplicating it — we always emit our own.
    let attrs = attrs.iter().filter(|a| !a.path().is_ident("test"));
    let sig = Signature {
        inputs: Default::default(),
        ..sig
    };
    let stmts = &block.stmts;

    quote! {
        #[test]
        #(#attrs)*
        #vis #sig {
            use ::fake::Fake;
            #(#bindings)*
            #(#stmts)*
        }
    }
}
