pub(crate) mod models {
    mod associated_items_info;
    mod context;

    pub(crate) use associated_items_info::*;
    pub(crate) use context::*;
}

pub(crate) mod context;
pub(crate) mod data_field;
pub(crate) mod generics_field;
pub(crate) mod normalization;
pub(crate) mod rsubstitute_lifetime;
pub(crate) mod transmute_lifetime_expr;
pub(crate) mod generics_phantom_data;
pub(crate) mod rsubstitute_for_generated;

mod rsubstitute_self;
pub(crate) use rsubstitute_self::*;
