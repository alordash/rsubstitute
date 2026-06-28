use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_for_static_fn(
    ctx: &Context,
    source_span: Span,
    fn_info: &FnInfo,
    mock_path: Path,
) -> StaticSetupStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](source_span)),
        struct_token: Token![struct](source_span),
        ident: format_ident!("{}StaticSetup", fn_info.syntax.fn_ident),
        generics: fn_info.syntax.merged_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: punctuated([generics_field::new_field(
                source_span,
                &fn_info.syntax.merged_generics,
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident(item_struct.ident.clone());

    let item_impl = generate_item_impl(ctx, source_span, mock_path, path.clone(), fn_info);

    let result = StaticSetupStruct {
        path,
        item_struct,
        item_impl,
    };
    return result;
}

fn generate_item_impl(
    ctx: &Context,
    span: Span,
    mock_path: Path,
    static_setup_struct_path: Path,
    fn_info: &FnInfo,
) -> ItemImpl {
    let fn_setup = generate_setup_fn(ctx, span, mock_path, fn_info);

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: fn_info.syntax.merged_generics.clone(),
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: static_setup_struct_path,
        })),
        brace_token: token::Brace(span),
        items: vec![ImplItem::Fn(fn_setup)],
    };
    return result;
}

fn generate_setup_fn(ctx: &Context, span: Span, mock_path: Path, fn_info: &FnInfo) -> ImplItemFn {
    let mock_generic_argument = GenericArgument::Type(Type::Path(TypePath {
        qself: None,
        path: mock_path,
    }));
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("setup", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: [ref_self_fn_arg(span)]
            .into_iter()
            .chain(
                fn_info
                    .syntax
                    .arguments
                    .iter()
                    .map(|x| x.control_fn_arg.clone()),
            )
            .collect(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: punctuated([PathSegment {
                        ident: Ident::new("FnConfigurator", span),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Token![<](span),
                            args: punctuated([
                                GenericArgument::Lifetime(placeholder_lifetime(span)),
                                mock_generic_argument.clone(),
                                GenericArgument::Type(Type::Path(self_type(span))),
                                GenericArgument::Type(Type::Tuple(
                                    fn_info.syntax.arg_refs_tuple.clone(),
                                )),
                                GenericArgument::Type(match &fn_info.syntax.return_type {
                                    ReturnType::Default => void_type(span),
                                    ReturnType::Type(_, return_type) => *return_type.clone(),
                                }),
                                mock_generic_argument,
                                generic_argument::bool(
                                    span,
                                    match fn_info.syntax.return_type {
                                        ReturnType::Default => false,
                                        ReturnType::Type(_, _) => true,
                                    },
                                ),
                                generic_argument::bool(span, ctx.support_base_calling),
                                generic_argument::bool(span, false),
                            ]),
                            gt_token: Token![>](span),
                        }),
                    }]),
                },
            })),
        ),
    };

    let args_checker_var_path = expr::path::new(span, ["args_checker"]);
    let args_checker_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Path(args_checker_var_path.clone()),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
                brace_token: token::Brace(span),
                fields: [generics_field::new_value(span)]
                    .into_iter()
                    .chain(fn_info.syntax.arguments.iter().map(|x| FieldValue {
                        attrs: Vec::new(),
                        member: Member::Named(x.ident.clone()),
                        colon_token: Some(Token![:](span)),
                        expr: Expr::Macro(transmute_lifetime_expr::new(Expr::MethodCall(
                            expr::method_call::new(
                                span,
                                Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: path::from_ident(x.ident.clone()),
                                }),
                                Ident::new("into", span),
                                [],
                            ),
                        ))),
                    }))
                    .collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Local(args_checker_stmt)],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        defaultness: None,
        sig,
        block,
    };
    return result;
}
