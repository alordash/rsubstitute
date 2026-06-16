use super::*;
use crate::generation::r#fn::models::*;
use crate::generation::mock_controls::constants::data_ident;
use crate::generation::mock_controls::models::*;
use crate::generation::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a> {
    pub source_span: Span,
    pub target_ident: Ident,
    pub mock_data_ident: Ident,
    pub fn_infos: &'a [FnInfo],
}

pub(crate) fn generate(
    Params {
        source_span,
        target_ident,
        mock_data_ident,
        fn_infos,
    }: Params,
) -> MockReceived {
    let fields_named = FieldsNamed {
        brace_token: token::Brace(source_span),
        named: punctuated([arc_data_field::new(source_span, mock_data_ident)]),
    };

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        struct_token: Token![struct](source_span),
        ident: format_ident!("{target_ident}Received"),
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

    let items = fn_infos.into_iter().map(|x| generate_impl_fn(x)).collect();

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

    let result = MockReceived {
        path,
        item_struct,
        clone_impl,
        r#impl,
    };

    return result;
}

fn generate_impl_fn(fn_info: &FnInfo) -> ImplItem {
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

    const TIMES_ARG_NAME: &'static str = "times";
    let times_arg = PatType {
        attrs: Vec::new(),
        pat: Box::new(Pat::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new(span, [TIMES_ARG_NAME]),
        })),
        colon_token: Token![:](span),
        ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: path::new(span, ["Times"]),
        })),
    };
    let inputs = core::iter::once(ref_self_fn_arg(span))
        .chain(
            fn_info
                .syntax
                .arguments
                .iter()
                .map(|x| x.control_fn_arg.clone()),
        )
        .chain(core::iter::once(FnArg::Typed(times_arg)))
        .collect();

    let return_type = generate_fn_verifier(fn_info);

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

    let verify_received_stmt = Expr::MethodCall(expr::method_call::new(
        span,
        Expr::Field(expr::field::new(
            Expr::Field(expr::field::new(
                Expr::Path(self_expr_path(span)),
                data_ident(span),
            )),
            fn_info.syntax.fn_ident.clone(),
        )),
        Ident::new("verify_received", span),
        [Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: args_checker_syntax.var_path,
        })],
    ));
    let return_stmt = ExprCall {
        attrs: Vec::new(),
        func: Box::new(Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::new(span, [FN_VERIFIER_NAME, "new"]),
        })),
        paren_token: token::Paren(span),
        args: punctuated([Expr::MethodCall(expr::method_call::new(
            span,
            Expr::Path(self_expr_path(span)),
            Ident::new("clone", span),
            [],
        ))]),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![
            Stmt::Local(args_checker_syntax.local),
            Stmt::Expr(verify_received_stmt, Some(Token![;](span))),
            Stmt::Expr(Expr::Call(return_stmt), None),
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

const FN_VERIFIER_NAME: &'static str = "FnVerifier";
fn generate_fn_verifier(fn_info: &FnInfo) -> TypePath {
    let span = fn_info.syntax.spans.inputs;

    let arg_refs_tuple = arg_refs_tuple::new(span, &fn_info.syntax.arguments);

    let result = TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: punctuated([PathSegment {
                ident: Ident::new(FN_VERIFIER_NAME, span),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Token![<](span),
                    args: punctuated([
                        GenericArgument::Type(Type::Path(self_type(span))),
                        GenericArgument::Type(Type::Tuple(arg_refs_tuple)),
                    ]),
                    gt_token: Token![>](span),
                }),
            }]),
        },
    };

    return result;
}
