use crate::common::models::*;
use crate::generation::targets::common::*;
use crate::generation::targets::models::*;
use crate::generation::targets::*;
use crate::generation::*;
use crate::preparation::r#struct::*;
use crate::syntax::*;
use quote::format_ident;
use syn::spanned::Spanned;
use syn::*;

pub(crate) fn generate_module(ctx: &Context, mut item_impl: ItemImpl) -> MockMod {
    let source_span = item_impl.span();
    let Some((_, trait_path, _)) = item_impl.trait_ else {
        panic!("When mocking trait implementation `item_impl` must have trait field.")
    };
    let impl_trait_for_struct_syntax =
        impl_trait_for_struct_syntax::prepare(impl_trait_for_struct_syntax::Params {
            attributes: item_impl.attrs,
            generics: item_impl.generics,
            target_type: item_impl.self_ty,
            trait_path,
            impl_items: item_impl.items,
        });
    let impl_trait_for_struct_info =
        impl_trait_for_struct_info::generate(ctx, impl_trait_for_struct_syntax);

    let mock_mod_usages = mock_mod_usages::new(source_span);
    let items = [
        Item::Use(mock_mod_usages.use_rsubstitute_for_generated),
        Item::Use(mock_mod_usages.use_super),
    ]
    .into_iter()
    .collect();
    let call_site = proc_macro::Span::call_site();
    let line = call_site.line();
    let column = call_site.column();
    let mod_ident = format_ident!(
        "__rsubstitute_generated_{}_{}_{}",
        impl_trait_for_struct_info.target_ident,
        line,
        column
    );
    let usage = mod_usage::new_all(mod_ident.clone());
    let item_mod = ItemMod {
        attrs: vec![attributes::allow_non_camel_case_types(source_span)],
        vis: Visibility::Public(Token![pub](source_span)),
        unsafety: None,
        mod_token: Token![mod](source_span),
        ident: mod_ident,
        content: Some((token::Brace(source_span), items)),
        semi: None,
    };
    let result = MockMod { usage, item_mod };
    return result;
}
