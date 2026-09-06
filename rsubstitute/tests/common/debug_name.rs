use std::fmt::Display;

pub fn debug_name<T: Display>(
    #[cfg_attr(not(feature = "debug_naming"), allow(unused))] t: &T,
) -> String {
    #[cfg(feature = "debug_naming")]
    return t.to_string();
    #[cfg(not(feature = "debug_naming"))]
    return "?".to_string();
}
