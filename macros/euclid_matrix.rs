/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use darling::util::IdentList;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{DeriveInput, Path};
use synstructure::{Structure, VariantInfo};

pub fn derive(mut input: DeriveInput) -> TokenStream {
    TokenStream::new()
}
