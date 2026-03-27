use rsubstitute::prelude::*;
use std::marker::PhantomData;

macro_rules! define_marker_traits {
    ($($names:ident),*) => { $(trait $names {} impl<T> $names for T {})* };
}

define_marker_traits!(M1, M2, M3, M4);



struct Lifetime<'x: 'a, 'a, 'b>(PhantomData<&'x &'a &'b ()>);
struct Type<T: M1>(PhantomData<T>) where T: M2;
struct Const<const N: usize>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn compile() {
//         // let mock = Struct::<i32, String>::new();
//         //
//         // Same::work(&mock, &3);
//         //
//         // mock.received.as_Same.work(&3, Times::Once);
//     }
// }
