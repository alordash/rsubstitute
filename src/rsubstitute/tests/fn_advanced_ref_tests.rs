#![allow(unused)]
use rsubstitute::macros::mock;

#[mock]
fn accept_ref<'a>(r: &'a i32) -> &'a i32 {
    todo!()
}

fn check<'a>(r1: &'a i32, r2: &'a i32) -> &'a i32 {
    r1
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn compile() {
        let v1 = 1;
        let v2 = 2;
        let r1 = &v1;
        {
            let r2 = &v2;

            accept_ref::setup(r1).returns(r2);
            
            // check(r2, r1);

            accept_ref(r1);

            accept_ref::received(r1, Times::Once).no_other_calls();
        }
    }
}
