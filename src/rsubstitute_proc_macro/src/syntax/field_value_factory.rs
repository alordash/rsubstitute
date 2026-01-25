use crate::constants;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IFieldValueFactory {
    fn create_with_into_conversion(&self, field: &Field) -> FieldValue;

    fn create_as_phantom_data(&self, field_ident: Ident) -> FieldValue;
}

pub(crate) struct FieldValueFactory {
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
}

impl IFieldValueFactory for FieldValueFactory {
    fn create_with_into_conversion(&self, field: &Field) -> FieldValue {
        let field_ident = field.get_required_ident();
        let field_value = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(field_ident.clone()),
            colon_token: Some(Default::default()),
            expr: Expr::MethodCall(self.expr_method_call_factory.create(
                vec![field_ident],
                constants::INTO_FN_IDENT.clone(),
                Vec::new(),
            )),
        };

        return field_value;
    }

    fn create_as_phantom_data(&self, field_ident: Ident) -> FieldValue {
        let field_value = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(field_ident),
            colon_token: Some(Default::default()),
            expr: constants::PHANTOM_DATA_EXPR_PATH.clone(),
        };
        return field_value;
    }
}
