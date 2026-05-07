use proc_macro2::{Span, TokenStream};
use syn::{parse::ParseStream, punctuated::Punctuated, Expr, FnArg, Ident, LitInt, Token, Type};

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
    inputs.iter().map(|arg| parse_one(arg, is_inject)).collect()
}

fn parse_one(arg: &FnArg, is_inject: bool) -> syn::Result<Arg> {
    let pat_type = match arg {
        FnArg::Receiver(r) => {
            return Err(syn::Error::new_spanned(
                r,
                "#[fixtura::test] does not support `self`",
            ))
        }
        FnArg::Typed(pt) => pt,
    };

    let fixtura_attr = pat_type.attrs.iter().find(|a| a.path().is_ident("fixtura"));

    // In inject mode, args without #[fixtura] are pass-throughs — skip all validation.
    if is_inject && fixtura_attr.is_none() {
        let ident = match &*pat_type.pat {
            syn::Pat::Ident(p) => p.ident.clone(),
            _ => {
                return Err(syn::Error::new_spanned(
                    &pat_type.pat,
                    "#[fixtura::inject] only supports simple patterns like `pool: PgPool`",
                ))
            }
        };
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

    let ident = match &*pat_type.pat {
        syn::Pat::Ident(p) => p.ident.clone(),
        _ => {
            return Err(syn::Error::new_spanned(
                &pat_type.pat,
                "#[fixtura::test] only supports simple patterns like `user: User`",
            ))
        }
    };

    let span = ident.span();
    let ty = (*pat_type.ty).clone();

    let mut overrides = Vec::new();
    if let Some(attr) = fixtura_attr {
        match &attr.meta {
            syn::Meta::Path(_) => {
                // #[fixtura] with no args
                if !is_inject {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "in `#[fixtura::test]`, all args are owned by default — `#[fixtura]` is not needed here",
                    ));
                }
                // owned, no overrides
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
    let fields =
        Punctuated::<FieldOverride, Token![,]>::parse_terminated_with(input, parse_field_override)?;
    Ok(fields.into_iter().collect())
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
