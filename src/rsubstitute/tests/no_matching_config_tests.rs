use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn work(&self, v1: i32, v2: i32, v3: i32, v4: i32) -> i32;
}

#[mock]
fn work(v1: i32, v2: i32, v3: i32, v4: i32) -> i32 {
    1
}

mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use rsubstitute::Arg;
    use rsubstitute::assertions::assert_panics;

    #[test]
    fn trait_work_PanicsOk() {
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

    #[test]
    fn fn_work_PanicsOk() {
        // Arrange
        work::setup(Arg::Any, Arg::Any, Arg::Any, Arg::Is(|_| false))
            .returns(1)
            .setup(Arg::Any, Arg::Any, Arg::Is(|_| false), Arg::Is(|_| false))
            .returns(1)
            .setup(
                Arg::Any,
                Arg::Is(|_| false),
                Arg::Is(|_| false),
                Arg::Is(|_| false),
            )
            .returns(1)
            .setup(
                Arg::Is(|_| false),
                Arg::Is(|_| false),
                Arg::Is(|_| false),
                Arg::Is(|_| false),
            )
            .returns(1);

        // Act
        // Assert
        assert_panics(
            || work(10, 20, 30, 40),
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
