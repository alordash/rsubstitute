use crate::di::SERVICES;
use crate::syntax::*;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::clone::Clone;
use std::iter::{IntoIterator, Iterator};
use syn::punctuated::Punctuated;
use syn::*;

macro_rules! define {
    // For idents
    ($symbol:ident, $value:literal) => {
        pub(crate) const $symbol: LazyCell<Ident> = LazyCell::new(|| format_ident!($value));
    };

    // For everything else
    ($symbol:ident, $ty:ty, $block:block) => {
        pub(crate) const $symbol: LazyCell<$ty> = LazyCell::new(|| $block);
    };
}

define!(DATA_IDENT, "data");
pub(crate) const MOCK_STRUCT_IDENT_PREFIX: &'static str = "Mock";
define!(MOCK_SETUP_FIELD_IDENT, "setup");
define!(MOCK_RECEIVED_FIELD_IDENT, "received");

define!(SELF_IDENT, "self");
define!(SELF_EXPR, Expr, {
    let path_factory = &SERVICES.path_factory;
    let result = Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path_factory.create(SELF_IDENT.clone()),
    });
    return result;
});

define!(MACRO_VEC_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("vec"));
    return result;
});

define!(SUPER_IDENT, "super");
define!(FOR_GENERATED_IDENT, "for_generated");
define!(CRATE_IDENT, "rsubstitute");
define!(ARG_TYPE_IDENT, "Arg");

pub(crate) const I_ARGS_FORMATTER_TRAIT_NAME: &'static str = "IArgsFormatter";
define!(I_ARGS_FORMATTER_TRAIT_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("{I_ARGS_FORMATTER_TRAIT_NAME}"));
    return result;
});

pub(crate) const I_GENERICS_HASH_KEY_PROVIDER_TRAIT_NAME: &'static str = "IGenericsHashKeyProvider";
define!(I_GENERICS_HASH_KEY_PROVIDER_TRAIT_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("{I_GENERICS_HASH_KEY_PROVIDER_TRAIT_NAME}"));
    return result;
});

pub(crate) const CLONE_FOR_RSUBSTITUTE_TRAIT_NAME: &'static str = "CloneForRSubstitute";
define!(DERIVE_CLONE_FOR_RSUBSTITUTE_ATTRIBUTE, Attribute, {
    let result = attribute::create(DERIVE_IDENT.clone(), CLONE_FOR_RSUBSTITUTE_TRAIT_NAME);
    return result;
});

define!(I_ARGS_FORMATTER_FN_IDENT, "fmt_args");
define!(I_ARGS_CHECKER_TRAIT_IDENT, "IArgsChecker");

pub(crate) const I_ARGS_INFOS_PROVIDER_TRAIT_NAME: &'static str = "IArgsInfosProvider";
pub(crate) const I_ARGS_INFOS_PROVIDER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("{I_ARGS_INFOS_PROVIDER_TRAIT_NAME}"));

pub(crate) const I_ARGS_TUPLE_PROVIDER_TRAIT_NAME: &'static str = "IArgsTupleProvider";
pub(crate) const I_ARGS_TUPLE_PROVIDER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("{I_ARGS_TUPLE_PROVIDER_TRAIT_NAME}"));

define!(I_MOCK_DATA_TRAIT_IDENT, "IMockData");

pub(crate) const I_MOCK_DATA_GET_RECEIVED_NOTHING_ELSE_ERROR_MSGS_FN_SIGNATURE: LazyCell<
    Signature,
> = LazyCell::new(|| {
    let signature = Signature {
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
    };
    return signature;
});

define!(FN_DATA_TYPE_IDENT, "FnData");

define!(NEW_IDENT, "new");

pub(crate) const SETUP_MEMBER: LazyCell<Member> =
    LazyCell::new(|| Member::Named(format_ident!("setup")));
pub(crate) const RECEIVED_MEMBER: LazyCell<Member> =
    LazyCell::new(|| Member::Named(format_ident!("received")));

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

pub(crate) const DATA_SHORT_FIELD_VALUE: LazyCell<FieldValue> = LazyCell::new(|| FieldValue {
    attrs: Vec::new(),
    member: Member::Named(DATA_IDENT.clone()),
    colon_token: None,
    expr: EMPTY_PATH_EXPR.clone(),
});
define!(DATA_FIELD_VALUE, FieldValue, {
    let expr_method_call_factory = &SERVICES.expr_method_call_factory;
    let result = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(DATA_IDENT.clone()),
        colon_token: Some(Default::default()),
        expr: Expr::MethodCall(expr_method_call_factory.create(
            vec![DATA_IDENT.clone()],
            format_ident!("clone"),
            Vec::new(),
        )),
    };
    return result;
});

define!(FN_DATA_ADD_CONFIG_FN_IDENT, "add_config");

define!(FN_DATA_VERIFY_RECEIVED_FN_IDENT, "verify_received");

define!(FN_TUNER_TYPE_IDENT, "FnTuner");

define!(FN_VERIFIER_TYPE_IDENT, "FnVerifier");
define!(FN_VERIFIER_NEW_FN_EXPR, Expr, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory
        .create_expr_from_parts(vec![FN_VERIFIER_TYPE_IDENT.clone(), NEW_IDENT.clone()]);
    return result;
});

define!(ALLOW_IDENT, "allow");

define!(ALLOW_UNUSED_ATTRIBUTE, Attribute, {
    let result = attribute::create(ALLOW_IDENT.clone(), "unused");
    return result;
});

define!(ALLOW_UNUSED_PARENS_ATTRIBUTE, Attribute, {
    let result = attribute::create(ALLOW_IDENT.clone(), "unused_parens");
    return result;
});

define!(ALLOW_MISMATCHED_LIFETIME_SYNTAXES_ATTRIBUTE, Attribute, {
    let result = attribute::create(ALLOW_IDENT.clone(), "mismatched_lifetime_syntaxes");
    return result;
});

define!(ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE, Attribute, {
    let result = attribute::create(ALLOW_IDENT.clone(), "non_camel_case_types");
    return result;
});

define!(ALLOW_NON_SNAKE_CASE_ATTRIBUTE, Attribute, {
    let result = attribute::create(ALLOW_IDENT.clone(), "non_snake_case");
    return result;
});

pub(crate) const DEBUG_TRAIT_NAME: &'static str = "Debug";

pub(crate) const CLONE_TRAIT_STR: &'static str = "Clone";
pub(crate) const CLONE_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("{CLONE_TRAIT_STR}"));
define!(CLONE_FN_SIGNATURE, Signature, {
    let signature = Signature {
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
    };
    return signature;
});

define!(DERIVE_IDENT, "derive");

define!(DERIVE_MOCK_DATA_ATTRIBUTE, Attribute, {
    let result = attribute::create(DERIVE_IDENT.clone(), &I_MOCK_DATA_TRAIT_IDENT.to_string());
    return result;
});

define!(DOC_HIDDEN_ATTRIBUTE, Attribute, {
    let ident = format_ident!("doc");
    let result = attribute::create(ident, "hidden");
    return result;
});

define!(CFG_TEST_ATTRIBUTE, Attribute, {
    let ident = format_ident!("cfg");
    let result = attribute::create(ident, "test");
    return result;
});

define!(CFG_NOT_TEST_ATTRIBUTE, Attribute, {
    let ident = format_ident!("cfg");
    let result = attribute::create(ident, "not(test)");
    return result;
});

define!(VOID_TYPE, Type, {
    let result = Type::Tuple(TypeTuple {
        paren_token: Default::default(),
        elems: Punctuated::new(),
    });
    return result;
});

define!(MACRO_FORMAT_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("format"));
    return result;
});

define!(SELF_TYPE_IDENT, "Self");

define!(SELF_TYPE_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(SELF_TYPE_IDENT.clone());
    return result;
});

define!(SELF_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(SELF_TYPE_IDENT.clone());
    return result;
});

define!(REF_SELF_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.reference(
        Type::Path(TypePath {
            qself: None,
            path: SELF_TYPE_PATH.clone(),
        }),
        Some(DEFAULT_ARG_LIFETIME.clone()),
    );
    return result;
});

define!(STRING_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(format_ident!("String"));
    return result;
});

define!(VEC_OF_ARG_CHECK_RESULT_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let arg_check_result_type = type_factory.create(format_ident!("ArgCheckResult"));
    let result = type_factory.wrap_in(arg_check_result_type, format_ident!("Vec"));
    return result;
});

define!(VEC_OF_ARG_INFO_RESULT_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let arg_check_result_type = type_factory.create(format_ident!("ArgInfo"));
    let result = type_factory.wrap_in(arg_check_result_type, format_ident!("Vec"));
    return result;
});

define!(VEC_OF_VEC_OF_STRINGS_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.wrap_in(
        type_factory.wrap_in(STRING_TYPE.clone(), format_ident!("Vec")),
        format_ident!("Vec"),
    );
    return result;
});

define!(ARC_IDENT, "Arc");

define!(REF_SELF_ARG, FnArg, {
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

define!(USE_SUPER, ItemUse, {
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

define!(USE_FOR_GENERATED, ItemUse, {
    let result = ItemUse {
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
    };
    return result;
});

define!(PHANTOM_DATA_IDENT, "PhantomData");
define!(PHANTOM_DATA_EXPR_PATH, Expr, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create_expr(PHANTOM_DATA_IDENT.clone());
    return result;
});

define!(DEFAULT_ARG_LIFETIME_FIELD_IDENT, "_phantom_lifetime");

define!(DEFAULT_ARG_LIFETIME_FIELD, Field, {
    let type_factory = &SERVICES.type_factory;
    let ty = type_factory.phantom_data_lifetime(DEFAULT_ARG_LIFETIME.clone());
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(DEFAULT_ARG_LIFETIME_FIELD_IDENT.clone()),
        colon_token: Some(Default::default()),
        ty,
    };
    return result;
});

define!(RETURN_TYPE_PHANTOM_FIELD_IDENT, "_return_type");

pub(crate) const DERIVED_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("_"),
});

pub(crate) const ANONYMOUS_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("__rsa"),
});

pub(crate) const DEFAULT_ARG_LIFETIME_NAME: &'static str = "__rs";
pub(crate) const DEFAULT_ARG_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("{DEFAULT_ARG_LIFETIME_NAME}"),
});

pub(crate) const STATIC_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("static"),
});

define!(CLONE_FN_IDENT, "clone");

define!(RESET_IDENT, "reset");

define!(INTO_TRAIT_IDENT, "Into");
define!(INTO_FN_IDENT, "into");

define!(GET_GLOBAL_MOCK_FN_IDENT, "get_global_mock");

define!(DEFAULT_TRAIT_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("Default"));
    return result;
});

define!(DEFAULT_FN_IDENT, "default");
define!(INNER_DATA_FIELD_IDENT, "inner_data");

define!(DEREF_TRAIT_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("Deref"));
    return result;
});
define!(DEREF_TARGET_TYPE_IDENT, "Target");
define!(DEREF_FN_IDENT, "deref");

define!(IGNORE_IMPL_ATTRIBUTE_IDENT, "unmock");

define!(HASH_FN_IDENT, "hash");

define!(ARG_PRINTER_STRUCT_IDENT, "ArgPrinter");
define!(DEBUG_STRING_FN_IDENT, "debug_string");

define!(MUT_INFER_PTR_TYPE, Type, {
    Type::Ptr(TypePtr {
        star_token: Default::default(),
        const_token: Some(Default::default()),
        mutability: Some(Default::default()),
        elem: Box::new(Type::Infer(TypeInfer {
            underscore_token: Default::default(),
        })),
    })
});

define!(MUT_VOID_PTR_TYPE, Type, {
    Type::Ptr(TypePtr {
        star_token: Default::default(),
        const_token: Some(Default::default()),
        mutability: Some(Default::default()),
        elem: Box::new(VOID_TYPE.clone()),
    })
});

define!(DYN_CALL_REF_TYPE, Type, {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.reference(type_factory.create(format_ident!("DynCall")), None);
    return result;
});
define!(DYN_CALL_DOWNCAST_REF_FN_IDENT, "downcast_ref");

#[cfg(not(feature = "mock_base_by_default"))]
pub(crate) const SUPPORT_BASE_PARAMETER: &'static str = "base";

#[cfg(feature = "mock_base_by_default")]
pub(crate) const DO_NOT_SUPPORT_BASE_PARAMETER: &'static str = "no_base";

pub(crate) const BASE_FN_IDENT_PREFIX: &'static str = "base";

define!(BOX_IDENT, "Box");
define!(BOX_NEW_EXPR, Expr, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create_expr_from_parts(vec![BOX_IDENT.clone(), NEW_IDENT.clone()]);
    return result;
});
define!(BOX_LEAK_EXPR, Expr, {
    let path_factory = &SERVICES.path_factory;
    let result =
        path_factory.create_expr_from_parts(vec![BOX_IDENT.clone(), format_ident!("leak")]);
    return result;
});

define!(TRANSMUTE_LIFETIME_MACRO_PATH, Path, {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("transmute_lifetime"));
    return result;
});
