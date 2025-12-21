use crate::mock_macros::constants;
use crate::mock_macros::fn_info_generation::models::CallInfo;
use crate::mock_macros::models::FnDecl;
use crate::syntax::{IFieldFactory, IStructFactory};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use std::rc::Rc;
use syn::{Field, Fields, FieldsNamed, FnArg, PatType, Type};

pub trait ICallStructGenerator {
    fn generate(&self, fn_decl: &FnDecl) -> CallInfo;
}

pub struct CallStructGenerator {
    pub(crate) field_factory: Rc<dyn IFieldFactory>,
    pub(crate) struct_factory: Rc<dyn IStructFactory>,
}

impl ICallStructGenerator for CallStructGenerator {
    fn generate(&self, fn_decl: &FnDecl) -> CallInfo {
        let attrs = vec![
            constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone(),
            constants::DERIVE_CLONE_ATTRIBUTE.clone(),
        ];
        let ident = format_ident!("{}_{}", fn_decl.ident, Self::CALL_STRUCT_SUFFIX);
        let struct_fields = fn_decl
            .arguments
            .iter()
            .flat_map(|x| self.try_convert_fn_arg_to_field(x));
        let fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: struct_fields.collect(),
        });

        let item_struct = self.struct_factory.create(attrs, ident, fields);
        let call_info = CallInfo {
            item_struct,
        };

        return call_info;
    }
}

impl CallStructGenerator {
    pub const CALL_STRUCT_SUFFIX: &'static str = "Call";

    fn try_convert_fn_arg_to_field(&self, fn_arg: &FnArg) -> Option<Field> {
        let pat_type = match fn_arg {
            FnArg::Receiver(_) => return None,
            FnArg::Typed(pat_type) => pat_type,
        };
        let ty = self.get_field_type(pat_type)?;
        let ident = self.generate_field_ident(pat_type);

        let result = self.field_factory.create(ident, ty);
        return Some(result);
    }

    fn get_field_type(&self, pat_type: &PatType) -> Option<Type> {
        let result = match &*pat_type.ty {
            Type::Path(inner_type_path) => Some(Type::Path(inner_type_path.clone())),
            _ => None,
        };
        return result;
    }

    fn generate_field_ident(&self, pat_type: &PatType) -> Ident {
        let result =
            syn::parse2(pat_type.pat.to_token_stream()).expect("Should be valid field identifier");
        return result;
    }
}
