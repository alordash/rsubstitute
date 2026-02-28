use crate::constants;
use crate::mock_macros::fn_info_generation::models::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use crate::syntax::*;
use quote::format_ident;
use std::sync::Arc;
use syn::*;

pub trait IArgsCheckerGenerator {
    fn generate(&self, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> ArgsCheckerStruct;
}

pub(crate) struct ArgsCheckerGenerator {
    pub attribute_factory: Arc<dyn IAttributeFactory>,
    pub arg_type_factory: Arc<dyn IArgTypeFactory>,
    pub field_factory: Arc<dyn IFieldFactory>,
    pub struct_factory: Arc<dyn IStructFactory>,
    pub reference_normalizer: Arc<dyn IReferenceNormalizer>,
    pub arg_ident_extractor: Arc<dyn IArgIdentExtractor>,
}

impl IArgsCheckerGenerator for ArgsCheckerGenerator {
    fn generate(&self, fn_decl: &FnDecl, mock_generics: &MockGenerics) -> ArgsCheckerStruct {
        let attrs = vec![
            constants::DOC_HIDDEN_ATTRIBUTE.clone(),
            self.generate_arg_checker_derive_traits_attribute(),
        ];
        let ident = format_ident!(
            "{}_{}",
            fn_decl.get_full_ident(),
            Self::ARGS_CHECKER_STRUCT_SUFFIX
        );
        let fn_fields: Vec<_> = fn_decl
            .arguments
            .iter()
            .enumerate()
            .flat_map(|(i, x)| self.try_convert_fn_arg_to_field(i, x))
            .collect();
        let struct_fields = std::iter::once(constants::DEFAULT_ARG_FIELD_LIFETIME_FIELD.clone())
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
        let args_checker_struct = ArgsCheckerStruct { item_struct };

        return args_checker_struct;
    }
}

impl ArgsCheckerGenerator {
    const ARGS_CHECKER_STRUCT_SUFFIX: &'static str = "ArgsChecker";

    fn generate_arg_checker_derive_traits_attribute(&self) -> Attribute {
        let derive_attribute = self.attribute_factory.create(
            constants::DERIVE_IDENT.clone(),
            &format!(
                "{}, {}, {}",
                constants::DEBUG_TRAIT_NAME,
                constants::I_ARGS_FORMATTER_TRAIT_NAME,
                constants::I_GENERICS_HASH_KEY_PROVIDER_TRAIT_NAME,
            ),
        );
        return derive_attribute;
    }

    fn try_convert_fn_arg_to_field(&self, arg_number: usize, fn_arg: &FnArg) -> Option<Field> {
        let pat_type = match fn_arg {
            FnArg::Receiver(_) => return None,
            FnArg::Typed(pat_type) => pat_type,
        };
        let ty = self.arg_type_factory.create(*pat_type.ty.clone());
        let ident = self.arg_ident_extractor.extract(arg_number, pat_type);

        let result = self.field_factory.create(ident, ty);
        return Some(result);
    }
}
