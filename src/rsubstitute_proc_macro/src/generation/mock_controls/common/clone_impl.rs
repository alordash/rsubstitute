use crate::syntax::*;
use proc_macro2::Span;
use syn::*;

pub(crate) fn new(source_span: Span, target_type: Type) -> ItemImpl {
    let fn_clone = generate_fn_clone(source_span);
    let items = vec![ImplItem::Fn(fn_clone)];

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](source_span),
        generics: Generics::default(),
        trait_: Some((
            None,
            path::new(source_span, ["Clone"]),
            Token![for](source_span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(source_span),
        items,
    };

    return result;
}

fn generate_fn_clone(source_span: Span) -> ImplItemFn {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](source_span),
        ident: Ident::new("clone", source_span),
        generics: Generics::default(),
        paren_token: token::Paren(source_span),
        inputs: punctuated([ref_self_fn_arg(source_span)]),
        variadic: None,
        output: ReturnType::Type(
            Token!(->)(source_span),
            Box::new(Type::Path(self_type(source_span))),
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
