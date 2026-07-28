use rsubstitute::*;
use std::marker::PhantomData;

macro_rules! define_marker_traits {
    ($($names:ident),*) => { $(trait $names {} impl<T> $names for T {})* };
}

define_marker_traits!(M1, M2, M3, M4);

trait ILifetime<'x: 'c, 'a, 'b, 'c, 'd>
where
    'x: 'd,
{
}

trait IType<T: M3>
where
    T: M4,
{
}

trait IConst<const C: usize> {}

#[mock]
pub struct Lifetime<'x: 'a, 'a, 'b, 'c, 'd>(PhantomData<&'x &'a &'b &'c &'d ()>)
where
    'x: 'b;

#[mock(base)]
impl<'x: 'a + 'c, 'a, 'b, 'c, 'd> Lifetime<'x, 'a, 'b, 'c, 'd> where 'x: 'b + 'd {}

#[mock(base)]
impl<'x: 'a + 'c, 'a, 'b, 'c, 'd> ILifetime<'x, 'a, 'b, 'c, 'd> for Lifetime<'x, 'a, 'b, 'c, 'd> where
    'x: 'b + 'd
{
}

struct Type<T: M1>(PhantomData<T>)
where
    T: M2;
struct Const<const C: usize>;

#[cfg(test)]
mod tests {
    #[test]
    fn compile() {}
}
