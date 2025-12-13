use nameof::name_of_type;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use rsubstitute_core::arguments_matching::Arg;
use std::cell::LazyCell;
use std::str::FromStr;
use syn::{AttrStyle, Attribute, MacroDelimiter, Meta, MetaList, Path, PathArguments, PathSegment};

pub const CALL_STRUCT_SUFFIX: &'static str = "Call";

pub const ARG_WRAPPER_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| {
    let result = syn::parse_str(name_of_type!(Arg<()>))
        .expect("Should be able to parse arg wrapper type name as ident.");
    return result;
});

pub const ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    // TODO - create helper function for creation of attributes (accept attribute name and arguments as strings)
    let result = Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: format_ident!("allow"),
                    arguments: PathArguments::None,
                }]
                .into_iter()
                .collect(),
            },
            delimiter: MacroDelimiter::Paren(Default::default()),
            tokens: TokenStream::from_str("non_camel_case_types")
                .expect("Should be able to parse attribute arg."),
        }),
    };
    return result;
});

pub const DERIVE_CLONE_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let result = Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: Meta::List(MetaList {
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: format_ident!("derive"),
                    arguments: PathArguments::None,
                }]
                .into_iter()
                .collect(),
            },
            delimiter: MacroDelimiter::Paren(Default::default()),
            tokens: TokenStream::from_str("Clone").expect("Should be able to parse attribute arg."),
        }),
    };
    return result;
});
