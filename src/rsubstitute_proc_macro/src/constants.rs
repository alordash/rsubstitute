// TODO - move to crate root
use crate::di::SERVICES;
use proc_macro2::{Ident, Span, TokenStream};
use quote::format_ident;
use std::cell::LazyCell;
use std::str::FromStr;
use syn::punctuated::Punctuated;
use syn::*;

pub const SELF_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("self"));
pub const SELF_IDENT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(SELF_IDENT.clone());
    return result;
});

pub const MACRO_VEC_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("vec"));
    return result;
});

pub const SUPER_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("super"));

pub const PRELUDE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("prelude"));

// TODO - add test that it's equal to crate's name
pub const CRATE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("rsubstitute"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::Argh
pub const ARG_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Arg"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::IArgsFormatter
pub const I_ARGS_FORMATTER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsFormatter"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::IArgsFormatter::fmt_args
pub const I_ARGS_FORMATTER_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fmt_args"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::IArgschecker
pub const I_ARGS_CHECKER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsChecker"));

// TODO - add test that it's equal to rsubstitute_core::arguments_matching::IArgschecker::check
pub const I_ARGS_CHECKER_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("check"));

// TODO - add test that it's equal to rsubstitute_core::FnData
pub const FN_DATA_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("FnData"));

// TODO - add test that it's equal to rsubstitute_core::FnData::new
pub const FN_DATA_NEW_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

// TODO - add test that verifies that it's equal to rsubstitute_core::FnData::add_config
pub const FN_DATA_ADD_CONFIG_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("add_config"));

// TODO - add test that verifies that it's equal to rsubstitute_core::FnData::verify_received
pub const FN_DATA_VERIFY_RECEIVED_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("verify_received"));

pub const SERVICES_REF_EXPR: LazyCell<Expr> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let path = path_factory.create(format_ident!("SERVICES"));
    let result = Expr::Reference(ExprReference {
        attrs: Vec::new(),
        and_token: Default::default(),
        mutability: None,
        expr: Box::new(Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path,
        })),
    });
    return result;
});

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

pub const DERIVE_DEBUG_AND_I_ARGS_FORMATTER_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = format_ident!("derive");
    let arguments = TokenStream::from_str("Debug, IArgsFormatter")
        .expect("Should be able to parse attribute arg.");
    let result = attribute_factory.create(ident, arguments);
    return result;
});

pub const VOID_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let result = Type::Tuple(TypeTuple {
        paren_token: Default::default(),
        elems: Punctuated::new(),
    });
    return result;
});

pub const MACRO_FORMAT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("format"));
    return result;
});

pub const SELF_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Self"));

pub const SELF_TYPE_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(SELF_TYPE_IDENT.clone());
    return result;
});

pub const SELF_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(SELF_TYPE_IDENT.clone());
    return result;
});

pub const REF_SELF_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let result = Type::Reference(TypeReference {
        and_token: Default::default(),
        lifetime: None,
        mutability: None,
        elem: Box::new(Type::Path(TypePath {
            qself: None,
            path: SELF_TYPE_PATH.clone(),
        })),
    });
    return result;
});

pub const STRING_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(format_ident!("String"));
    return result;
});

// TODO - add tests to verify that ArcCheckResult ident is correct
pub const VEC_OF_ARG_CHECK_RESULT_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let arg_check_result_type = type_factory.create(format_ident!("ArcCheckResult"));
    let result = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: format_ident!("Vec"),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [GenericArgument::Type(arg_check_result_type)]
                        .into_iter()
                        .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    return result;
});

pub const REF_SELF_ARG: LazyCell<FnArg> = LazyCell::new(|| {
    let result = FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: Some((Default::default(), None)),
        mutability: None,
        self_token: Default::default(),
        colon_token: None,
        ty: Box::new(REF_SELF_TYPE.clone()),
    });
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
    return result;
});

pub const DEFAULT_ARG_FIELD_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("__rsubstitute_arg_field_lifetime"),
});
