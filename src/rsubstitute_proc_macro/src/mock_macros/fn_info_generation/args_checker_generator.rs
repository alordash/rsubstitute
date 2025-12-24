use crate::constants;
use crate::mock_macros::fn_info_generation::models::ArgsCheckerInfo;
use crate::mock_macros::models::FnDecl;
use crate::syntax::{IArgTypeFactory, IFieldFactory, IReferenceNormalizer, IStructFactory};
use proc_macro2::Ident;
use quote::{ToTokens, format_ident};
use std::rc::Rc;
use syn::{Field, Fields, FieldsNamed, FnArg, PatType, Type};

pub trait IArgsCheckerGenerator {
    fn generate(&self, fn_decl: &FnDecl) -> ArgsCheckerInfo;
}

pub struct ArgsCheckerGenerator {
    pub(crate) arg_type_factory: Rc<dyn IArgTypeFactory>,
    pub(crate) field_factory: Rc<dyn IFieldFactory>,
    pub(crate) struct_factory: Rc<dyn IStructFactory>,
    pub(crate) reference_normalizer: Rc<dyn IReferenceNormalizer>,
}

impl IArgsCheckerGenerator for ArgsCheckerGenerator {
    fn generate(&self, fn_decl: &FnDecl) -> ArgsCheckerInfo {
        let attrs = vec![
            constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone(),
            constants::DERIVE_DEBUG_AND_I_ARGS_FORMATTER_ATTRIBUTE.clone(),
        ];
        let ident = format_ident!("{}_{}", fn_decl.ident, Self::ARGS_CHECKER_STRUCT_SUFFIX);
        let struct_fields: Vec<_> = fn_decl
            .arguments
            .iter()
            .flat_map(|x| self.try_convert_fn_arg_to_field(x))
            .collect();
        let fields_named = FieldsNamed {
            brace_token: Default::default(),
            named: struct_fields.into_iter().collect(),
        };

        let mut item_struct =
            self.struct_factory
                .create_with_default_lifetime(attrs, ident, fields_named);
        self.reference_normalizer
            .normalize_in_struct(&mut item_struct);
        let args_checker_info = ArgsCheckerInfo { item_struct };

        return args_checker_info;
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
