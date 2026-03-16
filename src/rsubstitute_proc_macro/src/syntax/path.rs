use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn create(ident: Ident) -> Path {
    let result = Path {
        leading_colon: None,
        segments: [PathSegment {
            ident,
            arguments: PathArguments::None,
        }]
        .into_iter()
        .collect(),
    };
    return result;
}

pub(crate) fn create_with_generics(ident: Ident, generics: Generics) -> Path {
    let arguments = generics_to_path_arguments(generics);
    let result = Path {
        leading_colon: None,
        segments: [PathSegment { ident, arguments }].into_iter().collect(),
    };
    return result;
}

pub(crate) fn create_expr_with_generics(ident: Ident, generics: Generics) -> Expr {
    to_expr(create_with_generics(ident, generics))
}

pub(crate) fn create_from_parts(idents: Vec<Ident>) -> Path {
    let result = Path {
        leading_colon: None,
        segments: idents
            .into_iter()
            .map(|ident| PathSegment {
                ident,
                arguments: PathArguments::None,
            })
            .collect(),
    };
    return result;
}

pub(crate) fn create_expr_from_parts_with_generics(idents: Vec<Ident>, generics: Generics) -> Expr {
    let arguments = generics_to_path_arguments(generics);
    let mut result = Path {
        leading_colon: None,
        segments: idents
            .into_iter()
            .map(|ident| PathSegment {
                ident,
                arguments: PathArguments::None,
            })
            .collect(),
    };
    if let Some(ref mut last_segment) = result.segments.last_mut() {
        last_segment.arguments = arguments;
    }
    return to_expr(result);
}

pub(crate) fn create_expr(ident: Ident) -> Expr {
    to_expr(create(ident))
}

pub(crate) fn create_expr_from_parts(idents: Vec<Ident>) -> Expr {
    to_expr(create_from_parts(idents))
}

fn to_expr(path: Path) -> Expr {
    let expr = Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path,
    });
    return expr;
}

fn generics_to_path_arguments(generics: Generics) -> PathArguments {
    let arguments = if generics.params.is_empty() {
        PathArguments::None
    } else {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Default::default(),
            args: generics
                .params
                .iter()
                .map(|x| generic_argument::create(x.clone()))
                .collect(),
            gt_token: Default::default(),
        })
    };
    return arguments;
}
