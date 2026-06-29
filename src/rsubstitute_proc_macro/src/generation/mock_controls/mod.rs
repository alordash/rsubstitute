pub mod models {
    mod static_received;
    mod static_setup;

    pub(crate) use static_received::*;
    pub(crate) use static_setup::*;
}

mod common {
    pub(crate) mod args_checker_stmt;
    pub(crate) mod fn_data_stmt;
    pub(crate) mod fn_configurator_path;
    pub(crate) mod generic_arguments;
    pub(crate) mod times_arg;
}

use common::*;

pub mod fn_static_received;
pub mod fn_static_setup;
pub mod static_received;
pub mod static_setup;
