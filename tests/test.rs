#![allow(dead_code)]

mod trait_has_generics_type {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    struct MyType();
    impl<T> MyTrait<T> for MyType {}

    #[ensure_impl]
    impl<T> MyTrait<T> for MyType {}
    #[ensure_impl]
    impl MyTrait<i32> for MyType {}
}

mod trait_has_generics_type_with_bound {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    struct MyType();
    trait Bound {}
    impl Bound for i32 {}
    impl<T> MyTrait<T> for MyType where T: Bound {}

    #[ensure_impl]
    impl<T> MyTrait<T> for MyType where T: Bound {}
    #[ensure_impl]
    impl<T: Bound> MyTrait<T> for MyType {}
    #[ensure_impl]
    impl MyTrait<i32> for MyType {}
}

mod trait_has_generics_lifetime {
    use ensure_impl::prelude::*;
    trait MyTrait<'a> {}
    struct MyType();
    impl<'a> MyTrait<'a> for MyType {}

    #[ensure_impl]
    impl<'a> MyTrait<'a> for MyType {}
    #[ensure_impl]
    impl MyTrait<'static> for MyType {}
}

mod trait_has_generics_lifetime_with_bound {
    use ensure_impl::prelude::*;
    trait MyTrait<'a, 'b> {}
    struct MyType();
    impl<'a, 'b> MyTrait<'a, 'b> for MyType where 'a: 'b {}

    #[ensure_impl]
    impl<'a, 'b> MyTrait<'a, 'b> for MyType where 'a: 'b {}
    #[ensure_impl]
    impl<'a: 'b, 'b> MyTrait<'a, 'b> for MyType {}
    #[ensure_impl]
    impl MyTrait<'static, 'static> for MyType {}
}

mod trait_has_generics_const {
    use ensure_impl::prelude::*;
    trait MyTrait<const N: usize = 1> {}
    struct MyType();
    impl<const N: usize> MyTrait<N> for MyType {}

    #[ensure_impl]
    impl<const N: usize> MyTrait<N> for MyType {}
    #[ensure_impl]
    impl MyTrait<1> for MyType {}
    #[ensure_impl]
    impl MyTrait for MyType {}
}

mod type_is_generics_type {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    impl<T> MyTrait for T {}

    #[ensure_impl]
    impl<T> MyTrait for T {}
    #[ensure_impl]
    impl MyTrait for i32 {}
}

mod type_is_generics_type_with_bound {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    trait Bound {}
    impl Bound for i32 {}
    impl<T> MyTrait for T where T: Bound {}

    #[ensure_impl]
    impl<T> MyTrait for T where T: Bound {}
    #[ensure_impl]
    impl<T: Bound> MyTrait for T {}
    #[ensure_impl]
    impl MyTrait for i32 {}
}

mod type_has_generics_type {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<T>(T);
    impl<T> MyTrait for MyType<T> {}

    #[ensure_impl]
    impl<T> MyTrait for MyType<T> {}
    #[ensure_impl]
    impl MyTrait for MyType<i32> {}
}

mod type_has_generics_type_with_bound {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<T>(T);
    trait Bound {}
    impl Bound for i32 {}
    impl<T> MyTrait for MyType<T> where T: Bound {}

    #[ensure_impl]
    impl<T> MyTrait for MyType<T> where T: Bound {}
    #[ensure_impl]
    impl<T: Bound> MyTrait for MyType<T> {}
    #[ensure_impl]
    impl MyTrait for MyType<i32> {}
}

mod type_has_generics_lifetime {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<'a>(&'a i32);
    impl<'a> MyTrait for MyType<'a> {}

    #[ensure_impl]
    impl<'a> MyTrait for MyType<'a> {}
    #[ensure_impl]
    impl MyTrait for MyType<'static> {}
}

mod type_has_generics_lifetime_with_bound {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<'a, 'b>(&'a i32, &'b i32);
    impl<'a, 'b> MyTrait for MyType<'a, 'b> where 'a: 'b {}

    #[ensure_impl]
    impl<'a, 'b> MyTrait for MyType<'a, 'b> where 'a: 'b {}
    #[ensure_impl]
    impl<'a: 'b, 'b> MyTrait for MyType<'a, 'b> {}
    #[ensure_impl]
    impl MyTrait for MyType<'static, 'static> {}
}

mod type_has_generics_const {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<const N: usize = 1>();
    impl<const N: usize> MyTrait for MyType<N> {}

    #[ensure_impl]
    impl<const N: usize> MyTrait for MyType<N> {}
    #[ensure_impl]
    impl MyTrait for MyType<1> {}
    #[ensure_impl]
    impl MyTrait for MyType {}
}

mod multi_generics_at_trait_param_and_type {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    impl<T1, T2> MyTrait<T1> for T2 {}

    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for T2 {}
    #[ensure_impl]
    impl<T> MyTrait<T> for i32 {}
    #[ensure_impl]
    impl<T> MyTrait<i32> for T {}
}

mod multi_generics_at_trait_param_and_type_param {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    struct MyType<T>(T);
    impl<T1, T2> MyTrait<T1> for MyType<T2> {}

    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for MyType<T2> {}
    #[ensure_impl]
    impl<T> MyTrait<T> for MyType<i32> {}
    #[ensure_impl]
    impl<T> MyTrait<i32> for MyType<T> {}
}

mod bound_to_self {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<T>(T);
    trait Bound {}
    impl<T> MyTrait for MyType<T> where Self: Bound {}

    #[ensure_impl]
    impl<T> MyTrait for MyType<T> where Self: Bound {}
    #[ensure_impl]
    impl<T> MyTrait for MyType<T> where MyType<T>: Bound {}
}

mod bound_to_self_with_generics {
    use ensure_impl::prelude::*;
    trait MyTrait {}
    struct MyType<T>(T);
    trait Bound {}
    impl<T> Bound for MyType<T> {}
    impl<T> MyTrait for MyType<T> where Self: Bound {}

    #[ensure_impl]
    impl<T> MyTrait for MyType<T> where Self: Bound {}
    #[ensure_impl]
    impl<T> MyTrait for MyType<T> where MyType<T>: Bound {}
    #[ensure_impl]
    impl MyTrait for MyType<i32> {}
}

mod bound_to_self_with_associated_type {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    struct MyType<T>(T);
    trait Bound1 {
        type Assoc;
    }
    trait Bound2 {}
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        Self: Bound1,
        <Self as Bound1>::Assoc: Bound2,
    {
    }

    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        Self: Bound1,
        <Self as Bound1>::Assoc: Bound2,
    {
    }
    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        MyType<T2>: Bound1,
        <MyType<T2> as Bound1>::Assoc: Bound2,
    {
    }
}

mod bound_span_trait_and_type_with_generics {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    trait Bound1<T> {}
    trait Bound2<T> {}
    impl<T1, T2> MyTrait<T1> for T2
    where
        T1: Bound1<T2>,
        T2: Bound2<T1>,
    {
    }

    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for T2
    where
        T1: Bound1<T2>,
        T2: Bound2<T1>,
    {
    }
    #[ensure_impl]
    impl<T1: Bound1<T2>, T2: Bound2<T1>> MyTrait<T1> for T2 {}
}

mod bound_to_associated_type_in_trait {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    struct MyType<T>(T);
    trait Bound1 {
        type Assoc;
    }
    trait Bound2 {}
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        T1: Bound1,
        <T1 as Bound1>::Assoc: Bound2,
    {
    }

    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        T1: Bound1,
        <T1 as Bound1>::Assoc: Bound2,
    {
    }
}

mod bound_to_associated_type_in_type {
    use ensure_impl::prelude::*;
    trait MyTrait<T> {}
    struct MyType<T>(T);
    trait Bound1 {
        type Assoc;
    }
    trait Bound2 {}
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        T2: Bound1,
        <T2 as Bound1>::Assoc: Bound2,
    {
    }

    #[ensure_impl]
    impl<T1, T2> MyTrait<T1> for MyType<T2>
    where
        T2: Bound1,
        <T2 as Bound1>::Assoc: Bound2,
    {
    }
}
