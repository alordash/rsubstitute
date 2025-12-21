pub mod args_matching;

mod fn_config;
mod fn_data;
mod shared_fn_config;
mod times;
mod error_printer;
mod di;

pub use fn_config::*;
pub use fn_data::*;
pub use shared_fn_config::*;
pub use times::*;

pub mod prelude {
    pub use crate::args_matching::*;
    pub use crate::*;
}