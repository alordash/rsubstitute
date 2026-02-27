use crate::constants;
use crate::syntax::*;
use proc_macro::TokenStream;
use quote::ToTokens;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IDeriveCloneForRSubstituteMacroHandler {
    fn handle(&self, item: proc_macro::TokenStream) -> proc_macro::TokenStream;
}

pub(crate) struct DeriveCloneForRSubstituteMacroHandler {
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub field_access_expr_factory: Arc<dyn IFieldAccessExprFactory>,
    pub expr_reference_factory: Arc<dyn IExprReferenceFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
}

impl IDeriveCloneForRSubstituteMacroHandler for DeriveCloneForRSubstituteMacroHandler {
    fn handle(&self, item: TokenStream) -> TokenStream {
        let item_struct = parse_macro_input!(item as ItemStruct);

        let trait_path = self
            .path_factory
            .create(constants::CLONE_TRAIT_IDENT.clone());
        let self_ty = Box::new(self.type_factory.create_from_struct(&item_struct));
        let get_arg_infos_fn = self.generate_clone_fn(&item_struct);
        // TODO - make factory of ItemImpl
        let item_impl = ItemImpl {
            attrs: Vec::new(),
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: item_struct.generics.clone(),
            trait_: Some((None, trait_path, Default::default())),
            self_ty,
            brace_token: Default::default(),
            items: vec![get_arg_infos_fn],
        };
        return item_impl.into_token_stream().into();
    }
}

impl DeriveCloneForRSubstituteMacroHandler {
    fn generate_clone_fn(&self, item_struct: &ItemStruct) -> ImplItem {
        let return_stmt = self.generate_return_stmt(item_struct);
        let block = Block {
            brace_token: Default::default(),
            stmts: vec![return_stmt],
        };
        let impl_item = ImplItem::Fn(ImplItemFn {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            defaultness: None,
            sig: constants::CLONE_FN_SIGNATURE.clone(),
            block,
        });
        return impl_item;
    }

    fn generate_return_stmt(&self, item_struct: &ItemStruct) -> Stmt {
        let fields: Punctuated<_, Token![,]> = item_struct
            .fields
            .iter()
            .map(|field| self.generate_field_value(field))
            .collect();

        let self_expr = Expr::Struct(ExprStruct {
            attrs: Vec::new(),
            qself: None,
            path: constants::SELF_TYPE_PATH.clone(),
            brace_token: Default::default(),
            fields,
            dot2_token: None,
            rest: None,
        });
        let stmt = Stmt::Expr(self_expr, None);
        return stmt;
    }

    fn generate_field_value(&self, field: &Field) -> FieldValue {
        let field_ident = field
            .ident
            .clone()
            .expect("Call struct fields should have ident.");
        let field_clone_expr = self.expr_method_call_factory.create_with_base_receiver(
            Expr::Paren(ExprParen {
                attrs: Vec::new(),
                paren_token: Default::default(),
                expr: Box::new(
                    self.expr_reference_factory.create(
                        self.field_access_expr_factory
                            .create(vec![constants::SELF_IDENT.clone(), field_ident.clone()]),
                    ),
                ),
            }),
            Vec::new(),
            constants::CLONE_FN_IDENT.clone(),
            Vec::new(),
        );
        let field_value = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(field_ident),
            colon_token: Some(Default::default()),
            expr: Expr::MethodCall(field_clone_expr),
        };
        return field_value;
    }
}
