//! Provider of [`translate_ensure_impl`].

use crate::util;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Error};

/// Translate `ensure_impl` attribute and its item.
pub fn translate_ensure_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_attr(attr);
    let impl_item = parse_impl_item(item);
    let errs = [attr.as_ref().err(), impl_item.as_ref().err()];
    let errs = errs.iter().filter_map(|x| *x).collect::<Vec<_>>();
    if !errs.is_empty() {
        let mut ret = TokenStream::new();
        ret.extend(errs.iter().map(|&x| x.clone()));
        return ret;
    }

    create_ensure_stmt(impl_item.as_ref().unwrap())
}

/// Parse attribute.
fn parse_attr(input: TokenStream) -> Result<(), TokenStream> {
    if !input.is_empty() {
        let err = Error::new_spanned(input, msg::ATTR_ARG_EMPTY);
        let err = err.into_compile_error();
        return Err(err);
    }

    Ok(())
}

/// Parse item.
fn parse_impl_item(input: TokenStream) -> Result<syn::ItemImpl, TokenStream> {
    let impl_item = match syn::parse2::<syn::Item>(input.clone()) {
        Ok(syn::Item::Impl(x)) => x,
        Err(_) => return Err(input),
        Ok(_) => return Err(util::err_tokens(&input, msg::IMPL_ONLY)),
    };

    if impl_item.unsafety.is_some() {
        let err_tokens = impl_item.unsafety.unwrap();
        return Err(util::err_tokens(&err_tokens, msg::UNSAFE_INCLUDED));
    }

    if impl_item.trait_.is_none() {
        let err_tokens = impl_item.impl_token;
        return Err(util::err_tokens(&err_tokens, msg::IMPL_FOR_ONLY));
    }

    if impl_item.trait_.as_ref().is_some_and(|x| x.0.is_some()) {
        let err_tokens = impl_item.trait_.unwrap().0.unwrap();
        return Err(util::err_tokens(&err_tokens, msg::NEG_IMPL_DETECTED));
    }

    Ok(impl_item)
}

/// Creates ensure statement.
fn create_ensure_stmt(impl_item: &syn::ItemImpl) -> TokenStream {
    let gen_params = impl_item.generics.params.iter().collect::<Vec<_>>();
    let trait_path = &(impl_item.trait_).as_ref().unwrap().1;
    let self_ty = impl_item.self_ty.as_ref();
    let where_clause = impl_item.generics.where_clause.as_ref();
    let where_clause = where_clause.map(|x| util::replace_self(x, self_ty));
    let target_param = util::unused_id(&impl_item.generics);
    let gen_args = util::type_and_const_idents(&impl_item.generics);

    quote! {
        const _: fn() = || {
            fn _reserve_params<#(#gen_params,)*>()
            #where_clause {
                _ensure::<#(#gen_args,)* #self_ty>();
            }

            fn _ensure<#(#gen_params,)* #target_param: #trait_path>() {}
        };
    }
}

/// Messages.
mod msg {
    #![allow(clippy::missing_docs_in_private_items)]

    pub const ATTR_ARG_EMPTY: &str = "`ensure_impl` argument must be empty";
    pub const IMPL_ONLY: &str = "`ensure_impl` can only be used on `impl`";
    pub const IMPL_FOR_ONLY: &str = "`ensure_impl` can only be used on `impl` with `for`";
    pub const UNSAFE_INCLUDED: &str = "`ensure_impl` does not support `unsafe`";
    pub const NEG_IMPL_DETECTED: &str = "`ensure_impl` does not support negative impl";
}
