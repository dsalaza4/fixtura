use proc_macro::TokenStream;

mod codegen;
mod parser;

/// Replaces a `#[test]` function with one that injects fake values for every argument.
///
/// Every parameter receives a randomly generated value via `fake-rs`. Use
/// `#[fixtura(field = value, ...)]` on individual parameters to pin specific fields;
/// everything else is randomised.
///
/// An optional `seed` pins the RNG for deterministic replay:
///
/// ```rust,ignore
/// #[fixtura::test(seed = 42)]
/// fn my_test(user: User) { ... }
/// ```
///
/// On failure the seed is printed so you can paste it back to reproduce the exact values.
#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let seed = match parser::parse_seed(attr.into()) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    codegen::expand(input, seed).into()
}

/// Injects fake values into the arguments of an async (or otherwise attributed) test.
///
/// Unlike `#[fixtura::test]`, this macro does **not** emit `#[test]` itself — pair it
/// with the outer test runner attribute (e.g. `#[tokio::test]`).
///
/// Mark each argument fixtura should own with `#[fixtura]` or `#[fixtura(...)]`.
/// Unmarked arguments are left in the signature for the outer framework to inject.
///
/// ```rust,ignore
/// #[tokio::test]
/// #[fixtura::inject]
/// async fn my_test(
///     pool: PgPool,                              // sqlx owns
///     #[fixtura] user: User,                     // fixtura owns
///     #[fixtura(user_id = user.id)] order: Order, // fixtura owns, with override
/// ) { ... }
/// ```
///
/// Accepts an optional `seed` for deterministic replay:
///
/// ```rust,ignore
/// #[tokio::test]
/// #[fixtura::inject(seed = 42)]
/// async fn my_test(#[fixtura] user: User) { ... }
/// ```
#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    let seed = match parser::parse_seed(attr.into()) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    codegen::expand_inject(input, seed).into()
}
