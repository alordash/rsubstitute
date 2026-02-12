use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IInnerDataDerefImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        inner_data_struct: &InnerDataStruct,
    ) -> InnerDataDerefImpl;
}

pub(crate) struct InnerDataDerefImplGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
    pub field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
    pub expr_reference_factory: Arc<dyn IExprReferenceFactory>,
}

impl IInnerDataDerefImplGenerator for InnerDataDerefImplGenerator {
    fn generate(
        &self,
        mock_struct: &MockStruct,
        inner_data_struct: &InnerDataStruct,
    ) -> InnerDataDerefImpl {
        let self_ty = self
            .type_factory
            .create_from_struct(&mock_struct.item_struct);
        let inner_data_type = self
            .type_factory
            .create_from_struct(&inner_data_struct.item_struct);
        let target_type_item = self.generate_target_type_item(inner_data_type.clone());
        let deref_fn_item = self.generate_deref_fn_item();
        let items = vec![target_type_item, deref_fn_item];
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: mock_struct.item_struct.generics.clone(),
            trait_: Some((
                None,
                constants::DEREF_TRAIT_PATH.clone(),
                Default::default(),
            )),
            self_ty: Box::new(self_ty),
            brace_token: Default::default(),
            items,
        };
        let inner_data_deref_impl = InnerDataDerefImpl { item_impl };
        return inner_data_deref_impl;
    }
}

impl InnerDataDerefImplGenerator {
    fn generate_target_type_item(&self, ty: Type) -> ImplItem {
        let target_type_item = ImplItemType {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            type_token: Default::default(),
            ident: constants::DEREF_TARGET_TYPE_IDENT.clone(),
            generics: Generics::default(),
            eq_token: Default::default(),
            ty,
            semi_token: Default::default(),
        };
        return ImplItem::Type(target_type_item);
    }

    fn generate_deref_fn_item(&self) -> ImplItem {
        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: None,
            abi: None,
            fn_token: Default::default(),
            ident: constants::DEREF_FN_IDENT.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(self.type_factory.reference(
                    Type::Path(TypePath {
                        qself: None,
                        path: self.path_factory.create_from_parts(vec![
                            constants::SELF_TYPE_IDENT.clone(),
                            constants::DEREF_TARGET_TYPE_IDENT.clone(),
                        ]),
                    }),
                    None,
                )),
            ),
        };
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![Stmt::Expr(
                self.expr_reference_factory
                    .create(self.field_access_expr_factory.create(vec![
                        constants::SELF_IDENT.clone(),
                        constants::INNER_DATA_FIELD_IDENT.clone(),
                    ])),
                None,
            )],
        };
        let deref_fn_item = ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig,
            block,
        };
        return ImplItem::Fn(deref_fn_item);
    }
}
