//! Tests that should be compile errors.

/// Argument must be empty.
///
/// ```compile_fail
/// # use ensure_impl::prelude::*;
/// trait MyTrait {}
/// struct MyType();
/// impl<T> MyTrait for T {}
/// #[ensure_impl(42)]
/// impl MyTrait for i32 {}
/// ```
fn _arg_must_be_empty() {
    // nop.
}

/// Target must be `impl` block.
///
/// ```compile_fail
/// # use ensure_impl::prelude::*;
/// #[ensure_impl]
/// fn test() {}
/// ```
fn _target_must_be_impl() {
    // nop.
}

/// Target must be `impl` block with `for`.
///
/// ```compile_fail
/// # use ensure_impl::prelude::*;
/// struct MyType();
/// #[ensure_impl]
/// impl MyType {}
/// ```
fn _target_must_be_impl_with_for() {
    // nop.
}

/// Target must be safe `impl`.
///
/// ```compile_fail
/// # use ensure_impl::prelude::*;
/// trait MyTrait<T> {}
/// struct MyType();
/// impl<T> MyTrait<T> for MyType {}
/// #[ensure_impl]
/// unsafe impl<T> MyTrait<T> for MyType {}
/// ```
fn _target_must_be_safe_impl() {
    // nop.
}

/// Target must be positive `impl`.
///
/// ```compile_fail
/// # use ensure_impl::prelude::*;
/// trait MyTrait<T> {}
/// struct MyType();
/// impl<T> MyTrait<T> for MyType {}
/// #[ensure_impl]
/// impl<T> !MyTrait<T> for MyType {}
/// ```
fn _target_must_be_positive_impl() {
    // nop.
}

/// Target `impl` must be empty.
///
/// ```compile_fail
/// use crate::prelude::*;
/// trait MyTrait<T> {}
/// struct MyType();
/// impl<T> MyTrait<T> for MyType {}
/// #[ensure_impl]
/// impl<T> MyTrait<T> for MyType {
///     fn method() {}
/// }
/// ```
fn _target_impl_must_be_empty() {
    // nop.
}
