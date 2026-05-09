use proc_macro2::{Literal, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{visit::Visit, FnArg, Ident, ItemFn, Signature};

use crate::parser::{parse_args, push_error, Arg};

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

fn check_forward_refs(owned: &[&Arg]) -> syn::Result<()> {
    let mut combined: Option<syn::Error> = None;

    for (i, arg) in owned.iter().enumerate() {
        for ov in &arg.overrides {
            let mut roots = PathRoots(vec![]);
            roots.visit_expr(&ov.expr);
            for used in roots.0 {
                let err = if used == arg.ident {
                    Some(syn::Error::new(
                        used.span(),
                        format!("`{used}` cannot reference itself in `#[fixtura]`"),
                    ))
                } else if owned[i + 1..].iter().any(|a| a.ident == used) {
                    Some(syn::Error::new(
                        used.span(),
                        format!("`{used}` is not yet in scope — `#[fixtura]` expressions are evaluated top-to-bottom"),
                    ))
                } else {
                    None
                };
                if let Some(e) = err {
                    push_error(&mut combined, e);
                }
            }
        }
    }

    if let Some(e) = combined {
        return Err(e);
    }
    Ok(())
}

pub fn expand(input: ItemFn, seed: Option<u64>) -> TokenStream {
    if let Some(async_token) = &input.sig.asyncness {
        return syn::Error::new_spanned(
            async_token,
            "async functions are not supported by `#[fixtura::test]` — use `#[tokio::test]` above `#[fixtura::inject]` instead",
        )
        .to_compile_error();
    }
    expand_inner(input, Some(quote! { #[test] }), seed)
}

pub fn expand_inject(input: ItemFn, seed: Option<u64>) -> TokenStream {
    expand_inner(input, None, seed)
}

fn expand_inner(input: ItemFn, test_attr: Option<TokenStream>, seed: Option<u64>) -> TokenStream {
    let is_inject = test_attr.is_none();

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

    let fn_span = sig.fn_token.span;

    let args = match parse_args(&sig.inputs, is_inject) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error(),
    };

    let owned: Vec<&Arg> = args.iter().filter(|a| a.owned).collect();
    let passthrough: Vec<&FnArg> = args
        .iter()
        .filter(|a| !a.owned)
        .map(|a| &a.fn_arg)
        .collect();

    if let Err(e) = check_forward_refs(&owned) {
        return e.to_compile_error();
    }

    let rng_preamble = if owned.is_empty() {
        quote! {}
    } else {
        rng_preamble(seed, fn_span)
    };
    let bindings = owned.iter().map(|a| binding(a));
    // When emitting our own #[test], filter it from attrs to avoid duplication.
    let attrs = attrs
        .into_iter()
        .filter(|a| test_attr.is_none() || !a.path().is_ident("test"));
    let passthrough_inputs: syn::punctuated::Punctuated<FnArg, syn::token::Comma> =
        passthrough.into_iter().cloned().collect();
    let sig = Signature {
        inputs: passthrough_inputs,
        ..sig
    };
    let stmts = &block.stmts;

    quote_spanned! { fn_span =>
        #test_attr
        #(#attrs)*
        #vis #sig {
            use ::fake::Fake;
            #rng_preamble
            #(#bindings)*
            #(#stmts)*
        }
    }
}

fn rng_preamble(seed: Option<u64>, span: Span) -> TokenStream {
    let seed_expr = match seed {
        Some(n) => {
            let lit = Literal::u64_suffixed(n);
            quote! { #lit }
        }
        None => quote! { ::fake::rand::random::<u64>() },
    };
    quote_spanned! { span =>
        let __fixtura_seed: u64 = #seed_expr;
        eprintln!("[fixtura] seed = {}", __fixtura_seed);
        let mut __fixtura_rng = {
            use ::fake::rand::SeedableRng;
            ::fake::rand::rngs::StdRng::seed_from_u64(__fixtura_seed)
        };
    }
}

fn binding(arg: &Arg) -> TokenStream {
    let ident = &arg.ident;
    let ty = &arg.ty;

    if arg.overrides.is_empty() {
        return quote_spanned! { arg.span =>
            let #ident: #ty = ::fake::Faker.fake_with_rng(&mut __fixtura_rng);
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
            let mut v: #ty = ::fake::Faker.fake_with_rng(&mut __fixtura_rng);
            #(#override_stmts)*
            v
        };
    }
}
