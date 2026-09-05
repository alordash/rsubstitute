use std::sync::*;

/// Crate level configuration definition.
pub struct Config {
    /// Controls how many invalid calls will be listed in case of an error.
    pub max_invalid_calls_listed_count: usize,
}

/// Default crate level configuration value.
pub const DEFAULT_CONFIG: Config = Config {
    max_invalid_calls_listed_count: 10,
};

/// Crate level configuration value.
pub static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(DEFAULT_CONFIG));

/// Reads crate level configuration, returning read guard.
pub fn read_config<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG.read().expect(LOCK_ERROR_MSG)
}

/// Writes crate level configuration, returning write guard.
pub fn write_config<'a>() -> RwLockWriteGuard<'a, Config> {
    CONFIG.write().expect(LOCK_ERROR_MSG)
}

const LOCK_ERROR_MSG: &'static str = "Unable to lock `rsubstitute` config.";
