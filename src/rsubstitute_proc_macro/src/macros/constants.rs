use crate::di::SERVICES;
use crate::macros::constants;
use nameof::name_of_type;
use proc_macro2::{Ident, TokenStream};
use quote::format_ident;
use rsubstitute_core::arguments_matching::Arg;
use std::cell::LazyCell;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Expr, ExprCall, ExprPath, ItemUse, Path, PathArguments, PathSegment, Type,
    TypeTuple, UseGlob, UsePath, UseTree, Visibility,
};

pub const ARG_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| {
    let result = syn::parse_str(name_of_type!(Arg<()>))
        .expect("Should be able to parse arg wrapper type name as ident.");
    return result;
});

pub const SELF_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("self"));
pub const SELF_IDENT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(SELF_IDENT.clone());
    return result;
});

pub const SUPER_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("super"));

pub const PRELUDE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("prelude"));

// TODO - add test that it's equal to crate's name
pub const CRATE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("rsubstitute"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::IArgsMatcher
pub const I_ARGS_MATCHER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsMatcher"));

// TODO - add test that it's equal to rsubstitute_core::FnData
pub const FN_DATA_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("FnData"));

// TODO - add test that verifies that it's equal to rsubstitute_core::FnData::add_config
pub const FN_DATA_ADD_CONFIG_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("add_config"));

// TODO - add test that verifies that it's equal to rsubstitute_core::FnData::verify_received
pub const FN_DATA_VERIFY_RECEIVED_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("verify_received"));

// TODO - add test that it's equal to rsubstitute_core::SharedFnConfig
pub const SHARED_FN_CONFIG_TYPE_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("SharedFnConfig"));

// TODO - add test that it's equal to rsubstitute_core::SharedFnConfig::new
pub const SHARED_FN_CONFIG_NEW_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

pub const DISCARD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("_"));

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

pub const SELF_TYPE_KEYWORD: &'static str = "Self";

pub const SELF_TYPE_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("{}", SELF_TYPE_KEYWORD));

pub const SELF_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(SELF_TYPE_IDENT.clone());
    return result;
});

pub const SELF_TYPE_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(SELF_TYPE_IDENT.clone());
    return result;
});

pub const DEFAULT_INVOKE_EXPR: LazyCell<Expr> = LazyCell::new(|| {
    let func = Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [
                PathSegment {
                    ident: format_ident!("Default"),
                    arguments: PathArguments::None,
                },
                PathSegment {
                    ident: format_ident!("default"),
                    arguments: PathArguments::None,
                },
            ]
            .into_iter()
            .collect(),
        },
    });
    let result = Expr::Call(ExprCall {
        attrs: Vec::new(),
        func: Box::new(func),
        paren_token: Default::default(),
        args: Punctuated::new(),
    });
    return result;
});

pub const USE_SUPER: LazyCell<ItemUse> = LazyCell::new(|| {
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: SUPER_IDENT.clone(),
            colon2_token: Default::default(),
            tree: Box::new(UseTree::Glob(UseGlob {
                star_token: Default::default(),
            })),
        }),
        semi_token: Default::default(),
    };
    return result;
});

pub const USE_CRATE_PRELUDE: LazyCell<ItemUse> = LazyCell::new(|| {
    let result = ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: CRATE_IDENT.clone(),
            colon2_token: Default::default(),
            tree: Box::new(UseTree::Path(UsePath {
                ident: PRELUDE_IDENT.clone(),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Default::default(),
                })),
            })),
        }),
        semi_token: Default::default(),
    };
    todo!();
});
