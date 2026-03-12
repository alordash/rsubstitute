use crate::syntax::*;
use proc_macro2::Ident;
use syn::*;
use crate::constants;

pub(crate) fn create(ident: Ident, ty: Type) -> Field {
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

pub(crate) fn create_from_struct(ident: Ident, item_struct: &ItemStruct) -> Field {
    let ty = r#type::create_from_struct(item_struct);
    let result = create(ident, ty);
    return result;
}

pub(crate) fn create_pub_from_struct(ident: Ident, item_struct: &ItemStruct) -> Field {
    let mut result = create_from_struct(ident, item_struct);
    result.vis = Visibility::Public(Default::default());
    return result;
}

pub(crate) fn is_phantom_data(field: &Field) -> bool {
    if let Type::Path(type_path) = &field.ty
        && let Some(first_path_segment) = type_path.path.segments.first()
        && first_path_segment.ident == constants::PHANTOM_DATA_IDENT.clone()
    {
        return true;
    }
    return false;
}