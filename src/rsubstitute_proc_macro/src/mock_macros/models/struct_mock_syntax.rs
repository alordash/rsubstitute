use crate::di::SERVICES;
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
