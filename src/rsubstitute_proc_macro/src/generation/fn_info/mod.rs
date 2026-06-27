pub mod models {
    mod args_checker_struct;
    mod call_struct;
    mod fn_info;

    pub use args_checker_struct::*;
    pub use call_struct::*;
    pub use fn_info::*;
}

mod common;
mod generation;

pub use common::*;
pub use generation::*;

pub mod args_checker_struct;
pub mod call_struct;
