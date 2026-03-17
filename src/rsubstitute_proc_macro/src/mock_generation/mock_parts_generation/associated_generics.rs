use crate::mock_generation::mock_parts_generation::models::*;
use syn::*;

pub(crate) fn extract(impl_items: &[TraitItem]) -> AssociatedGenerics {
    let mut trait_item_consts = Vec::new();
    let mut trait_item_types = Vec::new();
    let generics_params: Vec<_> = impl_items
        .iter()
        .flat_map(|x| match &x {
            TraitItem::Const(trait_item_const) => {
                trait_item_consts.push(trait_item_const.clone());
                Some(extract_item_const_associated_generics(&trait_item_const))
            }
            TraitItem::Type(trait_item_type) => {
                trait_item_types.push(trait_item_type.clone());
                Some(extract_item_type_associated_generics(&trait_item_type))
            }
            _ => None,
        })
        .collect();
    let assoc_types_generics = AssociatedGenerics {
        trait_item_consts,
        trait_item_types,
        generics_params,
    };
    return assoc_types_generics;
}

fn extract_item_const_associated_generics(trait_item_const: &TraitItemConst) -> GenericParam {
    let (eq_token, default) = match trait_item_const.default {
        None => (None, None),
        Some(ref assoc_type_default) => (
            Some(assoc_type_default.0.clone()),
            Some(assoc_type_default.1.clone()),
        ),
    };
    let const_param = ConstParam {
        attrs: trait_item_const.attrs.clone(),
        const_token: Default::default(),
        ident: trait_item_const.ident.clone(),
        colon_token: Default::default(),
        ty: trait_item_const.ty.clone(),
        eq_token,
        default,
    };
    return GenericParam::Const(const_param);
}

fn extract_item_type_associated_generics(trait_item_type: &TraitItemType) -> GenericParam {
    let (eq_token, default) = match trait_item_type.default {
        None => (None, None),
        Some(ref assoc_type_default) => (
            Some(assoc_type_default.0.clone()),
            Some(assoc_type_default.1.clone()),
        ),
    };
    let type_param = TypeParam {
        attrs: trait_item_type.attrs.clone(), // TODO - copy all attributes EVERYWHERE!
        ident: trait_item_type.ident.clone(),
        colon_token: trait_item_type.colon_token.clone(),
        bounds: trait_item_type.bounds.clone(),
        eq_token,
        default,
    };
    return GenericParam::Type(type_param);
}
