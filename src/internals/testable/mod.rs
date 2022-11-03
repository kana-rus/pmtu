use syn::{ItemStruct, ItemEnum, parse::Parse, token};
use super::Build;

pub(super) enum Testable {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

impl Parse for Testable {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(token::Struct) {
            Ok(Self::Struct(input.parse()?))
        } else if input.peek(token::Enum) {
            Ok(Self::Enum(input.parse()?))
        } else {
            Err(input.error("`#[testable]` is only abalable for struct or enum."))
        }
    }
}

impl Build for Testable {
    fn build(self) -> proc_macro2::TokenStream {
        todo!()
    }
}