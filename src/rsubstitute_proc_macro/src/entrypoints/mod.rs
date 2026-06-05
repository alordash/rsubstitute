pub mod models {
    mod mock_macro_usage;

    pub use mock_macro_usage::*;
}

mod automock_attribute;
mod mock_macro;

pub use automock_attribute::*;
pub use mock_macro::*;
