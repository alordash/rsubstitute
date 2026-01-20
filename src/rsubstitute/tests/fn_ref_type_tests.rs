#![allow(unused_variables)]
#![allow(non_snake_case)]

use rsubstitute_proc_macro::mock;

#[mock]
fn accept_ref(r: &i32) {}

const BASE_RETURN_REF: &'static i32 = &1000;
#[mock]
fn return_ref() -> &'static i32 {
    BASE_RETURN_REF
}

// TODO - support various lifetimes via generics
// TODO - make test that returns same lifetime as the one accepted
const BASE_ACCEPT_REF_RETURN_REF: &'static i32 = &2000;
#[mock]
fn accept_ref_return_ref(r: &i32) -> &'static i32 {
    BASE_ACCEPT_REF_RETURN_REF
}

#[mock]
fn accept_two_refs(r1: &i32, r2: &f32) {}

// TODO - support various lifetimes via generics
// #[mock]
// fn accept_two_refs_with_different_lifetimes<'a, 'b>(r1: &'a i32, r2: &'b f32) {}

// TODO - make test that returns same lifetime as the one accepted
const ACCEPT_TWO_REFS_RETURN_REF: &'static str = "quo vadis";
#[mock]
fn accept_two_refs_return_ref(r1: &i32, r2: &f32) -> &'static str {
    ACCEPT_TWO_REFS_RETURN_REF
}

mod accept_ref_tests {
    use super::*;
    use rsubstitute::assertions::assert_panics;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_ref_Ok() {
        // Arrange
        let r = &1;

        // Act
        accept_ref(r);

        // Assert
        accept_ref::received(r, Times::Once).only();
    }

    #[test]
    fn accept_value_PanicsOk() {
        // Arrange
        let r = &11;
        let r_ptr = std::ptr::from_ref(r);

        // Act
        accept_ref(r);

        // Assert
        assert_panics(
            || accept_ref::received(Arg::Any, Times::Never),
            format!(
                r"Expected to never receive a call matching:
	accept_ref((&i32): any)
Actually received 1 matching call:
	accept_ref({r})
Received no non-matching calls"
            ),
        );

        assert_panics(
            || accept_ref::received(Arg::Any, Times::Exactly(3)),
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
            || accept_ref::received(invalid_r, Times::Once),
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
        let r = Box::leak(Box::new(11));
        return_ref::setup().returns(r);

        // Act
        let actual_r = return_ref();

        // Assert
        assert_eq!(r, actual_r);
    }

    #[test]
    fn return_ref_CallBase_Ok() {
        // Arrange
        return_ref::setup().call_base();

        // Act
        let actual_r = return_ref();

        // Assert
        assert_eq!(BASE_RETURN_REF, actual_r);
    }
}

mod accept_ref_return_ref_tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_ref_return_ref_Ok() {
        // Arrange
        let accepted_r = &10;
        let returned_r = &20;
        accept_ref_return_ref::setup(accepted_r).returns(returned_r);

        // Act
        let actual_returned_r = accept_ref_return_ref(accepted_r);

        // Assert
        assert_eq!(returned_r, actual_returned_r);

        accept_ref_return_ref::received(accepted_r, Times::Once)
            .received(Arg::NotEq(accepted_r), Times::Never)
            .only();
    }

    #[test]
    fn accept_ref_return_ref_CallBase_Ok() {
        // Arrange
        let accepted_r = &10;
        accept_ref_return_ref::setup(accepted_r).call_base();

        // Act
        let actual_returned_r = accept_ref_return_ref(accepted_r);

        // Assert
        assert_eq!(BASE_ACCEPT_REF_RETURN_REF, actual_returned_r);
        accept_ref_return_ref::received(accepted_r, Times::Once).only();
    }
}

mod accept_two_refs_tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_two_refs_Ok() {
        // Arrange
        let r1 = &10;
        let r2 = &20.2;

        // Act
        accept_two_refs(r1, r2);

        // Assert
        accept_two_refs::received(r1, r2, Times::Once)
            .received(Arg::NotEq(r1), Arg::NotEq(r2), Times::Never)
            .only();
    }
}

mod accept_two_refs_return_ref_tests {
    use super::*;
    use rsubstitute_core::Times;

    #[test]
    fn accept_two_refs_return_ref_Ok() {
        // Arrange
        let r1 = &10;
        let r2 = &20.2;
        let returned_r = "veridis quo";
        accept_two_refs_return_ref::setup(r1, r2).returns(returned_r);

        // Act
        let actual_returned_r = accept_two_refs_return_ref(r1, r2);

        // Assert
        assert_eq!(returned_r, actual_returned_r);

        accept_two_refs_return_ref::received(r1, r2, Times::Once).only();
    }

    #[test]
    fn accept_two_refs_return_ref_CallBase_Ok() {
        // Arrange
        let r1 = &10;
        let r2 = &20.2;
        accept_two_refs_return_ref::setup(r1, r2).call_base();

        // Act
        let actual_returned_r = accept_two_refs_return_ref(r1, r2);

        // Assert
        assert_eq!(ACCEPT_TWO_REFS_RETURN_REF, actual_returned_r);

        accept_two_refs_return_ref::received(r1, r2, Times::Once).only();
    }
}
