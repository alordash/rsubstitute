use rsubstitute::prelude::*;

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
//     impl<T> Trait<T> for Struct {
//         fn work(&self, t: T) -> T {
//             t
//         }
//     }
// }

mod tests {
    use super::*;

    #[test]
    fn compile() {}
}
