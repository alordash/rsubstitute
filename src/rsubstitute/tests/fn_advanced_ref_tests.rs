#![allow(unused)]
use rsubstitute::macros::mock;

#[mock]
fn accept_ref<'a>(r: &'a i32) -> &'a i32 {
    todo!()
}

// #[mock]
// fn accept_mut_ref<'a>() -> &'a mut i32 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn compile() {}

    // #[test]
    // fn accept_ref_test() {
    //     let v1 = 1;
    //     let v2 = 2;
    //     let r1 = &v1;
    //     {
    //         let r2 = &v2;
    //
    //         accept_ref::setup(r1).returns(r2);
    //
    //         accept_ref(r1);
    //
    //         accept_ref::received(r1, Times::Once).no_other_calls();
    //     }
    // }
}
