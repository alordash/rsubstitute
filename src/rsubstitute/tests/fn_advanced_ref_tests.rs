#![allow(unused)]
use rsubstitute::macros::mock;

// #[mock]
// fn accept_refs<'a>(r: &mut Vec<i32>) {
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

    #[test]
    fn payload() {
        // let mut v = Vec::new();
        // let r = &mut v;
        //
        // accept_refs::setup(r).does(|| {});
        //
        // accept_refs(r);
        //
        // accept_refs::received(r, Times::Once);
    }
}
