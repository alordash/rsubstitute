use crate::IStaticLocalKey;
use std::sync::{LazyLock, Mutex, MutexGuard};

pub const DEFAULT_CONFIG: Config = Config {
    max_invalid_calls_listed_count: 10,
};

thread_local! {
    pub(crate) static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| {
        Mutex::new(DEFAULT_CONFIG)
    });
}

pub struct Config {
    pub max_invalid_calls_listed_count: usize,
}

pub fn get_max_invalid_calls_listed_count() -> usize {
    lock_config().max_invalid_calls_listed_count
}

pub fn set_max_invalid_calls_listed_count(max_invalid_calls_listed_count: usize) {
    lock_config().max_invalid_calls_listed_count = max_invalid_calls_listed_count;
}

pub fn set_local_max_invalid_calls_listed_count(
    max_invalid_calls_listed_count: usize,
) -> MaxInvalidCallsListedCountResetter {
    let mut config_lock = lock_config();
    let max_invalid_calls_listed_count_resetter = MaxInvalidCallsListedCountResetter {
        old_max_invalid_calls_listed_count: config_lock.max_invalid_calls_listed_count,
    };
    config_lock.max_invalid_calls_listed_count = max_invalid_calls_listed_count;
    return max_invalid_calls_listed_count_resetter;
}

pub struct MaxInvalidCallsListedCountResetter {
    old_max_invalid_calls_listed_count: usize,
}

impl Drop for MaxInvalidCallsListedCountResetter {
    fn drop(&mut self) {
        set_max_invalid_calls_listed_count(self.old_max_invalid_calls_listed_count);
    }
}

fn lock_config<'a>() -> MutexGuard<'a, Config> {
    return CONFIG
        .as_static()
        .lock()
        .expect("Unable to rsubstitute config.");
}
