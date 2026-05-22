use crate::models::r#fn::IFnOwner;
use crate::models::r#trait::*;
use crate::preparation::r#fn::{PrepareFnSyntaxArgs, prepare_fn_syntax};
use crate::syntax;
use quote::ToTokens;
use syn::*;

pub(crate) struct PrepareTraitFnSyntaxArgs {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub items: Vec<TraitItem>,
}

pub(crate) fn prepare_trait_syntax(
    PrepareTraitFnSyntaxArgs {
        attributes,
        visibility,
        ident,
        generics,
        items,
    }: PrepareTraitFnSyntaxArgs,
) -> TraitSyntax {
    let split_items = split_items(items);
    let trait_syntax_as_fn_owner = TraitSyntaxAsFnOwner {
        ident: &ident,
        generics: &generics,
    };
    let methods = split_items
        .fns
        .into_iter()
        .map(|x| {
            prepare_fn_syntax(PrepareFnSyntaxArgs {
                attributes: x.attrs,
                visibility: Visibility::Inherited,
                signature: x.sig,
                is_default: false,
                maybe_base_impl: None,
                maybe_owner: Some(&trait_syntax_as_fn_owner),
            })
        })
        .collect();
    let merged_generics = merge_generics_with_assoc_types(generics, &split_items.assoc_types);

    let result = TraitSyntax {
        attributes,
        visibility,
        ident,
        merged_generics,
        constants: split_items.constants,
        assoc_types: split_items.assoc_types,
        methods,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub constants: Vec<TraitItemConst>,
    pub assoc_types: Vec<TraitItemType>,
    pub fns: Vec<TraitItemFn>,
}
fn split_items(items: Vec<TraitItem>) -> SplitItems {
    let mut split_items = SplitItems::default();
    for item in items.into_iter() {
        match item {
            TraitItem::Const(trait_item_const) => split_items.constants.push(trait_item_const),
            TraitItem::Fn(trait_item_fn) => split_items.fns.push(trait_item_fn),
            TraitItem::Type(trait_item_type) => split_items.assoc_types.push(trait_item_type),
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
    assoc_types: &[TraitItemType],
) -> Generics {
    let assoc_types_as_generic_parameters = assoc_types
        .iter()
        .map(|x| syntax::generic_param::from_type_ident(x.ident.clone()));
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
