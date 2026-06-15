use super::*;
use crate::generation::mock_controls::constants::data_ident;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::generation::r#fn::models::*;
use crate::generation::*;
use crate::preparation::models::Context;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a> {
    pub ctx: &'a Context,
    pub source_span: Span,
    pub target_ident: Ident,
    pub mock_type: Type,
    pub mock_data_ident: Ident,
    pub stores_mock_data: bool,
    pub fn_infos: &'a [FnInfo],
}

pub(crate) fn generate(
    Params {
        ctx,
        source_span,
        target_ident,
        mock_type,
        mock_data_ident,
        stores_mock_data,
        fn_infos,
    }: Params,
) -> MockSetup {
    let fields_named = FieldsNamed {
        brace_token: token::Brace(source_span),
        named: punctuated([Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(data_ident(source_span)),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(r#type::arc_of(
                source_span,
                Type::Path(TypePath {
                    qself: None,
                    path: path::from_ident(mock_data_ident),
                }),
            )),
        }]),
    };

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        struct_token: Token![struct](source_span),
        ident: format_ident!("{target_ident}Setup"),
        generics: Generics::default(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let r#type = Type::Path(TypePath {
        qself: None,
        path: path::from_ident(item_struct.ident.clone()),
    });
    let clone_impl = clone_impl::new(source_span, r#type.clone());

    let items = fn_infos
        .into_iter()
        .map(|x| generate_impl_fn(ctx, mock_type.clone(), stores_mock_data, x))
        .collect();

    let r#impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](source_span),
        generics: Generics::default(),
        trait_: None,
        self_ty: Box::new(r#type.clone()),
        brace_token: token::Brace(source_span),
        items,
    };

    let result = MockSetup {
        r#type,
        item_struct,
        clone_impl,
        r#impl,
    };

    return result;
}

fn generate_impl_fn(
    ctx: &Context,
    mock_type: Type,
    stores_mock_data: bool,
    fn_info: &FnInfo,
) -> ImplItem {
    let span = fn_info.syntax.spans.inputs;

    let mut generics = fn_info.syntax.merged_generics.clone();
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: anonymous_lifetime::new(span),
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );

    let inputs = core::iter::once(ref_self_fn_arg(span))
        .chain(
            fn_info
                .syntax
                .arguments
                .iter()
                .map(|x| x.control_fn_arg.clone()),
        )
        .collect();

    let return_type = fn_configurator::new(fn_configurator::Params {
        ctx,
        mock_type,
        stores_mock_data,
        fn_info,
    });

    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: fn_info.syntax.fn_ident.clone(),
        generics,
        paren_token: token::Paren(span),
        inputs,
        variadic: None,
        output: ReturnType::Type(Token!(->)(span), Box::new(Type::Path(return_type.clone()))),
    };

    let args_checker_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Path(TypePath {
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::Struct(ExprStruct {
                attrs: Vec::new(),
                qself: None,
                path: fn_info.args_checker_struct.path.clone(),
                brace_token: token::Brace(span),
                fields: fn_info
                    .syntax
                    .arguments
                    .iter()
                    .map(|argument| FieldValue {
                        attrs: Vec::new(),
                        member: Member::Named(argument.ident.clone()),
                        colon_token: Some(Token![:](span)),
                        expr: Expr::Macro(transmute_lifetime_expr::new(Expr::MethodCall(
                            expr::method_call::new(
                                span,
                                Expr::Path(ExprPath {
                                    attrs: Vec::new(),
                                    qself: None,
                                    path: fn_info.args_checker_struct.path.clone(),
                                }),
                                Ident::new("into", span),
                                [],
                            ),
                        ))),
                    })
                    .collect(),
                dot2_token: None,
                rest: None,
            })),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };

    const FN_CONFIGURATOR_VAR_NAME: &'static str = "fn_configurator";
    let fn_configurator_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(PatPath {
                attrs: Vec::new(),
                qself: None,
                path: path::new(span, [FN_CONFIGURATOR_VAR_NAME]),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Path(return_type)),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Field(expr::field::new(
                    Expr::Field(expr::field::new_self(data_ident(span))),
                    fn_info.syntax.fn_ident.clone(),
                )),
                Ident::new("add_config", span),
                [
                    Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: fn_info.args_checker_struct.path.clone(),
                    }),
                    Expr::Path(self_expr_path(span)),
                ],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    let return_stmt = transmute_lifetime_expr::new(Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: path::new(span, [FN_CONFIGURATOR_VAR_NAME]),
    }));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(args_checker_stmt),
            Stmt::Local(fn_configurator_stmt),
            Stmt::Expr(Expr::Macro(return_stmt), None),
        ],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token!(pub)(span)),
        defaultness: None,
        sig,
        block,
    };

    return ImplItem::Fn(result);
}
