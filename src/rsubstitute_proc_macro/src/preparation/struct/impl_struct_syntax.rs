use super::models::*;
use crate::preparation::models::*;
use crate::preparation::r#fn::models::{FnSyntax, IFnOwner};
use crate::preparation::r#fn::*;
use crate::syntax::signature;
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
    let split_items = split_items(impl_items);
    let SplitTargetType {
        modules,
        target_ident,
    } = split_target_type(&target_type);
    let impl_struct_syntax_as_fn_owner = ImplStructSyntaxAsFnOwner {
        generics: &generics,
    };
    let static_fns = split_items
        .static_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| map_impl_item_fn_to_fn_syntax(x, &impl_struct_syntax_as_fn_owner))
        })
        .collect();
    let associated_fns = split_items
        .associated_fns
        .into_iter()
        .map(|ordered| {
            ordered.map(|x| map_impl_item_fn_to_fn_syntax(x, &impl_struct_syntax_as_fn_owner))
        })
        .collect();

    let result = ImplStructSyntax {
        attributes,
        modules,
        target_ident,
        generics,
        target_type: *target_type,
        constants: split_items.constants,
        static_fns,
        associated_fns,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub constants: Vec<Ordered<ImplItemConst>>,
    pub static_fns: Vec<Ordered<ImplItemFn>>,
    pub associated_fns: Vec<Ordered<ImplItemFn>>,
}
fn split_items(items: Vec<ImplItem>) -> SplitItems {
    let mut split_items = SplitItems::default();
    for (order_number, item) in items.into_iter().enumerate() {
        match item {
            ImplItem::Const(impl_item_const) => split_items
                .constants
                .push(Ordered::new(order_number, impl_item_const)),
            ImplItem::Fn(impl_item_fn) => {
                if signature::is_associated(&impl_item_fn.sig) {
                    split_items
                        .associated_fns
                        .push(Ordered::new(order_number, impl_item_fn))
                } else {
                    split_items
                        .static_fns
                        .push(Ordered::new(order_number, impl_item_fn))
                }
            }
            ImplItem::Type(_) => panic!("Inherent associated types are not supported"), // feature `inherent_associated_types`
            ImplItem::Macro(_) => panic!("Macro invocations inside impl blocks are not supported"),
            _ => panic!(
                "Unexpected impl item: {}",
                item.to_token_stream().to_string()
            ),
        }
    }

    return split_items;
}

fn map_impl_item_fn_to_fn_syntax(
    impl_item_fn: ImplItemFn,
    impl_struct_syntax_as_fn_owner: &ImplStructSyntaxAsFnOwner,
) -> FnSyntax {
    let result = fn_syntax::prepare(fn_syntax::Params {
        attributes: impl_item_fn.attrs,
        visibility: impl_item_fn.vis,
        signature: impl_item_fn.sig,
        maybe_base_impl: Some(Box::new(impl_item_fn.block)),
        maybe_owner: Some(impl_struct_syntax_as_fn_owner),
    });
    return result;
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
    pub generics: &'a Generics,
}
impl<'a> IFnOwner for ImplStructSyntaxAsFnOwner<'a> {
    fn maybe_ident(&self) -> Option<&Ident> {
        None
    }

    fn generics(&self) -> &Generics {
        &self.generics
    }
}
