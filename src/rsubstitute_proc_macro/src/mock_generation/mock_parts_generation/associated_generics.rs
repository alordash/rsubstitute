use crate::mock_generation::mock_parts_generation::models::*;
use quote::format_ident;
use syn::*;

pub(crate) fn extract(parent_ident: &Ident, impl_items: &[TraitItem]) -> AssociatedGenerics {
    let mut trait_items = Vec::new();
    let generics_params: Vec<_> = impl_items
        .iter()
        .flat_map(|x| match &x {
            TraitItem::Const(trait_item_const) => {
                trait_items.push(TraitItem::Const(trait_item_const.clone()));
                let const_param =
                    extract_item_const_associated_generics(parent_ident, &trait_item_const);
                Some(GenericParam::Const(const_param))
            }
            TraitItem::Type(trait_item_type) => {
                trait_items.push(TraitItem::Type(trait_item_type.clone()));
                let type_param =
                    extract_item_type_associated_generics(parent_ident, &trait_item_type);
                Some(GenericParam::Type(type_param))
            }
            _ => None,
        })
        .collect();
    let assoc_types_generics = AssociatedGenerics {
        parent_ident: parent_ident.clone(),
        trait_items,
        generics_params,
    };
    return assoc_types_generics;
}

pub(crate) fn format_associated_param_ident(parent_ident: &Ident, ident: &Ident) -> Ident {
    format_ident!("{parent_ident}_{ident}")
}

fn extract_item_const_associated_generics(
    parent_ident: &Ident,
    trait_item_const: &TraitItemConst,
) -> ConstParam {
    let const_param = ConstParam {
        attrs: trait_item_const.attrs.clone(),
        const_token: Default::default(),
        ident: format_associated_param_ident(parent_ident, &trait_item_const.ident),
        colon_token: Default::default(),
        ty: trait_item_const.ty.clone(),
        eq_token: None,
        default: None,
    };
    return const_param;
}

fn extract_item_type_associated_generics(
    parent_ident: &Ident,
    trait_item_type: &TraitItemType,
) -> TypeParam {
    let type_param = TypeParam {
        attrs: trait_item_type.attrs.clone(), // TODO - copy all attributes EVERYWHERE!
        ident: format_associated_param_ident(parent_ident, &trait_item_type.ident),
        colon_token: trait_item_type.colon_token.clone(),
        bounds: trait_item_type.bounds.clone(),
        eq_token: None,
        default: None,
    };
    return type_param;
}
