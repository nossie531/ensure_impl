//! Trait implementation assertion.
//!
//! *The author of this crate is not good at English.*
//! *Forgive me if the document is hard to read.*

#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub mod prelude;

pub use ensure_impl_macro::ensure_impl;

#[doc(hidden)]
#[path = "../tests_compile_fail/mod.rs"]
mod tests_compile_fail;
