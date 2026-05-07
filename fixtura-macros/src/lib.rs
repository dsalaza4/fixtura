use proc_macro::TokenStream;

mod codegen;
mod parser;

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    codegen::expand(input).into()
}
