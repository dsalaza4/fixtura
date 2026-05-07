use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{visit::Visit, Ident, ItemFn, Signature};

use crate::parser::{parse_args, Arg};

struct PathRoots(Vec<Ident>);

impl<'ast> Visit<'ast> for PathRoots {
    fn visit_expr_path(&mut self, node: &'ast syn::ExprPath) {
        if node.qself.is_none() {
            if let Some(ident) = node.path.get_ident() {
                self.0.push(ident.clone());
            }
        }
        syn::visit::visit_expr_path(self, node);
    }
}

fn check_forward_refs(args: &[Arg]) -> syn::Result<()> {
    for (i, arg) in args.iter().enumerate() {
        for ov in &arg.overrides {
            let mut roots = PathRoots(vec![]);
            roots.visit_expr(&ov.expr);
            for used in roots.0 {
                if used == arg.ident {
                    return Err(syn::Error::new(
                        used.span(),
                        format!("`{used}` cannot reference itself in `#[with]`"),
                    ));
                }
                if args[i + 1..].iter().any(|a| a.ident == used) {
                    return Err(syn::Error::new(
                        used.span(),
                        format!("`{used}` is not yet in scope — `#[with]` expressions are evaluated top-to-bottom"),
                    ));
                }
            }
        }
    }
    Ok(())
}

pub fn expand(input: ItemFn) -> TokenStream {
    if let Some(async_token) = &input.sig.asyncness {
        return syn::Error::new_spanned(
            async_token,
            "async functions are not supported by `#[fixtura::test]` — use `#[tokio::test]` above `#[fixtura::inject]` instead",
        )
        .to_compile_error();
    }
    expand_inner(input, Some(quote! { #[test] }))
}

pub fn expand_inject(input: ItemFn) -> TokenStream {
    expand_inner(input, None)
}

fn expand_inner(input: ItemFn, test_attr: Option<TokenStream>) -> TokenStream {
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

    if let Err(e) = check_forward_refs(&args) {
        return e.to_compile_error();
    }

    let bindings = args.iter().map(binding);
    // When emitting our own #[test], filter it from attrs to avoid duplication.
    let attrs = attrs
        .into_iter()
        .filter(|a| test_attr.is_none() || !a.path().is_ident("test"));
    let sig = Signature {
        inputs: Default::default(),
        ..sig
    };
    let stmts = &block.stmts;

    quote! {
        #test_attr
        #(#attrs)*
        #vis #sig {
            use ::fake::Fake;
            #(#bindings)*
            #(#stmts)*
        }
    }
}

fn binding(arg: &Arg) -> TokenStream {
    let ident = &arg.ident;
    let ty = &arg.ty;

    if arg.overrides.is_empty() {
        return quote_spanned! { arg.span =>
            let #ident: #ty = ::fake::Faker.fake();
        };
    }

    let override_stmts = arg.overrides.iter().map(|o| {
        let expr = &o.expr;
        let field_access = o.path.iter().fold(quote! { v }, |acc, seg| {
            quote! { #acc.#seg }
        });
        quote_spanned! { o.span =>
            #field_access = #expr;
        }
    });

    quote_spanned! { arg.span =>
        let #ident: #ty = {
            let mut v: #ty = ::fake::Faker.fake();
            #(#override_stmts)*
            v
        };
    }
}
