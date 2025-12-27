pub use rsubstitute_core::args_matching::*;
pub use rsubstitute_core::*;

pub mod macros {
    pub use rsubstitute_proc_macro::*;
}

pub mod prelude {
    pub use crate::*;
    pub use macros::*;
}

// TODO - move to separate crate?
pub mod assertions;

#[cfg(not(test))]
pub fn static_fn(a: i32, b: &[i32]) -> String {
    let c: Vec<_> = b.iter().map(|x| x * a).collect();
    return format!("{c:?}");
}

#[cfg(test)]
pub fn static_fn(a: i32, b: &[i32]) -> String {
    return "MOCK!!!".to_string();
}

pub fn qweee() {
    println!("{}", crate::static_fn(2, &[3, 4]));
}

// #[cfg(test)]
mod qwe {
    #[test]
    fn ee() {
        crate::qweee();
    }
}
