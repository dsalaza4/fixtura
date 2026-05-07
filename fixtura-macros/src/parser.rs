use proc_macro2::Span;
use syn::{
    parse::ParseStream, punctuated::Punctuated, Expr, FnArg, Ident, Token, Type,
};

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
}

pub fn parse_args(
    inputs: &Punctuated<FnArg, Token![,]>,
) -> syn::Result<Vec<Arg>> {
    inputs.iter().map(parse_one).collect()
}

fn parse_one(arg: &FnArg) -> syn::Result<Arg> {
    let pat_type = match arg {
        FnArg::Receiver(r) => {
            return Err(syn::Error::new_spanned(
                r,
                "#[fixtura::test] does not support `self`",
            ))
        }
        FnArg::Typed(pt) => pt,
    };

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
    for attr in &pat_type.attrs {
        if attr.path().is_ident("with") {
            let mut fields = attr.parse_args_with(parse_with_args)?;
            overrides.append(&mut fields);
        }
    }

    Ok(Arg { ident, ty, span, overrides })
}

fn parse_with_args(input: ParseStream) -> syn::Result<Vec<FieldOverride>> {
    if input.is_empty() {
        return Err(input.error(
            "#[with] requires at least one field override, e.g. #[with(active = false)]",
        ));
    }
    let fields = Punctuated::<FieldOverride, Token![,]>::parse_terminated_with(
        input,
        parse_field_override,
    )?;
    Ok(fields.into_iter().collect())
}

fn parse_field_override(input: ParseStream) -> syn::Result<FieldOverride> {
    let first: Ident = input.parse()?;
    let span = first.span();
    let mut path = vec![first];

    while input.peek(Token![.]) {
        let _: Token![.] = input.parse()?;
        path.push(input.parse()?);
    }

    let _: Token![=] = input.parse()?;
    let expr: Expr = input.parse()?;

    Ok(FieldOverride { path, expr, span })
}
