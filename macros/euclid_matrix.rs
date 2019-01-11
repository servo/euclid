/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use darling::util::IdentList;
use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn::{self, DeriveInput, Path};

fn derive_trait(
    input: &DeriveInput,
    trait_name: TokenStream,
    t: &syn::TypeParam,
    body: impl FnOnce() -> TokenStream,
) -> TokenStream {
    let struct_name = &input.ident;
    let mut generics = input.generics.clone();
    generics
        .where_clause
        .get_or_insert(parse_quote!(where))
        .predicates
        .push(parse_quote!(#t: #trait_name));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let body = body();
    quote! {
        impl #impl_generics #trait_name for #struct_name #ty_generics #where_clause {
            #body
        }
    }
}

fn clone_impl(
    input: &DeriveInput,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    unit: &syn::Field,
    t: &syn::TypeParam,
) -> TokenStream {
    derive_trait(input, quote! { Clone }, t, || {
        let body = fields.iter().fold(quote! {}, |body, field| {
            let name = field.ident.as_ref().unwrap();
            let expr = if field.ident == unit.ident {
                quote! { PhantomData }
            } else {
                quote! { self.#name.clone() }
            };

            quote! {
                #body
                #name: #expr,
            }
        });

        quote! {
            fn clone(&self) -> Self {
                Self {
                    #body
                }
            }
        }
    })
}

fn copy_impl(input: &DeriveInput, t: &syn::TypeParam) -> TokenStream {
    derive_trait(input, quote!{ Copy }, t, || quote! {})
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

    let clone = clone_impl(&input, fields, unit_field.value(), &type_param);
    let copy = copy_impl(&input, &type_param);

    quote! {
        #clone
        #copy
    }
}
