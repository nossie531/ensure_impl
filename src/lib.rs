//! Trait implementation assertion.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;

/// Trait implementation assertion attribute.
///
/// Adding this attribute to an implementation confirms
/// that the implementation already exists, otherwise
/// compile error will occur.
///
/// # Examples
///
/// ```
/// # use ensure_impl::prelude::*;
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
pub use ensure_impl_macro::ensure_impl;

#[doc(hidden)]
#[path = "../tests_compile_fail/mod.rs"]
mod tests_compile_fail;
