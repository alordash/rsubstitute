use rsubstitute::prelude::*;

// TODO - write in docs limitation: if mocking `base`, then all args must be cloneable
trait Trait<T> {
    fn work(&self, t: T) -> T;
}

// mocked_base! {
//     struct Struct;
//
//     impl Struct {
//         pub fn new() -> Self {
//             Self
//         }
//     }
//
//     impl<T: Clone> Trait<T> for Struct {
//         fn work(&self, t: T) -> T {
//             t
//         }
//     }
// }

mod tests {
    #[test]
    fn compile() {}
}
