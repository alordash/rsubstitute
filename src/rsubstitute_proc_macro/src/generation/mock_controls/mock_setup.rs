use super::*;
use crate::generation::r#fn::models::*;
use crate::generation::mock_controls::constants::data_ident;
use crate::generation::mock_controls::models::*;
use crate::generation::*;
use crate::preparation::models::*;
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
        named: punctuated([arc_data_field::new(source_span, mock_data_ident)]),
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

    let path = path::from_ident(item_struct.ident.clone());
    let r#type = Type::Path(TypePath {
        qself: None,
        path: path.clone(),
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
        self_ty: Box::new(r#type),
        brace_token: token::Brace(source_span),
        items,
    };

    let result = MockSetup {
        path,
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
            lifetime: rsubstitute_lifetime::new(span),
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

    let return_type = generate_fn_configurator(ctx, mock_type, stores_mock_data, fn_info);

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

    let args_checker_syntax = args_checker_syntax::new(span, fn_info);

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
                        path: args_checker_syntax.var_path,
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
            Stmt::Local(args_checker_syntax.local),
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

fn generate_fn_configurator(
    ctx: &Context,
    mock_type: Type,
    stores_mock_data: bool,
    fn_info: &FnInfo,
) -> TypePath {
    let span = fn_info.syntax.spans.inputs;

    let arg_refs_tuple = arg_refs_tuple::new(span, &fn_info.syntax.arguments);

    let return_type = match &fn_info.syntax.return_type {
        ReturnType::Default => void_type(span),
        ReturnType::Type(_, ty) => r#type::anonymize_all_references(*ty.clone()),
    };

    let mock_arg = if stores_mock_data {
        Type::Reference(TypeReference {
            and_token: Token![&](span),
            lifetime: None,
            mutability: None,
            elem: Box::new(mock_type.clone()),
        })
    } else {
        mock_type.clone()
    };

    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: punctuated([PathSegment {
                ident: Ident::new("FnConfigurator", span),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token![<](span),
                    args: punctuated([
                        GenericArgument::Lifetime(placeholder_lifetime::new(span)),
                        GenericArgument::Type(mock_type),
                        GenericArgument::Type(Type::Path(self_type(span))),
                        GenericArgument::Type(Type::Tuple(arg_refs_tuple)),
                        GenericArgument::Type(return_type),
                        GenericArgument::Type(mock_arg),
                        bool_generic_arg::new(span, ctx.support_base_calling),
                        bool_generic_arg::new(span, stores_mock_data),
                    ]),
                    gt_token: Token![>](span),
                }),
            }]),
        },
    };

    return result;
}
