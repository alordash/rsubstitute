use crate::syntax::IPathFactory;
use std::rc::Rc;
use syn::{Expr, ExprPath, Field, FieldValue, Member};

pub trait IFieldValueFactory {
    fn create(&self, field: &Field) -> FieldValue;
}

pub struct FieldValueFactory {
    pub(crate) path_factory: Rc<dyn IPathFactory>,
}

impl IFieldValueFactory for FieldValueFactory {
    fn create(&self, field: &Field) -> FieldValue {
        let field_ident = field
            .ident
            .clone()
            .expect("Field in call struct should be named");
        let field_value = FieldValue {
            attrs: Vec::new(),
            member: Member::Named(field_ident.clone()),
            colon_token: None,
            expr: Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: self.path_factory.create(field_ident),
            }),
        };

        return field_value;
    }
}
