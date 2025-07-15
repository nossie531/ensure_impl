//! Procedual macros for crate `ensure_impl`.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use proc_macro as pm;
use proc_macro2::TokenStream;
use translate::translate_ensure_impl;

mod translate;
mod util;

#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn ensure_impl(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);
    let ret = translate_ensure_impl(attr, item);
    pm::TokenStream::from(ret)
}
