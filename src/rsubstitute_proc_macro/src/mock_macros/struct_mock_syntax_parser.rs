use crate::constants;
use crate::mock_macros::models::StructMockSyntax;
use quote::format_ident;
use std::cell::LazyCell;
use syn::parse::*;
use syn::*;

pub trait IStructMockSyntaxParser {
    fn parse(&self, input: ParseStream) -> Result<StructMockSyntax>;
}

pub struct StructMockSyntaxParser;

impl IStructMockSyntaxParser for StructMockSyntaxParser {
    fn parse(&self, input: ParseStream) -> Result<StructMockSyntax> {
        let r#struct = input.call(ItemStruct::parse)?;
        let mut maybe_new_fn = None;
        let mut trait_impls = Vec::new();
        let mut struct_impls = Vec::new();
        while !input.is_empty() {
            let item_impl = input.call(ItemImpl::parse)?;
            let Type::Path(type_path) = item_impl.self_ty.as_ref() else {
                panic!("{}", Self::STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE);
            };
            let type_ident = type_path.path.require_ident()?;
            if *type_ident != r#struct.ident {
                panic!("{}", Self::STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE);
            }
            if item_impl.trait_.is_some() {
                trait_impls.push(item_impl);
            } else {
                if maybe_new_fn.is_none() {
                    maybe_new_fn = self.try_extract_new_fn(&r#struct.ident, &item_impl);
                }
                struct_impls.push(item_impl);
            }
        }
        let Some(new_fn) = maybe_new_fn else {
            panic!("{}", Self::NO_NEW_FN_ERROR_MESSAGE);
        };
        let struct_mock_syntax = StructMockSyntax {
            r#struct,
            new_fn,
            trait_impls,
            struct_impls,
        };
        return Ok(struct_mock_syntax);
    }
}

impl StructMockSyntaxParser {
    const STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE: &'static str =
        "Struct mock should contain only `impl` blocks for it's own type.";
    const NO_NEW_FN_ERROR_MESSAGE: &'static str = "In order to be mockable structure must have function `pub fn new(args) -> Self`, where `args` is arbitrary collection of user-defined arguments.";
    const NEW_FN_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("new"));

    fn try_extract_new_fn(&self, struct_ident: &Ident, item_impl: &ItemImpl) -> Option<ImplItemFn> {
        let maybe_new_fn = item_impl
            .items
            .iter()
            .filter_map(|item| match item {
                ImplItem::Fn(impl_item_fn)
                    if self.is_fn_impl_item_fn_is_new_fn(struct_ident, impl_item_fn) =>
                {
                    Some(impl_item_fn.clone())
                }
                _ => None,
            })
            .next();
        return maybe_new_fn;
    }

    fn is_fn_impl_item_fn_is_new_fn(
        &self,
        struct_ident: &Ident,
        impl_item_fn: &ImplItemFn,
    ) -> bool {
        if impl_item_fn.sig.ident == Self::NEW_FN_IDENT.clone()
            && let Visibility::Public(_) = impl_item_fn.vis
            && let ReturnType::Type(_, return_type) = &impl_item_fn.sig.output
            && let Type::Path(type_path) = &**return_type
            && let Some(type_ident) = type_path.path.get_ident()
            && (type_ident == struct_ident || type_ident == &constants::SELF_TYPE_IDENT.clone())
        {
            return true;
        }
        return false;
    }
}
