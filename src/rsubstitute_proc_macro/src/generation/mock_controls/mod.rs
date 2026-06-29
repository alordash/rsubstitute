pub mod models {
    mod static_received;
    mod static_setup;

    pub(crate) use static_received::*;
    pub(crate) use static_setup::*;
}

mod common {
    pub(crate) mod args_checker_stmt;
    pub(crate) mod data_stmt;
    pub(crate) mod generic_arguments;
}

use common::*;

pub mod static_received;
pub mod static_setup;
