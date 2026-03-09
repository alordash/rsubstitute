use rsubstitute::macros::*;
use std::sync::Arc;

mocked! {
    #[allow(unused)]
    struct Struct {
        pub number: i32
    }

    #[allow(unused)]
    impl Struct {
        pub fn new() -> Self {
            Self { number: 3 }
        }

        #[allow(unused)]
        pub(crate) fn accept_arc(&self, r: Arc<i32>) {
            todo!()
        }

        pub(crate) fn return_arc(&self) -> Arc<i32> {
            todo!()
        }

        pub(crate) fn accept_arc_return_arc(&self, r: Arc<i32>) -> Arc<i32> {
            todo!()
        }

        pub(crate) fn accept_two_arcs(&self, r1: Arc<i32>, r2: Arc<f32>) {
            todo!()
        }

        pub(crate) fn accept_two_arcs_return_arc(&self, r1: Arc<i32>, r2: Arc<f32>) -> Arc<String> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    mod accept_arc_tests {
        use super::*;

        #[test]
        fn accept_arc_Ok() {
            // Arrange
            let mock = StructMock::new();
            let r = Arc::new(1);

            // Act
            mock.accept_arc(r.clone());

            // Assert
            mock.received.accept_arc(r, Times::Once).no_other_calls();
        }

        #[test]
        fn accept_arc_Panics() {
            // Arrange
            let mock = StructMock::new();
            let r = Arc::new(11);
            let r_ptr = Arc::as_ptr(&r);

            // Act
            mock.accept_arc(r.clone());

            // Assert
            assert_panics(
                || mock.received.accept_arc(Arg::Any, Times::Never),
                format!(
                    "Expected to never receive a call matching:
	accept_arc((alloc::sync::Arc<i32>): any)
Actually received 1 matching call:
	accept_arc({r})
Received no non-matching calls"
                ),
            );

            assert_panics(
                || mock.received.accept_arc(Arg::Any, Times::Exactly(3)),
                format!(
                    "Expected to receive a call 3 times matching:
	accept_arc((alloc::sync::Arc<i32>): any)
Actually received 1 matching call:
	accept_arc({r})
Received no non-matching calls"
                ),
            );

            let invalid_r = Arc::new(22);
            let invalid_r_ptr = Arc::as_ptr(&invalid_r);
            assert_panics(
                || mock.received.accept_arc(invalid_r.clone(), Times::Once),
                format!(
                    "Expected to receive a call exactly once matching:
	accept_arc((alloc::sync::Arc<i32>): equal to {invalid_r})
Actually received no matching calls
Received 1 non-matching call (non-matching arguments indicated with '*' characters):
accept_arc(*{r}*)
	1. r (alloc::sync::Arc<i32>):
		Expected Arc (ptr: {invalid_r_ptr:?}): {invalid_r}
		Actual Arc   (ptr: {r_ptr:?}): {r}"
                ),
            )
        }
    }

    mod return_arc_tests {
        use super::*;

        #[test]
        fn return_arc_Ok() {
            // Arrange
            let mock = StructMock::new();
            let r = Arc::new(10);
            mock.setup.return_arc().returns(r.clone());

            // Act
            let actual_r = mock.return_arc();

            // Assert
            assert_eq!(r, actual_r);
        }
    }

    mod accept_arc_return_arc_tests {
        use super::*;

        #[test]
        fn accept_arc_return_arc_Ok() {
            // Arrange
            let mock = StructMock::new();
            let accepted_r = Arc::new(10);
            let returned_r = Arc::new(20);
            mock.setup
                .accept_arc_return_arc(accepted_r.clone())
                .returns(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_arc_return_arc(accepted_r.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received
                .accept_arc_return_arc(accepted_r.clone(), Times::Once)
                .accept_arc_return_arc(Arg::NotEq(accepted_r), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_arcs_tests {
        use super::*;

        #[test]
        fn accept_two_arcs_Ok() {
            // Arrange
            let mock = StructMock::new();
            let r1 = Arc::new(10);
            let r2 = Arc::new(20.2);

            // Act
            mock.accept_two_arcs(r1.clone(), r2.clone());

            // Assert
            mock.received
                .accept_two_arcs(r1.clone(), r2.clone(), Times::Once)
                .accept_two_arcs(Arg::NotEq(r1), Arg::NotEq(r2), Times::Never)
                .no_other_calls();
        }
    }

    mod accept_two_arcs_return_arc_tests {
        use super::*;

        #[test]
        fn accept_two_arcs_return_arc_Ok() {
            // Arrange
            let mock = StructMock::new();
            let r1 = Arc::new(10);
            let r2 = Arc::new(20.2);
            let returned_r = Arc::new(String::from("veridis quo"));
            mock.setup
                .accept_two_arcs_return_arc(r1.clone(), r2.clone())
                .returns(returned_r.clone());

            // Act
            let actual_returned_r = mock.accept_two_arcs_return_arc(r1.clone(), r2.clone());

            // Assert
            assert_eq!(returned_r, actual_returned_r);

            mock.received
                .accept_two_arcs_return_arc(r1, r2, Times::Once)
                .no_other_calls();
        }
    }
}
