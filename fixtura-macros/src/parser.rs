use proc_macro2::Span;
use syn::{FnArg, Ident, Type};

pub struct Arg {
    pub ident: Ident,
    pub ty: Type,
    pub span: Span,
}

pub fn parse_args(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::token::Comma>,
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

    Ok(Arg { ident, ty, span })
}
