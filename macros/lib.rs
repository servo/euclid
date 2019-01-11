/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![recursion_limit = "128"]

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

#[proc_macro_derive(EuclidMatrix)]
pub fn derive_euclid_matrix(_: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
