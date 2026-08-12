use crate::syntax::path;
use proc_macro2::Span;
use syn::*;

pub(crate) fn bool(span: Span, value: bool) -> GenericArgument {
    let result = GenericArgument::Const(Expr::Lit(ExprLit {
        attrs: Vec::new(),
        lit: Lit::Bool(LitBool { span, value }),
    }));
    return result;
}

pub(crate) fn from_param(generic_param: GenericParam) -> GenericArgument {
    let result = match generic_param {
        GenericParam::Lifetime(lifetime_param) => {
            GenericArgument::Lifetime(lifetime_param.lifetime)
        }
        GenericParam::Type(type_param) => GenericArgument::Type(Type::Path(TypePath {
            attrs: Vec::new(),
            qself: None,
            path: path::from_ident(type_param.ident),
        })),
        GenericParam::Const(const_param) => GenericArgument::Const(Expr::Path(ExprPath {
            attrs: Vec::new(),
            qself: None,
            path: path::from_ident(const_param.ident),
        })),
    };
    return result;
}
