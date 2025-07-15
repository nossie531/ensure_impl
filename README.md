ensure_impl
===

Trait implementation assertion.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides an attribute macro to check the implementation of traits.
Primary intended use is blanket implementation checking.

The notation is just adding `#[ensure_impl]` to usual trait implementation.
In usual trait implementation, duplication with existing implementations
are checked, and if there are any, compile error occurs. On the other hand,
if the attribute macro is added, the behavior is reversed, and if there is
no duplication, compile error occurs.

## Background

Blanket implementation is great feature.

However, if its boundary is complex, in code reading, we humans can't judge
at a glance whether the type fits the blanket or not. Moreover, even if the
boundary is complex, the conditions may be loose. In such cases, there may be
unintended implementations. So, later developer can't judge them.

All of these problems come from lack of explicit implementation checks.

## Examples

```rust
use ensure_impl::prelude::*;

trait SomeTrait {}
trait Bound1 {}
trait Bound2 {}
trait Bound3 {}
impl<T> SomeTrait for T
where T: Bound1 + Bound2 + Bound3 {}

struct MyType();

#[ensure_impl]
impl SomeTrait for MyType {}
impl Bound1 for MyType {}
impl Bound2 for MyType {}
impl Bound3 for MyType {}
```

## Under the hood

For example, following code is ...

```rust
use ensure_impl::prelude::*;

trait MyTrait<T> {}
struct MyType<T>(T);
impl<T1, T2> MyTrait<T1> for MyType<T2> {}

#[ensure_impl]
impl<T1, T2> MyTrait<T1> for MyType<T2> {}
```

... expanded like this.

```rust
use ensure_impl::prelude::*;

trait MyTrait<T> {}
struct MyType<T>(T);
impl<T1, T2> MyTrait<T1> for MyType<T2> {}

const _: fn() = || {
    fn _reserve_params<T1, T2>() {
        _ensure::<T1, T2, MyType<T2>>();
    }
    fn _ensure<T1, T2, T: MyTrait<T1>>() {}
};
```

## Versions

See [CHANGELOG](CHANGELOG.md).
