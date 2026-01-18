use rsubstitute_proc_macro::mock;

#[mock]
trait Trait {
    fn accept_ref(&self, r: &i32);

    fn return_ref(&self) -> &'static i32;

    // TODO - support various lifetimes via generics
    // TODO - make test that returns same lifetime as the one accepted
    fn accept_ref_return_ref(&self, r: &i32) -> &'static i32;

    fn accept_two_refs(&self, r1: &i32, r2: &f32);

    // TODO - support various lifetimes via generics
    // fn accept_two_refs_with_different_lifetimes<'a, 'b>(&self, r1: &'a i32, r2: &'b f32);

    // TODO - make test that returns same lifetime as the one accepted
    fn accept_two_refs_return_ref(&self, r1: &i32, r2: &f32) -> &'static str;
}

mod accept_ref_tests {
    use super::*;
    use rsubstitute::assertions::assert_panics;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_ref_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r = &1;

        // Act
        mock.accept_ref(r);

        // Assert
        mock.received.accept_ref(r, Times::Once);
    }

    #[test]
    fn accept_rc_PanicsOk() {
        // Arrange
        let mock = TraitMock::new();
        let r = &11;
        let r_ptr = std::ptr::from_ref(r);

        // Act
        mock.accept_ref(r);

        // Assert
        assert_panics(
            || mock.received.accept_ref(Arg::Any, Times::Never),
            format!(
                r"Expected to never receive a call matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
            ),
        );

        assert_panics(
            || mock.received.accept_ref(Arg::Any, Times::Exactly(3)),
            format!(
                r"Expected to receive a call 3 times matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
            ),
        );

        let invalid_r = &22;
        let invalid_r_ptr = std::ptr::from_ref(invalid_r);
        assert_panics(
            || mock.received.accept_ref(invalid_r, Times::Once),
            format!(
                r"Expected to receive a call exactly once matching:
	accept_ref((&i32): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_ref(*{r}*)
	1. r (&i32):
		Expected reference (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual reference   (ptr: {r_ptr:?}): {r}"
            ),
        )
    }
}

mod return_ref_tests {
    use super::*;

    #[test]
    fn return_ref_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r = Box::leak(Box::new(11));
        mock.setup.return_ref().returns(r);

        // Act
        let actual_r = mock.return_ref();

        // Assert
        assert_eq!(r, actual_r);
    }
}

mod accept_ref_return_ref_tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_ref_return_ref_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let accepted_r = &10;
        let returned_r = &20;
        mock.setup
            .accept_ref_return_ref(accepted_r)
            .returns(returned_r);

        // Act
        let actual_returned_r = mock.accept_ref_return_ref(accepted_r);

        // Assert
        assert_eq!(returned_r, actual_returned_r);

        mock.received
            .accept_ref_return_ref(accepted_r, Times::Once)
            .accept_ref_return_ref(Arg::NotEq(accepted_r), Times::Never);
    }
}

mod accept_two_refs_tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_two_refs_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r1 = &10;
        let r2 = &20.2;

        // Act
        mock.accept_two_refs(r1, r2);

        // Assert
        mock.received
            .accept_two_refs(r1, r2, Times::Once)
            .accept_two_refs(
                // TODO - maybe add received_only ?
                Arg::NotEq(r1),
                Arg::NotEq(r2),
                Times::Never,
            );
    }
}

mod accept_two_refs_return_ref_tests {
    use super::*;
    use rsubstitute_core::Times;

    #[test]
    fn accept_two_refs_return_ref_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r1 = &10;
        let r2 = &20.2;
        let returned_r = "veridis quo";
        mock.setup
            .accept_two_refs_return_ref(r1, r2)
            .returns(returned_r);

        // Act
        let actual_returned_r = mock.accept_two_refs_return_ref(r1, r2);

        // Assert
        assert_eq!(returned_r, actual_returned_r);

        mock.received
            .accept_two_refs_return_ref(r1, r2, Times::Once);
    }
}
