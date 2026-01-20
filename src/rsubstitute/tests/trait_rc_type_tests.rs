#![allow(unused_variables)]
#![allow(non_snake_case)]

use rsubstitute_proc_macro::mock;
use std::rc::Rc;

#[mock]
trait Trait {
    fn accept_rc(&self, r: Rc<i32>);

    fn return_rc(&self) -> Rc<i32>;

    // TODO - support various lifetimes via generics
    // TODO - make test that returns same lifetime as the one accepted
    fn accept_rc_return_rc(&self, r: Rc<i32>) -> Rc<i32>;

    fn accept_two_rcs(&self, r1: Rc<i32>, r2: Rc<f32>);

    // TODO - support various lifetimes via generics
    // fn accept_two_rcs_with_different_lifetimes<'a, 'b>(&self, r1: &'a i32, r2: &'b f32);

    // TODO - make test that returns same lifetime as the one accepted
    fn accept_two_rcs_return_rc(&self, r1: Rc<i32>, r2: Rc<f32>) -> Rc<String>;
}

mod accept_rc_tests {
    use super::*;
    use rsubstitute::assertions::assert_panics;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_rc_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r = Rc::new(1);

        // Act
        mock.accept_rc(r.clone());

        // Assert
        mock.received.accept_rc(r, Times::Once).only();
    }

    #[test]
    fn accept_rc_PanicsOk() {
        // Arrange
        let mock = TraitMock::new();
        let r = Rc::new(11);
        let r_ptr = Rc::as_ptr(&r);

        // Act
        mock.accept_rc(r.clone());

        // Assert
        assert_panics(
            || mock.received.accept_rc(Arg::Any, Times::Never),
            format!(
                r"Expected to never receive a call matching:
	accept_rc((alloc::rc::Rc<i32>): any)
Actually received 1 matching call:
	accept_rc({r})
Received no non-matching calls"
            ),
        );

        assert_panics(
            || mock.received.accept_rc(Arg::Any, Times::Exactly(3)),
            format!(
                r"Expected to receive a call 3 times matching:
	accept_rc((alloc::rc::Rc<i32>): any)
Actually received 1 matching call:
	accept_rc({r})
Received no non-matching calls"
            ),
        );

        let invalid_r = Rc::new(22);
        let invalid_r_ptr = Rc::as_ptr(&invalid_r);
        assert_panics(
            || mock.received.accept_rc(invalid_r.clone(), Times::Once),
            format!(
                r"Expected to receive a call exactly once matching:
	accept_rc((alloc::rc::Rc<i32>): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_rc(*{r}*)
	1. r (alloc::rc::Rc<i32>):
		Expected Rc (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual Rc   (ptr: {r_ptr:?}): {r}"
            ),
        )
    }
}

mod return_rc_tests {
    use super::*;

    #[test]
    fn return_rc_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r = Rc::new(10);
        mock.setup.return_rc().returns(r.clone());

        // Act
        let actual_r = mock.return_rc();

        // Assert
        assert_eq!(r, actual_r);
    }
}

mod accept_rc_return_rc_tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_rc_return_rc_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let accepted_r = Rc::new(10);
        let returned_r = Rc::new(20);
        mock.setup
            .accept_rc_return_rc(accepted_r.clone())
            .returns(returned_r.clone());

        // Act
        let actual_returned_r = mock.accept_rc_return_rc(accepted_r.clone());

        // Assert
        assert_eq!(returned_r, actual_returned_r);

        mock.received
            .accept_rc_return_rc(accepted_r.clone(), Times::Once)
            .accept_rc_return_rc(Arg::NotEq(accepted_r), Times::Never)
            .only();
    }
}

mod accept_two_rcs_tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    fn accept_two_rcs_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r1 = Rc::new(10);
        let r2 = Rc::new(20.2);

        // Act
        mock.accept_two_rcs(r1.clone(), r2.clone());

        // Assert
        mock.received
            .accept_two_rcs(r1.clone(), r2.clone(), Times::Once)
            .accept_two_rcs(Arg::NotEq(r1), Arg::NotEq(r2), Times::Never)
            .only();
    }
}

mod accept_two_rcs_return_rc_tests {
    use super::*;
    use rsubstitute_core::Times;

    #[test]
    fn accept_two_rcs_return_rc_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let r1 = Rc::new(10);
        let r2 = Rc::new(20.2);
        let returned_r = Rc::new(String::from("veridis quo"));
        mock.setup
            .accept_two_rcs_return_rc(r1.clone(), r2.clone())
            .returns(returned_r.clone());

        // Act
        let actual_returned_r = mock.accept_two_rcs_return_rc(r1.clone(), r2.clone());

        // Assert
        assert_eq!(returned_r, actual_returned_r);

        mock.received
            .accept_two_rcs_return_rc(r1, r2, Times::Once)
            .only();
    }
}
