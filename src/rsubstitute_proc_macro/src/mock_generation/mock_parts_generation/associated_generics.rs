use crate::mock_generation::mock_parts_generation::models::*;
use quote::format_ident;
use syn::*;

pub(crate) fn extract(parent_ident: &Ident, impl_items: &[TraitItem]) -> AssociatedGenerics {
    let mut trait_item_consts = Vec::new();
    let mut trait_item_consts_source_idents = Vec::new();
    let mut trait_item_types = Vec::new();
    let mut trait_item_types_source_idents = Vec::new();
    let generics_params: Vec<_> = impl_items
        .iter()
        .flat_map(|x| match &x {
            TraitItem::Const(trait_item_const) => {
                trait_item_consts.push(trait_item_const.clone());
                trait_item_consts_source_idents.push(trait_item_const.ident.clone());
                Some(extract_item_const_associated_generics(
                    parent_ident,
                    &trait_item_const,
                ))
            }
            TraitItem::Type(trait_item_type) => {
                trait_item_types.push(trait_item_type.clone());
                trait_item_types_source_idents.push(trait_item_type.ident.clone());
                Some(extract_item_type_associated_generics(
                    parent_ident,
                    &trait_item_type,
                ))
            }
            _ => None,
        })
        .collect();
    let assoc_types_generics = AssociatedGenerics {
        trait_item_consts,
        trait_item_consts_source_idents,
        trait_item_types,
        trait_item_types_source_idents,
        generics_params,
    };
    return assoc_types_generics;
}

fn format_associated_param_ident(parent_ident: &Ident, ident: &Ident) -> Ident {
    format_ident!("{parent_ident}_{ident}")
}

fn extract_item_const_associated_generics(
    parent_ident: &Ident,
    trait_item_const: &TraitItemConst,
) -> GenericParam {
    let const_param = ConstParam {
        attrs: trait_item_const.attrs.clone(),
        const_token: Default::default(),
        ident: format_associated_param_ident(parent_ident, &trait_item_const.ident),
        colon_token: Default::default(),
        ty: trait_item_const.ty.clone(),
        eq_token: None,
        default: None,
    };
    return GenericParam::Const(const_param);
}

fn extract_item_type_associated_generics(
    parent_ident: &Ident,
    trait_item_type: &TraitItemType,
) -> GenericParam {
    let type_param = TypeParam {
        attrs: trait_item_type.attrs.clone(), // TODO - copy all attributes EVERYWHERE!
        ident: format_associated_param_ident(parent_ident, &trait_item_type.ident),
        colon_token: trait_item_type.colon_token.clone(),
        bounds: trait_item_type.bounds.clone(),
        eq_token: None,
        default: None,
    };
    return GenericParam::Type(type_param);
}
