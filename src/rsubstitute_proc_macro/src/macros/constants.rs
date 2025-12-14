use crate::di::SERVICES;
use nameof::name_of_type;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use rsubstitute_core::arguments_matching::Arg;
use std::cell::LazyCell;
use std::str::FromStr;
use syn::Attribute;

pub const ARG_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| {
    let result = syn::parse_str(name_of_type!(Arg<()>))
        .expect("Should be able to parse arg wrapper type name as ident.");
    return result;
});

pub const ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = format_ident!("allow");
    let arguments = TokenStream::from_str("non_camel_case_types")
        .expect("Should be able to parse attribute arg.");
    let result = attribute_factory.create(ident, arguments);
    return result;
});

pub const DERIVE_CLONE_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = format_ident!("derive");
    let arguments = TokenStream::from_str("Clone").expect("Should be able to parse attribute arg.");
    let result = attribute_factory.create(ident, arguments);
    return result;
});
