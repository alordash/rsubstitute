use crate::preparation::r#fn::models::*;
use crate::syntax::{path, r#type};
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate_args_checker_impl(
    arguments: &[Argument],
    target_type: Type,
    span: Span,
) -> ItemImpl {
    let fn_check = generate_fn_check(arguments, span);
    let fn_fmt_args = generate_fn_fmt_args(arguments, span);
    let items = vec![ImplItem::Fn(fn_check), ImplItem::Fn(fn_fmt_args)];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: Generics::default(),
        trait_: Some((None, path::new(["IArgsChecker"], span), Token![for](span))),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_check(arguments: &[Argument], span: Span) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("check", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: todo!(),
        variadic: None,
        output: ReturnType::Type(
            Token!(->)(span),
            Box::new(Type::Path(r#type::vec_of(
                Type::Path(r#type::path::new(["ArgCheckResult"], span)),
                span,
            ))),
        ),
    };

    let block = todo!();

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };

    return result;
}

fn generate_fn_fmt_args(arguments: &[Argument], span: Span) -> ImplItemFn {
    let result = todo!();

    return result;
}
