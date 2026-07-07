pub mod models {
    mod received;
    mod setup;
    mod static_received;
    mod static_setup;
    mod control_type;

    pub(crate) use received::*;
    pub(crate) use setup::*;
    pub(crate) use static_received::*;
    pub(crate) use static_setup::*;
    pub(crate) use control_type::*;
}

mod common {
    pub(crate) mod args_checker_stmt;
    pub(crate) mod fn_configurator_path;
    pub(crate) mod received_impl;
    pub(crate) mod setup_impl;
    pub(crate) mod times_arg;
    pub(crate) mod control_struct;
}

use common::*;

pub mod fn_static_received;
pub mod fn_static_setup;
pub mod received;
pub mod setup;
pub mod static_received;
pub mod static_setup;
