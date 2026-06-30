use crate::common::models::*;
use crate::common::*;
use crate::generation::fn_info::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::*;

pub(crate) struct Params<'a, 'b, 'c> {
    pub ctx: &'a Context,
    pub source_span: Span,
    pub target_ident: Ident,
    pub target_generics: Generics,
    pub mock_path: &'b Path,
    pub fn_infos: &'c [FnInfo],
}
pub(crate) fn generate(
    Params {
        ctx,
        source_span,
        target_ident,
        target_generics,
        mock_path,
        fn_infos,
    }: Params,
) -> StaticSetupStruct {
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](source_span)),
        struct_token: Token![struct](source_span),
        ident: format_ident!("{}StaticSetup", target_ident),
        generics: target_generics.clone(),
        fields: Fields::Named(FieldsNamed {
            brace_token: token::Brace(source_span),
            named: punctuated([generics_field::new_field(
                source_span,
                target_generics.clone(),
            )]),
        }),
        semi_token: None,
    };
    let path = path::from_ident_with_generics(item_struct.ident.clone(), &item_struct.generics);

    let item_impl = generate_item_impl(
        ctx,
        source_span,
        target_generics,
        mock_path,
        path.clone(),
        fn_infos,
    );

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
    target_generics: Generics,
    mock_path: &Path,
    static_setup_struct_path: Path,
    fn_infos: &[FnInfo],
) -> ItemImpl {
    let fn_setups = fn_infos
        .iter()
        .map(|fn_info| generate_setup_fn(ctx, span, mock_path, fn_info))
        .map(ImplItem::Fn)
        .collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: target_generics,
        trait_: None,
        self_ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: static_setup_struct_path,
        })),
        brace_token: token::Brace(span),
        items: fn_setups,
    };
    return result;
}

fn generate_setup_fn(ctx: &Context, span: Span, mock_path: &Path, fn_info: &FnInfo) -> ImplItemFn {
    let generic_arguments = generic_arguments::new(ctx, span, mock_path.clone(), fn_info);
    let fn_configurator_path = fn_configurator_path::new(
        span,
        fn_info,
        &generic_arguments,
        static_lifetime(span),
        None,
    );
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("setup", span),
        generics: generics_with_rsubstitute_anonymous_lifetime::new(Generics::default()),
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
                path: fn_configurator_path.clone(),
            })),
        ),
    };

    let mut fn_configurator_path_for_var = fn_configurator_path;
    let PathArguments::AngleBracketed(ref mut fn_configurator_path_for_var_args) =
        fn_configurator_path_for_var.segments[0].arguments
    else {
        panic!("FnConfigurator should have angle bracketed arguments.");
    };
    fn_configurator_path_for_var_args.args[0] =
        GenericArgument::Lifetime(placeholder_lifetime(span));
    let (data_var_path, data_stmt) = fn_data_stmt::new_static(span, fn_info, generic_arguments);
    let (args_checker_var_path, args_checker_stmt) = args_checker_stmt::new(span, fn_info);
    let fn_configurator_var_path = path::new(span, ["fn_configurator"]);
    let fn_configurator_stmt = Local {
        attrs: Vec::new(),
        let_token: Token![let](span),
        pat: Pat::Type(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: fn_configurator_var_path.clone(),
            })),
            colon_token: Token![:](span),
            ty: Box::new(Type::Path(TypePath {
                qself: None,
                path: fn_configurator_path_for_var,
            })),
        }),
        init: Some(LocalInit {
            eq_token: Token![=](span),
            expr: Box::new(Expr::MethodCall(expr::method_call::new(
                span,
                Expr::Path(data_var_path.clone()),
                Ident::new("add_config", span),
                [
                    Expr::Path(args_checker_var_path),
                    Expr::Path(self_expr_path(span)),
                ],
            ))),
            diverge: None,
        }),
        semi_token: Token![;](span),
    };
    let return_stmt = Expr::Macro(transmute_lifetime_expr::new(Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: fn_configurator_var_path,
    })));

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(data_stmt),
            Stmt::Local(args_checker_stmt),
            Stmt::Local(fn_configurator_stmt),
            Stmt::Expr(return_stmt, None),
        ],
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
