/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use darling::util::IdentList;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{self, DeriveInput, Path};

fn clone_impl(
    input: &DeriveInput,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    unit: &syn::Field,
    t: &syn::TypeParam,
) -> TokenStream {
    let name = &input.ident;
    let mut generics = input.generics.clone();
    generics
        .where_clause
        .get_or_insert(parse_quote!(where))
        .predicates
        .push(parse_quote!(#t: Clone));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut body = quote! {};
    for field in fields.iter().filter(|f| f.ident != unit.ident) {
        let name = field.ident.as_ref().unwrap();
        body = quote! {
            #body
            #name: self.#name.clone(),
        }
    }

    let unit_name = unit.ident.as_ref().unwrap();
    quote! {
        impl #impl_generics Clone for #name #ty_generics #where_clause {
            fn clone(&self) -> Self {
                Self {
                    #body
                    #unit_name: PhantomData,
                }
            }
        }
    }
}

pub fn derive(input: DeriveInput) -> TokenStream {
    let s = match input.data {
        syn::Data::Struct(ref s) => s,
        _ => panic!("Need to derive this on a struct"),
    };

    let fields = match s.fields {
        syn::Fields::Named(ref named) => &named.named,
        _ => panic!("Need to use named fields"),
    };

    assert!(!fields.is_empty());

    let unit_field = fields.last().unwrap();
    assert_eq!(
        unit_field.value().ident.as_ref().unwrap().to_string(),
        "_unit",
        "You need to have a _unit field to derive this trait",
    );

    let type_param =
        input.generics.type_params().next().cloned().expect("Need a T");

    clone_impl(&input, fields, unit_field.value(), &type_param)
}
