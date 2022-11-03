use proc_macro::TokenStream;
mod internals;

#[proc_macro_attribute]
pub fn testable(_: TokenStream, target: TokenStream) -> TokenStream {
    match internals::testable(target.into()) {
        Err(error) => error.into_compile_error(),
        Ok(expanded) => expanded,
    }.into()
}