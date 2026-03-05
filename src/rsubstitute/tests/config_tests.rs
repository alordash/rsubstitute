use rsubstitute::macros::mock;

#[mock]
trait Trait {
    fn work(&self, v: i32);
}

#[cfg(test)]
mod max_invalid_calls_listed_count_tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    mod default {
        use super::*;
        #[test]
        fn DefaultMaxInvalidCallsListedCount_LessThanLimit_PrintsAll() {
            // Arrange
            let mock = TraitMock::new();
            let calls_count = DEFAULT_CONFIG.max_invalid_calls_listed_count - 1;
            let unexpected_v = 10;
            let expected_v = 20;

            // Act
            for _ in 0..calls_count {
                mock.work(unexpected_v);
            }
            let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

            // Assert
            let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (non-matching arguments indicated with '*' characters):
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
            );
            assert_eq!(expected_error_msg, actual_error_msg);
        }

        #[test]
        fn DefaultMaxInvalidCallsListedCount_MoreThanLimit_PrintsTrimmed() {
            // Arrange
            let mock = TraitMock::new();
            let max_invalid_calls_listed_count = DEFAULT_CONFIG.max_invalid_calls_listed_count;
            let calls_count = max_invalid_calls_listed_count + 1;
            let unexpected_v = 10;
            let expected_v = 20;

            // Act
            for _ in 0..calls_count {
                mock.work(unexpected_v);
            }
            let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

            // Assert
            let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (listing only first {max_invalid_calls_listed_count}) (non-matching arguments indicated with '*' characters):
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
            );
            assert_eq!(expected_error_msg, actual_error_msg);
        }
    }

    #[test]
    fn set_max_invalid_calls_listed_count_Ok() {
        // Arrange
        let mock = TraitMock::new();
        let max_invalid_calls_listed_count = 3;
        let calls_count = max_invalid_calls_listed_count + 1;
        let unexpected_v = 10;
        let expected_v = 20;
        set_max_invalid_calls_listed_count(max_invalid_calls_listed_count);

        // Act
        for _ in 0..calls_count {
            mock.work(unexpected_v);
        }
        let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

        // Assert
        let expected_error_msg = format!(
            "Expected to receive a call exactly once matching:
	work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (listing only first {max_invalid_calls_listed_count}) (non-matching arguments indicated with '*' characters):
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
        );
        assert_eq!(expected_error_msg, actual_error_msg);
    }

    #[test]
    fn set_local_max_invalid_calls_listed_count_Ok() {
        // Arrange
        let old_max_invalid_calls_listed_count = 1;
        set_max_invalid_calls_listed_count(old_max_invalid_calls_listed_count);

        // Act
        {
            // Arrange
            let mock = TraitMock::new();
            let max_invalid_calls_listed_count = 3;
            let calls_count = max_invalid_calls_listed_count + 1;
            let unexpected_v = 10;
            let expected_v = 20;
            let _guard = set_local_max_invalid_calls_listed_count(max_invalid_calls_listed_count);

            // Act
            for _ in 0..calls_count {
                mock.work(unexpected_v);
            }
            let actual_error_msg = record_panic(|| mock.received().work(expected_v, Times::Once));

            // Assert
            let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (listing only first {max_invalid_calls_listed_count}) (non-matching arguments indicated with '*' characters):
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}
work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
            );
            assert_eq!(expected_error_msg, actual_error_msg);
        }

        // Assert
        let actual_max_invalid_calls_listed_count = get_max_invalid_calls_listed_count();
        assert_eq!(
            old_max_invalid_calls_listed_count,
            actual_max_invalid_calls_listed_count
        );
    }
}
