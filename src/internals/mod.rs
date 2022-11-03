use proc_macro2::TokenStream;
use syn::{parse2, Result};

trait Build {
    fn build(self) -> TokenStream;
}

mod testable;
pub(super) fn testable(target: TokenStream) -> Result<TokenStream> {
    use testable::Testable;
    Ok(parse2::<Testable>(target)?.build())
}