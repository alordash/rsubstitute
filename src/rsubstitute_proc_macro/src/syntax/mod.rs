pub(crate) mod arg_ident;
pub(crate) mod arg_type;
pub(crate) mod attribute;
pub(crate) mod bool_lit;
pub(crate) mod expr_call;
pub(crate) mod expr_method_call;
pub(crate) mod expr_reference;
pub(crate) mod field;
pub(crate) mod field_access_expr;
pub(crate) mod field_value;
pub(crate) mod generic_argument;
pub(crate) mod generics;
pub(crate) mod r#impl;
pub(crate) mod local;
pub(crate) mod path;
pub(crate) mod reference;
pub(crate) mod r#struct;
pub(crate) mod transmute_lifetime_expr;
pub(crate) mod r#type;

mod field_required_ident_extension;

pub use field_required_ident_extension::*;
