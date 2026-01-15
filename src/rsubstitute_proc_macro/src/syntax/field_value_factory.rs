use crate::constants;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IFieldValueFactory {
    fn create_with_into_conversion(&self, field: &Field) -> FieldValue;
}

pub(crate) struct FieldValueFactory {
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
}

impl IFieldValueFactory for FieldValueFactory {
    fn create_with_into_conversion(&self, field: &Field) -> FieldValue {
        let field_ident = field
            .ident
            .clone()
            .expect("TODO: Field in call struct should be named");
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
}
