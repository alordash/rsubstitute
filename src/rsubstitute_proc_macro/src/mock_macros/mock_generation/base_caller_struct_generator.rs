use crate::constants;
use crate::mock_macros::mock_generation::models::BaseCallerStruct;
use crate::mock_macros::models::*;
use quote::format_ident;
use syn::*;

pub trait IBaseCallerStructGenerator {
    fn generate(&self, fn_decl: &FnDecl) -> BaseCallerStruct;
}

pub(crate) struct BaseCallerStructGenerator;

impl IBaseCallerStructGenerator for BaseCallerStructGenerator {
    fn generate(&self, fn_decl: &FnDecl) -> BaseCallerStruct {
        let ident = format_ident!("{}{}", fn_decl.ident, Self::IDENT_SUFFIX);
        let item_struct = ItemStruct {
            attrs: vec![constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone()],
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Generics::default(),
            fields: Fields::Unit,
            semi_token: Some(Default::default()),
        };
        let base_caller_struct = BaseCallerStruct { item_struct };
        return base_caller_struct;
    }
}

impl BaseCallerStructGenerator {
    const IDENT_SUFFIX: &'static str = "BaseCaller";
}
