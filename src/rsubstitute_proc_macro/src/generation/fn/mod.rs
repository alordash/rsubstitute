pub mod models {
    mod args_checker_struct;
    mod call_struct;
    mod fn_info;

    pub use args_checker_struct::*;
    pub use call_struct::*;
    pub use fn_info::*;
}

mod arg_printer_expr;
mod args_checker_struct_generation;
mod call_struct_generation;
mod fn_info_generation;
mod generics_info_provider_generation;
mod transmute_lifetime_expr;

pub use arg_printer_expr::*;
pub use args_checker_struct_generation::*;
pub use call_struct_generation::*;
pub use fn_info_generation::*;
pub use generics_info_provider_generation::*;
pub use transmute_lifetime_expr::*;
