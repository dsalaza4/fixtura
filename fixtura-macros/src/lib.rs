use proc_macro::TokenStream;

mod codegen;
mod parser;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    codegen::expand(input).into()
}
