use proc_macro2::{Span, TokenStream};
use syn::{Expr, FnArg, Ident, LitInt, Token, Type, parse::ParseStream, punctuated::Punctuated};

pub(crate) fn push_error(combined: &mut Option<syn::Error>, e: syn::Error) {
    match combined {
        Some(c) => c.combine(e),
        None => *combined = Some(e),
    }
}

pub fn parse_seed(attr: TokenStream) -> syn::Result<Option<u64>> {
    if attr.is_empty() {
        return Ok(None);
    }
    syn::parse::Parser::parse2(
        |input: ParseStream| {
            let ident: Ident = input.parse()?;
            if ident != "seed" {
                return Err(syn::Error::new(
                    ident.span(),
                    "expected `seed = <u64>`, e.g. `#[fixtura::test(seed = 42)]`",
                ));
            }
            input.parse::<Token![=]>()?;
            let lit: LitInt = input.parse()?;
            let value: u64 = lit.base10_parse().map_err(|_| {
                syn::Error::new(lit.span(), "seed must be a u64 literal, e.g. `seed = 42`")
            })?;
            Ok(Some(value))
        },
        attr,
    )
}

pub struct FieldOverride {
    pub path: Vec<Ident>,
    pub expr: Expr,
    pub span: Span,
}

pub struct Arg {
    pub ident: Ident,
    pub ty: Type,
    pub span: Span,
    pub overrides: Vec<FieldOverride>,
    /// false only for inject pass-through args (no `#[fixtura]` marker)
    pub owned: bool,
    /// original FnArg, used to re-emit pass-through args into the signature
    pub fn_arg: FnArg,
}

pub fn parse_args(inputs: &Punctuated<FnArg, Token![,]>, is_inject: bool) -> syn::Result<Vec<Arg>> {
    let mut args = Vec::new();
    let mut combined: Option<syn::Error> = None;

    for arg in inputs.iter() {
        match parse_one(arg, is_inject) {
            Ok(a) => args.push(a),
            Err(e) => push_error(&mut combined, e),
        }
    }

    if let Some(e) = combined {
        return Err(e);
    }
    Ok(args)
}

fn extract_ident(pat: &syn::Pat, msg: &str) -> syn::Result<Ident> {
    match pat {
        syn::Pat::Ident(p) => Ok(p.ident.clone()),
        _ => Err(syn::Error::new_spanned(pat, msg)),
    }
}

fn parse_one(arg: &FnArg, is_inject: bool) -> syn::Result<Arg> {
    let pat_type = match arg {
        FnArg::Receiver(r) => {
            return Err(syn::Error::new_spanned(
                r,
                "#[fixtura::test] does not support `self`",
            ));
        }
        FnArg::Typed(pt) => pt,
    };

    const DUPLICATE_MSG: &str = "duplicate `#[fixtura]` — only one is allowed per argument";
    let fixtura_attrs: Vec<_> = pat_type
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("fixtura"))
        .collect();

    if fixtura_attrs.len() > 1 {
        let mut err = syn::Error::new_spanned(fixtura_attrs[1], DUPLICATE_MSG);
        for extra in &fixtura_attrs[2..] {
            err.combine(syn::Error::new_spanned(extra, DUPLICATE_MSG));
        }
        return Err(err);
    }

    let fixtura_attr = fixtura_attrs.into_iter().next();

    if is_inject && fixtura_attr.is_none() {
        let ident = extract_ident(
            &pat_type.pat,
            "#[fixtura::inject] only supports simple patterns like `pool: PgPool`",
        )?;
        return Ok(Arg {
            span: ident.span(),
            ty: (*pat_type.ty).clone(),
            ident,
            overrides: vec![],
            owned: false,
            fn_arg: arg.clone(),
        });
    }

    if let Type::Reference(r) = &*pat_type.ty {
        return Err(syn::Error::new_spanned(
            r,
            "#[fixtura::test] does not support reference arguments — use the owned type",
        ));
    }

    let ident = extract_ident(
        &pat_type.pat,
        "#[fixtura::test] only supports simple patterns like `user: User`",
    )?;

    let span = ident.span();
    let ty = (*pat_type.ty).clone();

    let mut overrides = Vec::new();
    if let Some(attr) = fixtura_attr {
        match &attr.meta {
            syn::Meta::Path(_) => {
                if !is_inject {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "in `#[fixtura::test]`, all args are owned by default — `#[fixtura]` is not needed here",
                    ));
                }
            }
            syn::Meta::List(_) => {
                overrides.extend(attr.parse_args_with(parse_fixtura_args)?);
            }
            syn::Meta::NameValue(nv) => {
                return Err(syn::Error::new_spanned(
                    nv,
                    "expected `#[fixtura]` or `#[fixtura(field = value, ...)]`",
                ));
            }
        }
    }

    Ok(Arg {
        ident,
        ty,
        span,
        overrides,
        owned: true,
        fn_arg: arg.clone(),
    })
}

fn parse_fixtura_args(input: ParseStream) -> syn::Result<Vec<FieldOverride>> {
    if input.is_empty() {
        return Err(input.error(
            "#[fixtura(...)] requires at least one field override, e.g. #[fixtura(active = false)]",
        ));
    }
    Ok(
        Punctuated::<FieldOverride, Token![,]>::parse_terminated_with(input, parse_field_override)?
            .into_iter()
            .collect(),
    )
}

fn parse_field_override(input: ParseStream) -> syn::Result<FieldOverride> {
    let first: Ident = input.parse()?;
    let span = first.span();
    let mut path = vec![first];

    while input.peek(Token![.]) {
        input.parse::<Token![.]>()?;
        path.push(input.parse()?);
    }

    input.parse::<Token![=]>()?;
    let expr: Expr = input.parse()?;

    Ok(FieldOverride { path, expr, span })
}
