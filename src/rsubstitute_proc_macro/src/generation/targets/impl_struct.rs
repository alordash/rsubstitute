use quote::format_ident;
use crate::common::models::*;
use crate::generation::targets::models::*;
use crate::preparation::r#struct::*;
use crate::preparation::r#struct::models::*;
use syn::spanned::Spanned;
use syn::*;
use crate::generation::targets::*;
use crate::generation::targets::common::*;
use crate::syntax::attributes;

pub(crate) fn generate_module(ctx: &Context, item_impl: ItemImpl) -> MockMod {
    let source_span = item_impl.span();
    let impl_struct_syntax = impl_struct_syntax::prepare(impl_struct_syntax::Params {
        attributes: item_impl.attrs.clone(),
        generics: item_impl.generics.clone(),
        target_type: item_impl.self_ty.clone(),
        impl_items: item_impl.items.clone(),
    });
    
    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [Item::Use(mock_mod_usages.use_rsubstitute_for_generated),
    Item::Use(mock_mod_usages.use_super)].into_iter().collect();
    let mod_ident = format_ident!("__rsubstitute_generated_{}Mock", impl_struct_syntax.target_ident);
    let usage = mod_usage::new(mod_ident.clone(), [impl_struct_syntax.target_ident.clone()]);
    let item_mod = ItemMod {
        attrs: vec![attributes::allow_non_camel_case_types(source_span)],
        vis: Visibility::Public(Token![pub](source_span)),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None
    };
    let result = MockMod {usage, item_mod};
    return result;
}
 