use proc_macro::TokenStream;

mod codegen;
mod parser;

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let seed = match parser::parse_seed(attr.into()) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    codegen::expand(input, seed).into()
}

#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    let seed = match parser::parse_seed(attr.into()) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    codegen::expand_inject(input, seed).into()
}
