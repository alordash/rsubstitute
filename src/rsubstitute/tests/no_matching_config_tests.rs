use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn work(&self, v1: i32, v2: i32, v3: i32, v4: i32) -> i32;
}

#[mock]
fn work(_v1: i32, _v2: i32, _v3: i32, _v4: i32) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    use rsubstitute::Arg;
    use not_enough_asserts::panics::*;

    #[test]
    fn trait_work_Panics() {
        // Arrange
        let mock = TraitMock::new();
        let (v1, v2, v3, v4) = (10, 20, 30, 40);
        mock.setup()
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
        let actual_error_msg = record_panic(|| mock.work(v1, v2, v3, v4));

        // Assert
        let expected_error_msg = format!("Mock wasn't configured to handle following call:
	work({v1}, {v2}, {v3}, {v4})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 3/4 arguments: work({v1}, {v2}, {v3}, *{v4}*)
	2. Matched 2/4 arguments: work({v1}, {v2}, *{v3}*, *{v4}*)
	3. Matched 1/4 arguments: work({v1}, *{v2}*, *{v3}*, *{v4}*)
	4. Matched 0/4 arguments: work(*{v1}*, *{v2}*, *{v3}*, *{v4}*)");
        assert_eq!(expected_error_msg, actual_error_msg);
    }

    #[test]
    fn fn_work_Panics() {
        // Arrange
        let (v1, v2, v3, v4) = (10, 20, 30, 40);
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
        let actual_error_msg = record_panic(|| work(v1, v2, v3, v4));

        // Assert
        let expected_error_msg = format!("Mock wasn't configured to handle following call:
	work({v1}, {v2}, {v3}, {v4})
List of existing configuration ordered by number of correctly matched arguments (non-matching arguments indicated with '*' characters):
	1. Matched 3/4 arguments: work({v1}, {v2}, {v3}, *{v4}*)
	2. Matched 2/4 arguments: work({v1}, {v2}, *{v3}*, *{v4}*)
	3. Matched 1/4 arguments: work({v1}, *{v2}*, *{v3}*, *{v4}*)
	4. Matched 0/4 arguments: work(*{v1}*, *{v2}*, *{v3}*, *{v4}*)");
        assert_eq!(expected_error_msg, actual_error_msg);
    }
}
