use crate::models::r#fn::IFnOwner;
use crate::models::r#struct::*;
use crate::preparation::r#fn::{prepare_fn_syntax, PrepareFnSyntaxArgs};
use crate::syntax::*;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::*;

pub(crate) struct PrepareImplStructSyntaxArgs {
    pub attributes: Vec<Attribute>,
    pub generics: Generics,
    pub self_ty: Box<Type>,
    pub impl_items: Vec<ImplItem>,
}

pub(crate) fn prepare_impl_struct_syntax(
    PrepareImplStructSyntaxArgs {
        attributes,
        generics,
        self_ty,
        impl_items,
    }: PrepareImplStructSyntaxArgs,
) -> ImplStructSyntax {
    let split_items = split_items(impl_items);
    let ident = r#type::to_ident(&self_ty);
    let impl_struct_syntax_as_fn_owner = ImplStructSyntaxAsFnOwner {
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
                maybe_owner: Some(&impl_struct_syntax_as_fn_owner),
            })
        })
        .collect();

    let result = ImplStructSyntax {
        attributes,
        target_ident: ident,
        generics,
        target_type: *self_ty,
        constants: split_items.constants,
        assoc_types: split_items.assoc_types,
        methods,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub constants: Vec<ImplItemConst>,
    pub assoc_types: Vec<ImplItemType>,
    pub fns: Vec<ImplItemFn>,
}
fn split_items(items: Vec<ImplItem>) -> SplitItems {
    let mut split_items = SplitItems::default();
    for item in items.into_iter() {
        match item {
            ImplItem::Const(trait_item_const) => split_items.constants.push(trait_item_const),
            ImplItem::Fn(trait_item_fn) => split_items.fns.push(trait_item_fn),
            ImplItem::Type(trait_item_type) => split_items.assoc_types.push(trait_item_type),
            ImplItem::Macro(_) => todo!("macro invocations inside impl blocks are not supported"),
            ImplItem::Verbatim(_) => todo!("verbatim impl items are not supported"),
            _ => panic!(
                "Unexpected impl item: {}",
                item.to_token_stream().to_string()
            ),
        }
    }

    return split_items;
}

struct ImplStructSyntaxAsFnOwner<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
}
impl<'a> IFnOwner for ImplStructSyntaxAsFnOwner<'a> {
    fn ident(&self) -> &Ident {
        self.ident
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }
}
