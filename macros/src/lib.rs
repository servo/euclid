extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_derive(EuclidMatrix)]
pub fn derive_euclid_matrix(_: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
