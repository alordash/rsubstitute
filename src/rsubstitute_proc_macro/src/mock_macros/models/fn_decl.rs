use crate::constants;
use proc_macro2::Ident;
use quote::format_ident;
use syn::*;

pub(crate) struct FnDecl {
    pub attrs: Vec<Attribute>,
    pub maybe_parent_trait_ident: Option<Ident>,
    pub fn_ident: Ident,
    pub arguments: Vec<FnArg>,
    pub return_value: ReturnType,
    pub own_generics: Generics,
    pub merged_generics: Generics,
    pub visibility: Visibility,
    pub maybe_base_fn_block: Option<Block>,
    pub base_callable: bool, // TODO - set actual value
    pub maybe_phantom_return_field: Option<Field>,
    pub arg_refs_tuple: Type
}

impl FnDecl {
    pub fn get_full_ident(&self) -> Ident {
        if let Some(trait_prefix) = &self.maybe_parent_trait_ident {
            return format_ident!("{}_{}", trait_prefix, self.fn_ident);
        }
        return self.fn_ident.clone();
    }

    pub fn get_str_literal_full_ident(&self) -> String {
        if let Some(trait_prefix) = &self.maybe_parent_trait_ident {
            return format!("{}::{}", trait_prefix, self.fn_ident);
        }
        return self.fn_ident.to_string();
    }

    pub fn get_return_value_type(&self) -> Type {
        match &self.return_value {
            ReturnType::Default => constants::VOID_TYPE.clone(),
            ReturnType::Type(_, boxed_type) => *boxed_type.clone(),
        }
    }

    pub fn has_return_value(&self) -> bool {
        match self.return_value {
            ReturnType::Default => false,
            ReturnType::Type(_, _) => true,
        }
    }

    pub fn get_internal_phantom_types_count(&self) -> usize {
        if self.maybe_phantom_return_field.is_some() {
            2
        } else {
            1
        }
    }

    // pub fn get_internal_phantom_types_count_without_return_type(&self) -> usize {
    //     1
    // }
    pub fn get_internal_phantom_types_count_without_return_type(&self) -> usize {
        self.get_internal_phantom_types_count()
    }
}
