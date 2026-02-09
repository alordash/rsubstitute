use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::{ToTokens, format_ident};
use std::sync::Arc;
use syn::*;

pub trait IArgsCheckerGenerator {
    fn generate(&self, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> ArgsCheckerStruct;
}

pub struct ArgsCheckerGenerator {
    pub(crate) arg_type_factory: Arc<dyn IArgTypeFactory>,
    pub(crate) field_factory: Arc<dyn IFieldFactory>,
    pub(crate) struct_factory: Arc<dyn IStructFactory>,
    pub(crate) reference_normalizer: Arc<dyn IReferenceNormalizer>,
}

impl IArgsCheckerGenerator for ArgsCheckerGenerator {
    fn generate(&self, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> ArgsCheckerStruct {
        let attrs = vec![constants::DERIVE_DEBUG_AND_I_ARGS_FORMATTER_ATTRIBUTE.clone()];
        let ident = format_ident!(
            "{}_{}",
            fn_decl.get_full_ident(),
            Self::ARGS_CHECKER_STRUCT_SUFFIX
        );
        let fn_fields: Vec<_> = fn_decl
            .arguments
            .iter()
            .flat_map(|x| self.try_convert_fn_arg_to_field(x))
            .collect();
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
            .normalize_in_struct(&mut item_struct);
        let args_checker_struct = ArgsCheckerStruct { item_struct };

        return args_checker_struct;
    }
}

impl ArgsCheckerGenerator {
    const ARGS_CHECKER_STRUCT_SUFFIX: &'static str = "ArgsChecker";

    fn try_convert_fn_arg_to_field(&self, fn_arg: &FnArg) -> Option<Field> {
        let pat_type = match fn_arg {
            FnArg::Receiver(_) => return None,
            FnArg::Typed(pat_type) => pat_type,
        };
        let ty = self.arg_type_factory.create(*pat_type.ty.clone());
        let ident = self.generate_field_ident(pat_type);

        let result = self.field_factory.create(ident, ty);
        return Some(result);
    }

    fn generate_field_ident(&self, pat_type: &PatType) -> Ident {
        let result =
            syn::parse2(pat_type.pat.to_token_stream()).expect("Should be valid field identifier");
        return result;
    }
}
