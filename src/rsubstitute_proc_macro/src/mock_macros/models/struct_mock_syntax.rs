use syn::parse::{Parse, ParseStream};
use syn::*;

pub(crate) struct StructMockSyntax {
    r#struct: ItemStruct,
    new_fn: ImplItemFn,
    trait_impls: Vec<ItemImpl>,
    struct_impls: Vec<ItemImpl>,
}

impl Parse for StructMockSyntax {
    fn parse(input: ParseStream) -> Result<Self> {
        let r#struct = input.call(ItemStruct::parse)?;
        let new_fn; // TODO - add service for parsing struct mock
        let mut trait_impls = Vec::new();
        let mut struct_impls = Vec::new();
        while !input.is_empty() {
            let item_impl = input.call(ItemImpl::parse)?;
            let Type::Path(type_path) = item_impl.self_ty.as_ref() else {
                panic!("{STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE}");
            };
            let type_ident = type_path.path.require_ident()?;
            if *type_ident != r#struct.ident {
                panic!("{STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE}");
            }
            if item_impl.trait_.is_some() {
                trait_impls.push(item_impl);
            } else {
                struct_impls.push(item_impl);
            }
        }
        todo!()
    }
}

pub const STRUCT_MOCK_INVALID_IDENT_ERROR_MESSAGE: &'static str =
    "Struct mock should contain only `impl` blocks for it's own type.";
