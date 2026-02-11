use rsubstitute::macros::mock;

#[mock]
trait Trait<'a, 'b> {
    fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn compile() {}

    #[test]
    fn flex() {
        let trait_mock = TraitMock::new();

        let v1 = 11;
        let r1 = &&&&v1;
        {
            let v2 = 23;
            let r2 = &&&&v2;

            trait_mock.setup.accept_ref(r2).returns(r1);

            let r = trait_mock.accept_ref(r2);
            assert_eq!(r1, r);

            trait_mock
                .received
                .accept_ref(r2, Times::Once)
                .no_other_calls();
        }
    }
}
