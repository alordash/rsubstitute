pub mod models {
    mod static_setup;

    pub use static_setup::*;
}

mod common {
    pub(crate) mod args_checker_stmt;
}

use common::*; 

pub mod static_setup;
