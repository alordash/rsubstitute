pub mod models {
    mod argument;
    mod fn_syntax;
    mod i_fn_owner;

    pub use argument::*;
    pub use fn_syntax::*;
    pub use i_fn_owner::*;
}

mod common;

pub mod fn_syntax;
