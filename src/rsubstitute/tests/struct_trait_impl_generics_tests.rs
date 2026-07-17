use rsubstitute::*;

trait Trait<T> {
    fn work(&self, t: T) -> T;
}

// mocked_base! {
//     trait Struct;
// 
//     impl Struct {
//         pub fn_info new() -> Self {
//             Self
//         }
//     }
// 
//     impl<T> Trait<T> for Struct {
//         fn_info work(&self, t: T) -> T {
//             t
//         }
//     }
// }

mod tests {
    use super::*;

    #[test]
    fn compile() {}
}
