use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn work(&self, v1: i32, v2: i32, v3: i32, v4: i32) -> i32;
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use rsubstitute::Arg;
    use rsubstitute::assertions::assert_panics;

    #[test]
    fn work_PanicsOk() {
        // Arrange
        let mock = TraitMock::new();
        mock.setup
            .work(Arg::Any, Arg::Any, Arg::Any, Arg::Is(|_| false))
            .returns(1)
            .work(Arg::Any, Arg::Any, Arg::Is(|_| false), Arg::Is(|_| false))
            .returns(1)
            .work(
                Arg::Any,
                Arg::Is(|_| false),
                Arg::Is(|_| false),
                Arg::Is(|_| false),
            )
            .returns(1)
            .work(
                Arg::Is(|_| false),
                Arg::Is(|_| false),
                Arg::Is(|_| false),
                Arg::Is(|_| false),
            )
            .returns(1);

        // Act
        // Assert
        assert_panics(
            || mock.work(10, 20, 30, 40),
            r"Mock wasn't configured to handle following call:
	work(10, 20, 30, 40)
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. work(10, 20, 30, *40*)
	2. work(10, 20, *30*, *40*)
	3. work(10, *20*, *30*, *40*)
	4. work(*10*, *20*, *30*, *40*)",
        );
    }
}
