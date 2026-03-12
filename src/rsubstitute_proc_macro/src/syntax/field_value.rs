use crate::constants;
use crate::syntax::field_required_ident_extension::IFieldRequiredIdentExtension;
use crate::syntax::*;
use syn::*;

pub(crate) fn create_with_into_conversion(field: &Field) -> FieldValue {
    let field_ident = field.get_required_ident();
    let field_value = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(field_ident.clone()),
        colon_token: Some(Default::default()),
        expr: Expr::MethodCall(expr_method_call::create(
            vec![field_ident],
            constants::INTO_FN_IDENT.clone(),
            Vec::new(),
        )),
    };

    return field_value;
}

pub(crate) fn create_as_phantom_data(field_ident: Ident) -> FieldValue {
    let field_value = FieldValue {
        attrs: Vec::new(),
        member: Member::Named(field_ident),
        colon_token: Some(Default::default()),
        expr: constants::PHANTOM_DATA_EXPR_PATH.clone(),
    };
    return field_value;
}
