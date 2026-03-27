use rsubstitute::prelude::*;
use std::marker::PhantomData;

macro_rules! define_marker_traits {
    ($($names:ident),*) => { $(trait $names {} impl<T> $names for T {})* };
}

define_marker_traits!(M1, M2, M3, M4);

trait ILifetime<'x: 'c, 'a, 'b, 'c, 'd>
where
    'x: 'd,
{
    fn work(&self);
}

mocked_base! {
    struct Lifetime<'x: 'a, 'a, 'b, 'c, 'd>(PhantomData<&'x &'a &'b &'c &'d ()>)
    where
        'x: 'b;

    impl<'x: 'a + 'c, 'a, 'b, 'c, 'd> Lifetime<'x, 'a, 'b, 'c, 'd > where 'x: 'b + 'd  {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<'x: 'a + 'c, 'a, 'b, 'c, 'd> ILifetime<'x, 'a, 'b, 'c, 'd> for Lifetime<'x, 'a, 'b, 'c, 'd>
    where
        'x: 'b + 'd
    {
        fn work(&self) {
            unreachable!()
        }
    }
}

trait IType<T: M3>
where
    T: M4,
{
    fn work(&self);
}

mocked_base! {
    struct Type<T: M1>(PhantomData<T>)
    where
        T: M2;

    impl<T: M1> Type<T>
    where
        T: M2,
    {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }

    impl<T: M1 + M2> IType<T> for Type<T>
    where
        T: M3 + M4,
    {
        fn work(&self) {
            unreachable!()
        }
    }
}

trait IConst<const C: usize> {
    fn work(&self);
}

mocked_base! {
    struct Const<const C: usize>;

    impl<const C: usize> Const<C> {
        pub fn new() -> Self {
            Self
        }
    }

    impl<const C: usize> IConst<C> for Const<C> {
        fn work(&self) {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile() {}
}
