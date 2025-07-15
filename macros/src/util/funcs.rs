//! Crate's utility.

use crate::util::SelfInWhereReplacer;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident};
use std::collections::BTreeSet;
use std::fmt::Display;
use std::iter;
use syn::Error;

/// Returns compile error tokens.
pub fn err_tokens<T, U>(tokens: &T, message: U) -> TokenStream
where
    T: ToTokens,
    U: Display,
{
    Error::new_spanned(tokens, message).into_compile_error()
}

/// Returns `where` clause without `Self` keyword.
pub fn replace_self(wc: &syn::WhereClause, alt: &syn::Type) -> syn::WhereClause {
    SelfInWhereReplacer::new(wc, alt).exec()
}

/// Returns type and const generics parameter `Ident` vector.
pub fn type_and_const_idents(generics: &syn::Generics) -> Vec<syn::Ident> {
    type_and_const_ids(generics)
        .map(|x| format_ident!("{x}"))
        .collect::<Vec<_>>()
}

/// Returns unused id.
pub fn unused_id(generics: &syn::Generics) -> syn::Ident {
    let ids = type_and_const_ids(generics).collect::<BTreeSet<_>>();
    let ret = candidates("T").find(|x| !ids.contains(x)).unwrap();
    return format_ident!("{ret}");

    fn candidates(candidate: &str) -> impl Iterator<Item = String> {
        (0..).map(|x| String::from_iter(iter::repeat_n("_", x)) + candidate)
    }
}

/// Returns type and const generics parameter IDs.
fn type_and_const_ids(generics: &syn::Generics) -> impl Iterator<Item = String> {
    generics.params.iter().filter_map(|x| match x {
        syn::GenericParam::Type(x) => Some(x.ident.to_string()),
        syn::GenericParam::Const(x) => Some(x.ident.to_string()),
        _ => None,
    })
}
