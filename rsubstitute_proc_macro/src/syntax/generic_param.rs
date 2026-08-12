use syn::*;

pub(crate) fn get_ident(generic_param: &GenericParam) -> &Ident {
    match generic_param {
        GenericParam::Lifetime(lifetime_param) => &lifetime_param.lifetime.ident,
        GenericParam::Type(type_param) => &type_param.ident,
        GenericParam::Const(const_param) => &const_param.ident,
    }
}
