// TODO - write macros for ident definition, replace all LazyCell<Ident>
use crate::constants;
use crate::di::SERVICES;
use proc_macro2::{Ident, Span};
use quote::format_ident;
use std::cell::LazyCell;
use std::clone::Clone;
use std::iter::{IntoIterator, Iterator};
use syn::punctuated::Punctuated;
use syn::*;

pub const DATA_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("data"));
pub const MOCK_STRUCT_IDENT_PREFIX: &'static str = "Mock";
pub const MOCK_SETUP_FIELD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("setup"));
pub const MOCK_RECEIVED_FIELD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("received"));

pub const SELF_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("self"));

pub const RETURN_SELF_STMT: LazyCell<Stmt> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let return_self_stmt = Stmt::Expr(
        Expr::Return(ExprReturn {
            attrs: Vec::new(),
            return_token: Default::default(),
            expr: Some(Box::new(path_factory.create_expr(SELF_IDENT.clone()))),
        }),
        Some(Default::default()),
    );
    return return_self_stmt;
});

pub const MACRO_VEC_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("vec"));
    return result;
});

pub const SUPER_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("super"));

pub const FOR_GENERATED_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("for_generated"));

pub const CRATE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("rsubstitute"));

pub const ARG_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Arg"));

pub const I_ARGS_FORMATTER_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("IArgsFormatter"));
    return result;
});

pub const I_GENERICS_HASH_KEY_PROVIDER_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("IGenericsHashKeyProvider"));
    return result;
});

pub const I_ARGS_FORMATTER_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("fmt_args"));

pub const I_ARGS_CHECKER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsChecker"));

pub const I_ARGS_INFOS_PROVIDER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsInfosProvider"));

pub const I_ARGS_TUPLE_PROVIDER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IArgsTupleProvider"));

pub const I_BASE_CALLER_TRAIT_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("IBaseCaller"));

pub const I_BASE_CALLER_CALL_BASE_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("call_base"));

pub const I_MOCK_DATA_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("IMockData"));

pub const I_MOCK_DATA_GET_RECEIVED_NOTHING_ELSE_ERROR_MSGS_FN_SIGNATURE: LazyCell<Signature> =
    LazyCell::new(|| {
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

pub const FN_DATA_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("FnData"));

pub const NEW_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

pub const SETUP_MEMBER: LazyCell<Member> = LazyCell::new(|| Member::Named(format_ident!("setup")));
pub const RECEIVED_MEMBER: LazyCell<Member> =
    LazyCell::new(|| Member::Named(format_ident!("received")));

pub const EMPTY_PATH_EXPR: LazyCell<Expr> = LazyCell::new(|| {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: Punctuated::new(),
        },
    })
});

pub const DATA_SHORT_FIELD_VALUE: LazyCell<FieldValue> = LazyCell::new(|| FieldValue {
    attrs: Vec::new(),
    member: Member::Named(DATA_IDENT.clone()),
    colon_token: None,
    expr: EMPTY_PATH_EXPR.clone(),
});
pub const DATA_FIELD_VALUE: LazyCell<FieldValue> = LazyCell::new(|| {
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

pub const FN_DATA_ADD_CONFIG_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("add_config"));

pub const FN_DATA_VERIFY_RECEIVED_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("verify_received"));

pub const SERVICES_REF_EXPR: LazyCell<Expr> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let expr_reference_factory = &SERVICES.expr_reference_factory;
    let result = expr_reference_factory.create(path_factory.create_expr(format_ident!("SERVICES")));
    return result;
});

pub const FN_TUNER_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("FnTuner"));

pub const FN_TUNER_NEW_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

pub const ALLOW_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("allow"));

pub const ALLOW_DEAD_CODE_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let result = attribute_factory.create(ALLOW_IDENT.clone(), "dead_code");
    return result;
});

pub const ALLOW_UNUSED_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let result = attribute_factory.create(ALLOW_IDENT.clone(), "unused");
    return result;
});

pub const ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let result = attribute_factory.create(ALLOW_IDENT.clone(), "non_camel_case_types");
    return result;
});

pub const ALLOW_NON_SNAKE_CASE_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let result = attribute_factory.create(ALLOW_IDENT.clone(), "non_snake_case");
    return result;
});

pub const DERIVE_DEBUG_AND_I_ARGS_FORMATTER_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = DERIVE_IDENT.clone();
    let result = attribute_factory.create(ident, "Debug, IArgsFormatter");
    return result;
});
pub const DEBUG_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Debug"));
pub const DEBUG_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create_from_parts(vec![
        format_ident!("std"),
        format_ident!("fmt"),
        DEBUG_TRAIT_IDENT.clone(),
    ]);
    return result;
});

pub const PARTIAL_ORD_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("PartialOrd"));
pub const PARTIAL_ORD_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create_from_parts(vec![
        format_ident!("core"),
        format_ident!("cmp"),
        PARTIAL_ORD_TRAIT_IDENT.clone(),
    ]);
    return result;
});

pub const CLONE_TRAIT_STR: &'static str = "Clone";
pub const CLONE_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("{CLONE_TRAIT_STR}"));
pub const CLONE_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create_from_parts(vec![
        format_ident!("core"),
        format_ident!("clone"),
        CLONE_TRAIT_IDENT.clone(),
    ]);
    return result;
});
pub const CLONE_FN_SIGNATURE: LazyCell<Signature> = LazyCell::new(|| {
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

pub const DERIVE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("derive"));
pub const DERIVE_CLONE_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let result = attribute_factory.create(DERIVE_IDENT.clone(), &CLONE_TRAIT_IDENT.to_string());
    return result;
});

pub const DERIVE_MOCK_DATA_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let result =
        attribute_factory.create(DERIVE_IDENT.clone(), &I_MOCK_DATA_TRAIT_IDENT.to_string());
    return result;
});

pub const DOC_HIDDEN_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = format_ident!("doc");
    let result = attribute_factory.create(ident, "hidden");
    return result;
});

pub const CFG_TEST_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = format_ident!("cfg");
    let result = attribute_factory.create(ident, "test");
    return result;
});

pub const CFG_NOT_TEST_ATTRIBUTE: LazyCell<Attribute> = LazyCell::new(|| {
    let attribute_factory = &SERVICES.attribute_factory;
    let ident = format_ident!("cfg");
    let result = attribute_factory.create(ident, "not(test)");
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
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.reference(
        Type::Path(TypePath {
            qself: None,
            path: SELF_TYPE_PATH.clone(),
        }),
        Some(DEFAULT_ARG_FIELD_LIFETIME.clone()),
    );
    return result;
});

pub const STRING_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.create(format_ident!("String"));
    return result;
});

pub const VEC_OF_ARG_CHECK_RESULT_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let arg_check_result_type = type_factory.create(format_ident!("ArgCheckResult"));
    let result = type_factory.wrap_in(arg_check_result_type, format_ident!("Vec"));
    return result;
});

pub const VEC_OF_ARG_INFO_RESULT_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let arg_check_result_type = type_factory.create(format_ident!("ArgInfo"));
    let result = type_factory.wrap_in(arg_check_result_type, format_ident!("Vec"));
    return result;
});

pub const VEC_OF_VEC_OF_STRINGS_TYPE: LazyCell<Type> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = type_factory.wrap_in(
        type_factory.wrap_in(STRING_TYPE.clone(), format_ident!("Vec")),
        format_ident!("Vec"),
    );
    return result;
});

pub const ARC_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Arc"));

pub const SELF_ARG: LazyCell<FnArg> = LazyCell::new(|| {
    let result = FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: None,
        mutability: None,
        self_token: Default::default(),
        colon_token: None,
        ty: Box::new(SELF_TYPE.clone()),
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

pub const REF_SELF_ARG_WITH_LIFETIME: LazyCell<FnArg> = LazyCell::new(|| {
    let result = FnArg::Receiver(Receiver {
        attrs: Vec::new(),
        reference: Some((Default::default(), Some(DEFAULT_ARG_FIELD_LIFETIME.clone()))),
        mutability: None,
        self_token: Default::default(),
        colon_token: None,
        ty: Box::new(REF_SELF_TYPE.clone()),
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

pub const USE_FOR_GENERATED: LazyCell<ItemUse> = LazyCell::new(|| {
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

pub const PHANTOM_DATA_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("PhantomData"));
pub const PHANTOM_DATA_EXPR_PATH: LazyCell<Expr> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create_expr(PHANTOM_DATA_IDENT.clone());
    return result;
});

pub const DEFAULT_ARG_FIELD_LIFETIME_FIELD_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("_phantom_lifetime"));

pub const DEFAULT_ARG_FIELD_LIFETIME_FIELD: LazyCell<Field> = LazyCell::new(|| {
    let type_factory = &SERVICES.type_factory;
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(DEFAULT_ARG_FIELD_LIFETIME_FIELD_IDENT.clone()),
        colon_token: Some(Default::default()),
        ty: Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: [PathSegment {
                    ident: PHANTOM_DATA_IDENT.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Default::default(),
                        args: [GenericArgument::Type(type_factory.reference(
                            VOID_TYPE.clone(),
                            Some(DEFAULT_ARG_FIELD_LIFETIME.clone()),
                        ))]
                        .into_iter()
                        .collect(),
                        gt_token: Default::default(),
                    }),
                }]
                .into_iter()
                .collect(),
            },
        }),
    };
    return result;
});

pub const ANONYMOUS_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("__rsubstitute_arg_anonymous"),
});

pub const DEFAULT_ARG_FIELD_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("__rsubstitute_arg_field_lifetime"),
});

pub const STATIC_LIFETIME: LazyCell<Lifetime> = LazyCell::new(|| Lifetime {
    apostrophe: Span::call_site(),
    ident: format_ident!("static"),
});

pub const DEFAULT_ARG_FIELD_LIFETIME_FIELD_VALUE: LazyCell<FieldValue> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let field_value = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(DEFAULT_ARG_FIELD_LIFETIME_FIELD_IDENT.clone()),
        colon_token: Some(Default::default()),
        expr: path_factory.create_expr(PHANTOM_DATA_IDENT.clone()),
    };
    return field_value;
});

pub const SEND_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Send"));
pub const SYNC_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Sync"));

pub const CLONE_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("clone"));

pub const RESET_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("reset"));

pub const INTO_TRAIT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Into"));
pub const INTO_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("into"));

pub const GET_GLOBAL_MOCK_FN_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("get_global_mock"));

pub const DEFAULT_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("Default"));
    return result;
});

pub const DEFAULT_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("default"));
pub const INNER_DATA_FIELD_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("inner_data"));

pub const DEREF_TRAIT_PATH: LazyCell<Path> = LazyCell::new(|| {
    let path_factory = &SERVICES.path_factory;
    let result = path_factory.create(format_ident!("Deref"));
    return result;
});
pub const DEREF_TARGET_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Target"));
pub const DEREF_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("deref"));

pub const IGNORE_IMPL_ATTRIBUTE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("unmock"));

pub const CONFIG_LIFETIME_IDENT: LazyCell<Ident> =
    LazyCell::new(|| format_ident!("__rsubstitute_config"));
pub const CONFIG_LIFETIME_GENERICS: LazyCell<Generics> = LazyCell::new(|| {
    let lifetime_param = GenericParam::Lifetime(LifetimeParam {
        attrs: Vec::new(),
        lifetime: Lifetime {
            apostrophe: Span::call_site(),
            ident: CONFIG_LIFETIME_IDENT.clone(),
        },
        colon_token: None,
        bounds: Punctuated::new(),
    });
    let generics = Generics {
        lt_token: Some(Default::default()),
        params: [lifetime_param].into_iter().collect(),
        gt_token: Some(Default::default()),
        where_clause: None,
    };
    return generics;
});

pub const HASH_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("hash"));

pub const ARG_PRINTER_STRUCT_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("ArgPrinter"));
pub const DEBUG_STRING_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("debug_string"));

pub const VOID_PTR_TYPE: LazyCell<Type> = LazyCell::new(|| {
    Type::Ptr(TypePtr {
        star_token: Default::default(),
        const_token: Some(Default::default()),
        mutability: None,
        elem: Box::new(VOID_TYPE.clone()),
    })
});
