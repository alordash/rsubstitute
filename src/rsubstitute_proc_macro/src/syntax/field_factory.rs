use crate::syntax::ITypeFactory;
use proc_macro2::Ident;
use std::sync::Arc;
use syn::*;

// TODO - review code where fields are created manually, this could be used in many places
pub trait IFieldFactory {
    fn create(&self, ident: Ident, ty: Type) -> Field;

    fn create_from_struct(&self, ident: Ident, item_struct: &ItemStruct) -> Field;

    fn create_pub_from_struct(&self, ident: Ident, item_struct: &ItemStruct) -> Field;
}

pub(crate) struct FieldFactory {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl IFieldFactory for FieldFactory {
    fn create(&self, ident: Ident, ty: Type) -> Field {
        let result = Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(ident),
            colon_token: Some(Default::default()),
            ty,
        };
        return result;
    }

    fn create_from_struct(&self, ident: Ident, item_struct: &ItemStruct) -> Field {
        let ty = self.type_factory.create_from_struct(item_struct);
        let result = self.create(ident, ty);
        return result;
    }

    fn create_pub_from_struct(&self, ident: Ident, item_struct: &ItemStruct) -> Field {
        let mut result = self.create_from_struct(ident, item_struct);
        result.vis = Visibility::Public(Default::default());
        return result;
    }
}
