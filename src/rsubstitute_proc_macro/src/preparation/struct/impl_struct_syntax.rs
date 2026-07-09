use super::models::*;
use crate::preparation::r#fn::models::IFnOwner;
use crate::preparation::r#fn::*;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::*;

pub(crate) struct Params {
    pub attributes: Vec<Attribute>,
    pub generics: Generics,
    pub target_type: Box<Type>,
    pub impl_items: Vec<ImplItem>,
}

pub(crate) fn prepare(
    Params {
        attributes,
        generics,
        target_type,
        impl_items,
    }: Params,
) -> ImplStructSyntax {
    let SplitItems {
        constants,
        assoc_types,
        fns,
    } = split_items(impl_items);
    let SplitTargetType {
        modules,
        target_ident,
    } = split_target_type(&target_type);
    let impl_struct_syntax_as_fn_owner = ImplStructSyntaxAsFnOwner {
        ident: &target_ident,
        generics: &generics,
    };
    let methods = fns
        .into_iter()
        .map(|x| {
            fn_syntax::prepare(fn_syntax::Params {
                attributes: x.attrs,
                visibility: Visibility::Inherited,
                signature: x.sig,
                maybe_base_impl: None,
                maybe_owner: Some(&impl_struct_syntax_as_fn_owner),
            })
        })
        .collect();

    let result = ImplStructSyntax {
        attributes,
        modules,
        target_ident,
        generics,
        target_type: *target_type,
        constants,
        assoc_types,
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

struct SplitTargetType {
    pub modules: Vec<Ident>,
    pub target_ident: Ident,
}
fn split_target_type(target_type: &Type) -> SplitTargetType {
    let Type::Path(type_path) = target_type else {
        panic!("Can mock only `impl`s of structs.");
    };
    if type_path.qself.is_some() {
        panic!("Can not mock structs qualified with self-type.");
    }

    let modules: Vec<_> = type_path
        .path
        .segments
        .iter()
        .take(type_path.path.segments.len() - 1)
        .map(|x| x.ident.clone())
        .collect();
    let target_ident = type_path
        .path
        .segments
        .last()
        .expect("`impl` struct target type path should not be empty.")
        .ident
        .clone();
    let result = SplitTargetType {
        modules,
        target_ident,
    };
    return result;
}

struct ImplStructSyntaxAsFnOwner<'a> {
    pub ident: &'a Ident,
    pub generics: &'a Generics,
}
impl<'a> IFnOwner for ImplStructSyntaxAsFnOwner<'a> {
    fn maybe_ident(&self) -> Option<&Ident> {
        Some(self.ident)
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }
}
