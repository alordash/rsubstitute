use crate::di::SERVICES;
use quote::ToTokens;
use std::fmt::{Debug, Formatter};
use syn::parse::*;
use syn::*;

pub(crate) struct StructMockSyntax {
    pub r#struct: ItemStruct,
    pub new_fn: ImplItemFn,
    pub trait_impls: Vec<ItemImpl>,
    pub struct_impls: Vec<ItemImpl>,
}

impl Parse for StructMockSyntax {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_mock_syntax_parser = &SERVICES.struct_mock_syntax_parser;
        return struct_mock_syntax_parser.parse(input);
    }
}

impl Debug for StructMockSyntax {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "StructMockSyntax {{ struct = {}, new_fn = {}, trait_impls = {:?}, struct_impls = {:?} }}",
            self.r#struct.to_token_stream().to_string(),
            self.new_fn.to_token_stream(),
            self.trait_impls
                .iter()
                .map(|x| x.to_token_stream().to_string())
                .collect::<Vec<_>>(),
            self.struct_impls
                .iter()
                .map(|x| x.to_token_stream().to_string())
                .collect::<Vec<_>>(),
        );
    }
}

impl StructMockSyntax {
    pub fn get_struct_fns(&self) -> Vec<&ImplItemFn> {
        return self
            .struct_impls
            .iter()
            .flat_map(|struct_impl| {
                struct_impl.items.iter().filter_map(|item| match item {
                    ImplItem::Fn(impl_item_fn) => Some(impl_item_fn),
                    _ => None,
                })
            })
            .collect();
    }
}
