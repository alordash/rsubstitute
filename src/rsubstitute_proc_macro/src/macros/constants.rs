use crate::di::SERVICES;
use nameof::name_of_type;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use rsubstitute_core::arguments_matching::Arg;
use std::cell::LazyCell;
use std::iter::{IntoIterator, Iterator};
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::{Attribute, Path, PathArguments, PathSegment, Type, TypePath, TypeTuple};

pub const ARG_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| {
    let result = syn::parse_str(name_of_type!(Arg<()>))
        .expect("Should be able to parse arg wrapper type name as ident.");
    return result;
});

pub const SELF_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("self"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::IArgsMatcher
pub const I_ARGS_MATCHER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsMatcher"));

// TODO - add test that it's equal to rsubstitute_core::FnData
pub const FN_DATA_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("FnData"));

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

pub const BOOL_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(format_ident!("bool"));
    return result;
});

pub const VOID_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let result = Type::Tuple(TypeTuple {
        paren_token: Default::default(),
        elems: Punctuated::new(),
    });
    return result;
});
