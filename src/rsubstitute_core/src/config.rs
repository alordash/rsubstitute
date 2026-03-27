use std::sync::*;

pub struct Config {
    pub max_invalid_calls_listed_count: usize,
}

pub const DEFAULT_CONFIG: Config = Config {
    max_invalid_calls_listed_count: 10,
};

pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(DEFAULT_CONFIG));

pub fn read_config<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG.read().expect(LOCK_ERROR_MSG)
}

pub fn write_config<'a>() -> RwLockWriteGuard<'a, Config> {
    CONFIG.write().expect(LOCK_ERROR_MSG)
}

const LOCK_ERROR_MSG: &'static str = "Unable to lock `rsubstitute` config.";