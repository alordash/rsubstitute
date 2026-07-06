use super::models::*;
use crate::preparation::common::models::*;
use crate::preparation::r#fn::fn_syntax;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;
use quote::{format_ident, ToTokens};
use syn::*;

pub(crate) struct Params {
    pub attributes: Vec<Attribute>,
    pub unsafety: Option<Token![unsafe]>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub items: Vec<TraitItem>,
}

pub(crate) fn prepare(
    Params {
        attributes,
        unsafety,
        visibility,
        ident,
        generics,
        items,
    }: Params,
) -> TraitSyntax {
    let split_items = split_items(items, &ident);
    let trait_syntax_as_fn_owner = TraitSyntaxAsFnOwner {
        ident: &ident,
        generics: &generics,
    };
    let methods = split_items
        .fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| {
                fn_syntax::prepare(fn_syntax::Params {
                    attributes: x.attrs,
                    visibility: Visibility::Inherited,
                    signature: x.sig,
                    maybe_base_impl: None,
                    maybe_owner: Some(&trait_syntax_as_fn_owner),
                })
            })
        })
        .collect();
    let merged_generics = merge_generics_with_assoc_types(generics, &split_items.assoc_types);
    let path = path::from_ident_with_generics(ident.clone(), &merged_generics);

    let result = TraitSyntax {
        attributes,
        unsafety,
        visibility,
        ident,
        merged_generics,
        constants: split_items.constants,
        assoc_types: split_items.assoc_types,
        path,
        methods,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub constants: Vec<Ordered<TraitItemConstSyntax>>,
    pub assoc_types: Vec<Ordered<TraitItemTypeSyntax>>,
    pub fns: Vec<Ordered<TraitItemFn>>,
}
fn split_items(items: Vec<TraitItem>, trait_ident: &Ident) -> SplitItems {
    let mut split_items = SplitItems::default();
    for (order_number, item) in items.into_iter().enumerate() {
        match item {
            TraitItem::Const(trait_item_const) => split_items.constants.push(Ordered::new(
                order_number,
                TraitItemConstSyntax {
                    corresponding_generic_param_path: path::from_ident(format_ident!(
                        "{}_{}",
                        trait_ident,
                        trait_item_const.ident
                    )),
                    item: trait_item_const,
                },
            )),
            TraitItem::Fn(trait_item_fn) => split_items
                .fns
                .push(Ordered::new(order_number, trait_item_fn)),
            TraitItem::Type(trait_item_type) => split_items.assoc_types.push(Ordered::new(
                order_number,
                TraitItemTypeSyntax {
                    corresponding_generic_param_path: path::from_ident(format_ident!(
                        "{}_{}",
                        trait_ident,
                        trait_item_type.ident
                    )),
                    item: trait_item_type,
                },
            )),
            TraitItem::Macro(_) => todo!("macro invocations inside trait are not supported"),
            TraitItem::Verbatim(_) => todo!("verbatim trait items are not supported"),
            _ => panic!(
                "Unexpected trait item: {}",
                item.to_token_stream().to_string()
            ),
        }
    }

    return split_items;
}

fn merge_generics_with_assoc_types(
    mut generics: Generics,
    assoc_types: &[Ordered<TraitItemTypeSyntax>],
) -> Generics {
    let assoc_types_as_generic_parameters = assoc_types
        .iter()
        .map(|x| generic_param::from_type_ident(x.item.ident.clone()));
    generics.params.extend(assoc_types_as_generic_parameters);
    return generics;
}

struct TraitSyntaxAsFnOwner<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
}
impl<'a> IFnOwner for TraitSyntaxAsFnOwner<'a> {
    fn ident(&self) -> &Ident {
        self.ident
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }
}
