use rsubstitute::macros::mock;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

#[mock(base)]
trait Trait<'a, 'b> {
    fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;

    fn return_mut_ref(&self) -> &'a mut i32;

    fn return_mut_ref_with_base(&self) -> &'a mut i32 {
        todo!()
    }

    fn fooo(&mut self, Foo { mut number }: Foo) {
        println!("number: {number:?}")
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn accept_ref_Ok() {
        // Arrange
        let v = &&&&32;
        let r = &&&&8;

        let trait_mock = TraitMock::new();
        trait_mock.setup.accept_ref(v).returns(r);

        // Act
        let actual_r = trait_mock.accept_ref(v);

        // Assert
        assert_eq!(r, actual_r);

        trait_mock
            .received
            .accept_ref(v, Times::Once)
            .no_other_calls();
    }

    #[test]
    fn flex() {
        let v1 = 11;
        let r1 = &&&&v1;
        {
            let v2 = 23;
            let r2 = &&&&v2;

            let trait_mock = TraitMock::new();
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
