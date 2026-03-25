use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::clone::Clone;
use std::iter::{IntoIterator, Iterator};
use syn::punctuated::Punctuated;
use syn::*;

macro_rules! ident {
    ($symbol:ident, $value:literal) => {
        pub(crate) const $symbol: LazyCell<Ident> = LazyCell::new(|| format_ident!($value));
    };
    ($symbol:ident, $value_ident:ident) => {
        pub(crate) const $symbol: LazyCell<Ident> =
            LazyCell::new(|| format_ident!("{}", $value_ident));
    };
}

macro_rules! define {
    ($symbol:ident, $ty:ty, $expr:expr) => {
        pub(crate) const $symbol: LazyCell<$ty> = LazyCell::new(|| $expr);
    };
}

ident!(VEC_TYPE_IDENT, "Vec");

ident!(DATA_IDENT, "data");
pub(crate) const MOCK_STRUCT_IDENT_PREFIX: &'static str = "Mock";
ident!(MOCK_SETUP_FIELD_IDENT, "setup");
ident!(MOCK_RECEIVED_FIELD_IDENT, "received");

ident!(SELF_IDENT, "self");
define!(
    SELF_EXPR,
    Expr,
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path::create(SELF_IDENT.clone()),
    })
);

define!(MACRO_VEC_PATH, Path, path::create(format_ident!("vec")));

ident!(SUPER_IDENT, "super");
ident!(FOR_GENERATED_IDENT, "for_generated");
ident!(CRATE_IDENT, "rsubstitute");
ident!(ARG_TYPE_IDENT, "Arg");

pub(crate) const I_ARGS_FORMATTER_TRAIT_NAME: &'static str = "IArgsFormatter";
define!(
    I_ARGS_FORMATTER_TRAIT_PATH,
    Path,
    path::create(format_ident!("{I_ARGS_FORMATTER_TRAIT_NAME}"))
);

define!(
    I_GENERICS_HASH_KEY_PROVIDER_TRAIT_PATH,
    Path,
    path::create(format_ident!("IGenericsInfoProvider"))
);

ident!(I_ARGS_FORMATTER_FN_IDENT, "fmt_args");
ident!(I_ARGS_CHECKER_TRAIT_IDENT, "IArgsChecker");

pub(crate) const I_ARGS_INFOS_PROVIDER_TRAIT_NAME: &'static str = "IArgsInfosProvider";
ident!(
    I_ARGS_INFOS_PROVIDER_TRAIT_IDENT,
    I_ARGS_INFOS_PROVIDER_TRAIT_NAME
);

pub(crate) const I_ARGS_TUPLE_PROVIDER_TRAIT_NAME: &'static str = "IArgsTupleProvider";
ident!(
    I_ARGS_TUPLE_PROVIDER_TRAIT_IDENT,
    I_ARGS_TUPLE_PROVIDER_TRAIT_NAME
);

ident!(I_MOCK_DATA_TRAIT_IDENT, "IMockData");
define!(
    I_MOCK_DATA_GET_RECEIVED_NOTHING_ELSE_ERROR_MSGS_FN_SIGNATURE,
    Signature,
    Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: format_ident!("get_received_nothing_else_error_msgs"),
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs: [REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Default::default(),
            Box::new(VEC_OF_VEC_OF_STRINGS_TYPE.clone()),
        ),
    }
);

ident!(FN_DATA_TYPE_IDENT, "FnData");

ident!(NEW_IDENT, "new");

define!(SETUP_MEMBER, Member, Member::Named(format_ident!("setup")));
define!(
    RECEIVED_MEMBER,
    Member,
    Member::Named(format_ident!("received"))
);

define!(EMPTY_PATH_EXPR, Expr, {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::new(),
        },
    })
});

define!(
    DATA_SHORT_FIELD_VALUE,
    FieldValue,
    FieldValue {
        attrs: Vec::new(),
        member: Member::Named(DATA_IDENT.clone()),
        colon_token: None,
        expr: EMPTY_PATH_EXPR.clone(),
    }
);

define!(
    DATA_FIELD_VALUE,
    FieldValue,
    FieldValue {
        attrs: Vec::new(),
        member: Member::Named(DATA_IDENT.clone()),
        colon_token: Some(Default::default()),
        expr: Expr::MethodCall(method_call::create(
            vec![DATA_IDENT.clone()],
            format_ident!("clone"),
            Vec::new(),
        )),
    }
);

ident!(FN_DATA_ADD_CONFIG_FN_IDENT, "add_config");

ident!(FN_DATA_VERIFY_RECEIVED_FN_IDENT, "verify_received");

ident!(FN_TUNER_TYPE_IDENT, "FnTuner");

ident!(FN_VERIFIER_TYPE_IDENT, "FnVerifier");
define!(
    FN_VERIFIER_NEW_FN_EXPR,
    Expr,
    path::create_expr_from_parts(vec![FN_VERIFIER_TYPE_IDENT.clone(), NEW_IDENT.clone()])
);

ident!(ALLOW_IDENT, "allow");

define!(
    ALLOW_UNUSED_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "unused")
);

define!(
    ALLOW_UNUSED_PARENS_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "unused_parens")
);

define!(
    ALLOW_UNUSED_VARIABLES_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "unused_variables")
);

define!(
    ALLOW_NON_SHORTHAND_FIELD_PATTERNS_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "non_shorthand_field_patterns")
);

define!(
    ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "mismatched_lifetime_syntaxes")
);

define!(
    ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "non_camel_case_types")
);

define!(
    ALLOW_NON_SNAKE_CASE_ATTRIBUTE,
    Attribute,
    attribute::create(ALLOW_IDENT.clone(), "non_snake_case")
);

pub(crate) const DEBUG_TRAIT_NAME: &'static str = "Debug";

pub(crate) const CLONE_TRAIT_STR: &'static str = "Clone";
ident!(CLONE_TRAIT_IDENT, CLONE_TRAIT_STR);
define!(
    CLONE_FN_SIGNATURE,
    Signature,
    Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: format_ident!("clone"),
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs: [REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(Default::default(), Box::new(SELF_TYPE.clone())),
    }
);

ident!(DERIVE_IDENT, "derive");

define!(
    DERIVE_MOCK_DATA_ATTRIBUTE,
    Attribute,
    attribute::create(DERIVE_IDENT.clone(), &I_MOCK_DATA_TRAIT_IDENT.to_string())
);

define!(
    DOC_HIDDEN_ATTRIBUTE,
    Attribute,
    attribute::create(format_ident!("doc"), "hidden")
);

define!(
    CFG_TEST_ATTRIBUTE,
    Attribute,
    attribute::create(format_ident!("cfg"), "test")
);

define!(
    CFG_NOT_TEST_ATTRIBUTE,
    Attribute,
    attribute::create(format_ident!("cfg"), "not(test)")
);

define!(
    VOID_TYPE,
    Type,
    Type::Tuple(TypeTuple {
        paren_token: Default::default(),
        elems: Punctuated::new(),
    })
);

define!(
    MACRO_FORMAT_PATH,
    Path,
    path::create(format_ident!("format"))
);

pub const SELF_TYPE_NAME: &'static str = "Self";
ident!(SELF_TYPE_IDENT, SELF_TYPE_NAME);
define!(SELF_TYPE_PATH, Path, path::create(SELF_TYPE_IDENT.clone()));
define!(SELF_TYPE, Type, r#type::create(SELF_TYPE_IDENT.clone()));

define!(
    REF_SELF_TYPE,
    Type,
    r#type::reference(
        Type::Path(TypePath {
            qself: None,
            path: SELF_TYPE_PATH.clone(),
        }),None,
    )
);

define!(STRING_TYPE, Type, r#type::create(format_ident!("String")));

define!(
    VEC_OF_ARG_CHECK_RESULT_TYPE,
    Type,
    r#type::wrap_in(
        r#type::create(format_ident!("ArgCheckResult")),
        VEC_TYPE_IDENT.clone()
    )
);

define!(
    VEC_OF_ARG_INFO_RESULT_TYPE,
    Type,
    r#type::wrap_in(
        r#type::create(format_ident!("ArgInfo")),
        VEC_TYPE_IDENT.clone()
    )
);

define!(
    VEC_OF_VEC_OF_STRINGS_TYPE,
    Type,
    r#type::wrap_in(
        r#type::wrap_in(STRING_TYPE.clone(), VEC_TYPE_IDENT.clone()),
        VEC_TYPE_IDENT.clone(),
    )
);

ident!(ARC_IDENT, "Arc");

define!(
    REF_SELF_ARG,
    FnArg,
    FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: Some((Default::default(), None)),
        mutability: None,
        self_token: Default::default(),
        colon_token: None,
        ty: Box::new(REF_SELF_TYPE.clone()),
    })
);

define!(
    USE_SUPER,
    ItemUse,
    ItemUse {
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
    }
);

define!(
    USE_FOR_GENERATED,
    ItemUse,
    ItemUse {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        use_token: Default::default(),
        leading_colon: None,
        tree: UseTree::Path(UsePath {
            ident: CRATE_IDENT.clone(),
            colon2_token: Default::default(),
            tree: Box::new(UseTree::Path(UsePath {
                ident: FOR_GENERATED_IDENT.clone(),
                colon2_token: Default::default(),
                tree: Box::new(UseTree::Glob(UseGlob {
                    star_token: Default::default(),
                })),
            })),
        }),
        semi_token: Default::default(),
    }
);

ident!(PHANTOM_DATA_IDENT, "PhantomData");
define!(
    PHANTOM_DATA_EXPR_PATH,
    Expr,
    path::create_expr(PHANTOM_DATA_IDENT.clone())
);

ident!(RETURN_TYPE_PHANTOM_FIELD_IDENT, "_return_type");

define!(
    ANONYMOUS_LIFETIME,
    Lifetime,
    Lifetime {
        apostrophe: Span::call_site(),
        ident: format_ident!("_"),
    }
);
define!(
    ANONYMOUS_LIFETIME_PARAM,
    LifetimeParam,
    LifetimeParam {
        attrs: Vec::new(),
        lifetime: ANONYMOUS_LIFETIME.clone(),
        colon_token: None,
        bounds: Punctuated::new()
    }
);

pub(crate) const PLACEHOLDER_LIFETIME_NAME: &'static str = "__rsa";
define!(
    PLACEHOLDER_LIFETIME,
    Lifetime,
    Lifetime::new(&format!("'{PLACEHOLDER_LIFETIME_NAME}"), Span::call_site())
);
define!(
    PLACEHOLDER_LIFETIME_PARAM,
    LifetimeParam,
    LifetimeParam {
        attrs: Vec::new(),
        lifetime: PLACEHOLDER_LIFETIME.clone(),
        colon_token: None,
        bounds: Punctuated::new()
    }
);

define!(
    STATIC_LIFETIME,
    Lifetime,
    Lifetime {
        apostrophe: Span::call_site(),
        ident: format_ident!("static"),
    }
);

ident!(CLONE_FN_IDENT, "clone");

ident!(RESET_IDENT, "reset");

ident!(INTO_TRAIT_IDENT, "Into");
ident!(INTO_FN_IDENT, "into");

ident!(GET_STATIC_FN_GLOBAL_MOCK_FN_IDENT, "get_static_fn_global_mock");

define!(
    DEFAULT_TRAIT_PATH,
    Path,
    path::create(format_ident!("Default"))
);

ident!(DEFAULT_FN_IDENT, "default");
ident!(INNER_DATA_FIELD_IDENT, "inner_data");

define!(DEREF_TRAIT_PATH, Path, path::create(format_ident!("Deref")));
ident!(DEREF_TARGET_TYPE_IDENT, "Target");
ident!(DEREF_FN_IDENT, "deref");

pub const IGNORE_IMPL_ATTRIBUTE_IDENT_NAME: &'static str = "unmock";
ident!(
    IGNORE_IMPL_ATTRIBUTE_IDENT,
    IGNORE_IMPL_ATTRIBUTE_IDENT_NAME
);

ident!(HASH_FN_IDENT, "hash");

ident!(ARG_PRINTER_STRUCT_IDENT, "ArgPrinter");
ident!(DEBUG_STRING_FN_IDENT, "debug_string");

define!(
    MUT_INFER_PTR_TYPE,
    Type,
    Type::Ptr(TypePtr {
        star_token: Default::default(),
        const_token: Some(Default::default()),
        mutability: Some(Default::default()),
        elem: Box::new(Type::Infer(TypeInfer {
            underscore_token: Default::default(),
        })),
    })
);

define!(
    MUT_VOID_PTR_TYPE,
    Type,
    Type::Ptr(TypePtr {
        star_token: Default::default(),
        const_token: Some(Default::default()),
        mutability: Some(Default::default()),
        elem: Box::new(VOID_TYPE.clone()),
    })
);

define!(
    DYN_CALL_REF_TYPE,
    Type,
    r#type::reference(r#type::create(format_ident!("DynCall")), None)
);
ident!(DYN_CALL_DOWNCAST_REF_FN_IDENT, "downcast_ref");

#[cfg(not(feature = "mock_base_by_default"))]
pub(crate) const SUPPORT_BASE_PARAMETER: &'static str = "base";

#[cfg(feature = "mock_base_by_default")]
pub(crate) const DO_NOT_SUPPORT_BASE_PARAMETER: &'static str = "no_base";

pub(crate) const BASE_FN_IDENT_PREFIX: &'static str = "base";

ident!(BOX_IDENT, "Box");
define!(
    BOX_NEW_EXPR,
    Expr,
    path::create_expr_from_parts(vec![BOX_IDENT.clone(), NEW_IDENT.clone()])
);
define!(
    BOX_LEAK_EXPR,
    Expr,
    path::create_expr_from_parts(vec![BOX_IDENT.clone(), format_ident!("leak")])
);

define!(
    TRANSMUTE_LIFETIME_MACRO_PATH,
    Path,
    path::create(format_ident!("transmute_lifetime"))
);

ident!(GENERIC_TYPE_INFO_FN_IDENT, "generic_type_info");
ident!(GENERIC_CONST_INFO_FN_IDENT, "generic_const_info");

ident!(GET_MOCK_FN_IDENT, "get_mock");
define!(
    GET_MOCK_FN_CALL_EXPR,
    Expr,
    call::create_from_ident(GET_MOCK_FN_IDENT.clone(), Vec::new())
);

ident!(AS_REF_TRAIT_IDENT, "AsRef");
ident!(AS_REF_FN_IDENT, "as_ref");
