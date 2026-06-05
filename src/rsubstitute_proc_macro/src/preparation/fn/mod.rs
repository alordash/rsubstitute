pub mod models {
    mod argument;
    mod fn_syntax;
    mod i_fn_owner;

    pub use argument::*;
    pub use fn_syntax::*;
    pub use i_fn_owner::*;
}

mod argument_preparation;
mod fn_syntax_preparation;

pub use argument_preparation::*;
pub use fn_syntax_preparation::*;
