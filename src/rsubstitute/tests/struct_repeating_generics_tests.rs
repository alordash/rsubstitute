use rsubstitute::prelude::*;
use std::marker::PhantomData;

macro_rules! define_marker_traits {
    ($($names:ident),*) => { trait $($name),+ {} impl<T> $($name),+ for T {} };
}

define_marker_traits!(A, B);

trait M1 {}
trait M2 {}
trait M3 {}
trait M4 {}
impl<T> M1 for T {}
impl<T> M2 for T {}

struct Lifetime<'x: 'a, 'a, 'b>(PhantomData<&'x ()>);
struct Type<T: M1>(PhantomData<T>);
struct Const<const N: usize>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile() {
        // let mock = Struct::<i32, String>::new();
        // 
        // Same::work(&mock, &3);
        // 
        // mock.received.as_Same.work(&3, Times::Once);
    }
}
