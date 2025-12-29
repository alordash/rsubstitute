use crate::syntax::{IPathFactory, ITypeFactory};
use std::sync::Arc;
use syn::*;

pub trait IGenericArgumentFactory {
    fn create(&self, generic_param: GenericParam) -> GenericArgument;
}

pub(crate) struct GenericArgumentFactory {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
}

impl IGenericArgumentFactory for GenericArgumentFactory {
    fn create(&self, generic_param: GenericParam) -> GenericArgument {
        let result = match generic_param {
            GenericParam::Lifetime(lifetime_param) => {
                GenericArgument::Lifetime(lifetime_param.lifetime.clone())
            }
            GenericParam::Type(type_param) => {
                GenericArgument::Type(self.type_factory.create(type_param.ident.clone()))
            }
            GenericParam::Const(const_param) => GenericArgument::Const(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: self.path_factory.create(const_param.ident.clone()),
            })),
        };
        return result;
    }
}
