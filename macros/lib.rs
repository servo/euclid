/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#[macro_use]
extern crate darling;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate synstructure;

use proc_macro::TokenStream;

mod euclid_matrix;

#[proc_macro_derive(EuclidMatrix)]
pub fn derive_euclid_matrix(input: TokenStream) -> TokenStream {
    let input = syn::parse(input).unwrap();
    euclid_matrix::derive(input).into()
}
