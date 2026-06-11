use crate::preparation::r#fn::models::*;
use crate::syntax::{path, r#type};
use proc_macro2::Span;
use syn::*;

pub(crate) fn generate_args_checker_impl(
    span: Span,
    arguments: &[Argument],
    target_type: Type,
) -> ItemImpl {
    let fn_check = generate_fn_check(span, arguments);
    let fn_fmt_args = generate_fn_fmt_args(span, arguments);
    let items = vec![ImplItem::Fn(fn_check), ImplItem::Fn(fn_fmt_args)];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics: Generics::default(),
        trait_: Some((None, path::new(span, ["IArgsChecker"]), Token![for](span))),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_fn_check(span: Span, arguments: &[Argument]) -> ImplItemFn {
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
                span,
                Type::Path(r#type::path::new(span, ["ArgCheckResult"])),
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

fn generate_fn_fmt_args(span: Span, arguments: &[Argument]) -> ImplItemFn {
    let result = todo!();

    return result;
}
