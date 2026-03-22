#![feature(rwlock_downgrade)]

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
    use std::sync::*;

    // Used to run tests sequentially, otherwise `write_config` may cause deadlock
    // since `std::sync::RwLock` does not have priority policy for read and write locks.
    static TESTS_SYNCER: LazyLock<Mutex<()>> = LazyLock::new(|| Default::default());

    fn sync_test<'a>() -> MutexGuard<'a, ()> {
        TESTS_SYNCER.lock().expect("Unable to lock `TESTS_SYNCER`.")
    }

    mod default {
        use super::*;
        #[test]
        fn CallsCountLessThanLimit_PrintsAll() {
            let _lock = sync_test();

            // Arrange
            let mock = TraitMock::new();
            let max_invalid_calls_listed_count = 4;
            let calls_count = max_invalid_calls_listed_count - 1;

            let mut write_config = write_config();
            write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
            let _read_config_lock = RwLockWriteGuard::downgrade(write_config);

            let unexpected_v = 10;
            let expected_v = 20;

            // Act
            for _ in 0..calls_count {
                mock.work(unexpected_v);
            }
            let actual_error_msg = record_panic(|| mock.received.work(expected_v, Times::Once));

            // Assert
            let calls_error_msgs = format!(
                "
Trait::work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
            )
            .repeat(calls_count);
            let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (non-matching arguments indicated with '*' characters):{calls_error_msgs}"
            );
            assert_eq!(expected_error_msg, actual_error_msg);
            println!("First read");
        }

        #[test]
        fn CallsCountMoreThanLimit_PrintsTrimmed() {
            let _lock = sync_test();

            // Arrange
            let mock = TraitMock::new();
            let max_invalid_calls_listed_count = 4;
            let calls_count = max_invalid_calls_listed_count + 1;

            let mut write_config = write_config();
            write_config.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
            let _read_config_lock = RwLockWriteGuard::downgrade(write_config);

            let unexpected_v = 10;
            let expected_v = 20;

            // Act
            for _ in 0..calls_count {
                mock.work(unexpected_v);
            }
            let actual_error_msg = record_panic(|| mock.received.work(expected_v, Times::Once));

            // Assert
            let calls_error_msgs = format!(
                "
Trait::work(*{unexpected_v}*)
	1. v (i32):
		Expected: {expected_v}
		Actual:   {unexpected_v}"
            )
            .repeat(max_invalid_calls_listed_count);
            let expected_error_msg = format!(
                "Expected to receive a call exactly once matching:
	Trait::work((i32): equal to {expected_v})
Actually received no matching calls
Received {calls_count} non-matching calls (listing only first {max_invalid_calls_listed_count}) (non-matching arguments indicated with '*' characters):{calls_error_msgs}"
            );
            assert_eq!(expected_error_msg, actual_error_msg);
            println!("Second read");
        }
    }
}
