use std::sync::*;

pub const DEFAULT_CONFIG: Config = Config {
    max_invalid_calls_listed_count: 10,
};

pub(crate) static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| RwLock::new(DEFAULT_CONFIG));

pub struct Config {
    pub max_invalid_calls_listed_count: usize,
}

pub fn read_config<'a>() -> RwLockReadGuard<'a, Config> {
    println!("Trying to read config");
    CONFIG.read().expect("Unable to lock rsubstitute config")
}

pub fn write_config<'a>() -> RwLockWriteGuard<'a, Config> {
    println!("Trying to write config");
    CONFIG.write().expect("Unable to lock rsubstitute config")
}
