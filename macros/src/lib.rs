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

/// Trait implementation assertion attribute.
///
/// Adding this attribute to an implementation confirms
/// that the implementation already exists, otherwise
/// compile error will occur.
///
/// # Examples
///
/// ```
/// # use ensure_impl_macro::ensure_impl;
/// #
/// trait SomeTrait {}
/// trait Bound1 {}
/// trait Bound2 {}
/// trait Bound3 {}
/// impl<T> SomeTrait for T
/// where T: Bound1 + Bound2 + Bound3 {}
///
/// struct MyType();
///
/// #[ensure_impl]
/// impl SomeTrait for MyType {}
/// impl Bound1 for MyType {}
/// impl Bound2 for MyType {}
/// impl Bound3 for MyType {}
/// ```
#[allow(missing_docs)]
#[proc_macro_attribute]
pub fn ensure_impl(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let attr = TokenStream::from(attr);
    let item = TokenStream::from(item);
    let ret = translate_ensure_impl(attr, item);
    pm::TokenStream::from(ret)
}
