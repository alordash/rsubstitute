use super::models::*;
use crate::common::normalization;
use crate::preparation::r#fn::models::*;
use crate::preparation::r#fn::*;
use crate::preparation::models::*;
use crate::syntax::*;
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
        mut impl_items,
    }: Params,
) -> ImplStructSyntax {
    let target_path = parse_target_type(*target_type);
    let target_ident = target_path
        .segments
        .last()
        .expect("`impl` target path can not be empty")
        .ident
        .clone();
    impl_items = impl_items
        .into_iter()
        .map(|x| normalization::normalize_struct_type_references(x, &target_path))
        .collect();
    let split_items = split_items(impl_items);
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
        target_ident,
        target_path,
        generics,
        static_fns,
        associated_fns,
    };
    return result;
}

#[derive(Default)]
struct SplitItems {
    pub static_fns: Vec<Ordered<ImplItemFn>>,
    pub associated_fns: Vec<Ordered<ImplItemFn>>,
}
fn split_items(items: Vec<ImplItem>) -> SplitItems {
    let mut split_items = SplitItems::default();
    for (order_number, item) in items.into_iter().enumerate() {
        match item {
            ImplItem::Const(_) => {}
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

fn parse_target_type(target_type: Type) -> Path {
    let Type::Path(type_path) = target_type else {
        panic!("Can mock only `impl`s of structs.");
    };
    if type_path.qself.is_some() {
        panic!("Can not mock structs qualified with self-type.");
    }

    return type_path.path;
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
