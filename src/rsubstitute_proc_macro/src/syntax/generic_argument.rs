use crate::syntax::*;
use syn::*;

pub(crate) fn create(generic_param: GenericParam) -> GenericArgument {
    let result = match generic_param {
        GenericParam::Lifetime(lifetime_param) => {
            GenericArgument::Lifetime(lifetime_param.lifetime.clone())
        }
        GenericParam::Type(type_param) => {
            GenericArgument::Type(r#type::create(type_param.ident.clone()))
        }
        GenericParam::Const(const_param) => {
            GenericArgument::Const(path::create_expr(const_param.ident.clone()))
        }
    };
    return result;
}
