use crate::syntax;
use crate::syntax::*;
use quote::{ToTokens, format_ident};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_generics_info_provider(generics: Generics, target_type: Type) -> ItemImpl {
    let get_generic_parameter_infos_fn = generate_get_generic_parameter_infos(&generics);
    let items = vec![ImplItem::Fn(get_generic_parameter_infos_fn)];

    let span = generics.span();
    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](span),
        generics,
        // todo - maybe somehow test that it's equal to real trait
        trait_: Some((
            None,
            path::new(["IGenericsInfoProvider"], span),
            Token![for](span),
        )),
        self_ty: Box::new(target_type),
        brace_token: token::Brace(span),
        items,
    };

    return result;
}

fn generate_get_generic_parameter_infos(generics: &Generics) -> ImplItemFn {
    let span = generics.span();
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: Ident::new("get_generic_parameter_infos", span),
        generics: Generics::default(),
        paren_token: token::Paren(span),
        inputs: [ref_self_fn_arg(span)].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(
            Token![->](span),
            Box::new(Type::Path(r#type::vec_of(
                Type::Path(r#type::path::new(["GenericParameterInfo"], span)),
                span,
            ))),
        ),
    };

    let generic_parameter_infos: Punctuated<Expr, Token![,]> = generics
        .params
        .iter()
        .filter_map(|generic_param| match generic_param {
            GenericParam::Type(type_param) => Some(Expr::Call(expr::call::new(
                Expr::Path(expr::path::new(["generic_type_info"], span)),
                [
                    Expr::Path(expr::path::new([&type_param.ident.to_string()], span)),
                    Expr::Call(expr::call::new(
                        Expr::Path(expr::path::new(["core", "any", "type_name"], span)),
                        [],
                        span,
                    )),
                ],
                span,
            ))),
            GenericParam::Const(_) => {
                todo!()
            }
            _ => None,
        })
        .collect();

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![Stmt::Macro(StmtMacro {
            attrs: Vec::new(),
            mac: Macro {
                path: path::new(["vec"], span),
                bang_token: Token![!](span),
                delimiter: MacroDelimiter::Bracket(token::Bracket(span)),
                tokens: generic_parameter_infos.to_token_stream(),
            },
            semi_token: None,
        })],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };

    return result;
}
