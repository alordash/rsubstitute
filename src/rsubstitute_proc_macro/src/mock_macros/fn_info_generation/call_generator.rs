use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::MockGenerics;
use crate::mock_macros::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::{ToTokens, format_ident};
use std::sync::Arc;
use syn::*;

pub trait ICallStructGenerator {
    fn generate(&self, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> CallStruct;
}

pub(crate) struct CallStructGenerator {
    pub field_factory: Arc<dyn IFieldFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl ICallStructGenerator for CallStructGenerator {
    fn generate(&self, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> CallStruct {
        let attrs = vec![
            constants::DOC_HIDDEN_ATTRIBUTE.clone(),
            constants::DERIVE_CLONE_ATTRIBUTE.clone(),
        ];
        let ident = format_ident!("{}_{}", fn_decl.get_full_ident(), Self::CALL_STRUCT_SUFFIX);
        let fn_fields = fn_decl
            .arguments
            .iter()
            .enumerate()
            .flat_map(|(i, x)| self.try_convert_fn_arg_to_field(i, x));
        let struct_fields = std::iter::once(constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD.clone())
            .chain(mock_generics.phantom_type_fields.clone())
            .chain(fn_fields)
            .collect();
        let fields_named = FieldsNamed {
            brace_token: Default::default(),
            named: struct_fields,
        };

        let mut item_struct = self.struct_factory.create(
            attrs,
            ident,
            mock_generics.impl_generics.clone(),
            fields_named,
        );
        self.reference_normalizer
            .normalize_anonymous_lifetimes_in_struct(&mut item_struct);
        let call_struct = CallStruct { item_struct };

        return call_struct;
    }
}

impl CallStructGenerator {
    pub const CALL_STRUCT_SUFFIX: &'static str = "Call";

    fn try_convert_fn_arg_to_field(&self, arg_number: usize, fn_arg: &FnArg) -> Option<Field> {
        let pat_type = match fn_arg {
            FnArg::Receiver(_) => return None,
            FnArg::Typed(pat_type) => pat_type,
        };
        let ty = match &*pat_type.ty {
            Type::Reference(reference) if reference.mutability.is_some() => Type::Ptr(TypePtr {
                star_token: Default::default(),
                const_token: None,
                mutability: Some(Default::default()),
                elem: reference.elem.clone(),
            }),
            rest => rest.clone(),
        };
        let ident = self.generate_field_ident(arg_number, pat_type);

        let result = self.field_factory.create(ident, ty);
        return Some(result);
    }

    fn generate_field_ident(&self, arg_number: usize, pat_type: &PatType) -> Ident {
        let result = match &*pat_type.pat {
            Pat::Ident(pat_ident) => pat_ident.ident.clone(),
            _ => format_ident!("arg_{arg_number}"),
        };
        return result;
    }
}
