use crate::macros::constants;
use crate::macros::fn_info_generation::models::ArgsMatcherInfo;
use crate::macros::models::FnInfo;
use crate::syntax::{IArgTypeFactory, IFieldFactory, IStructFactory};
use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use std::rc::Rc;
use syn::{Field, Fields, FieldsNamed, FnArg, PatType, Type};

pub trait IArgsMatcherGenerator {
    fn generate<'a>(&self, fn_info: &'a FnInfo) -> ArgsMatcherInfo<'a>;
}

pub struct ArgsMatcherGenerator {
    pub(crate) arg_type_factory: Rc<dyn IArgTypeFactory>,
    pub(crate) field_factory: Rc<dyn IFieldFactory>,
    pub(crate) struct_factory: Rc<dyn IStructFactory>,
}

impl IArgsMatcherGenerator for ArgsMatcherGenerator {
    fn generate<'a>(&self, fn_info: &'a FnInfo) -> ArgsMatcherInfo<'a> {
        let attrs = vec![constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone()];
        let ident = format_ident!("{}_{}", fn_info.ident, Self::ARGS_MATCHER_STRUCT_SUFFIX);
        let struct_fields: Vec<_> = fn_info
            .arguments
            .iter()
            .flat_map(|x| self.try_convert_fn_arg_to_field(x))
            .collect();
        let fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: struct_fields.into_iter().collect(),
        });

        let item_struct = self.struct_factory.create(attrs, ident, fields);
        let args_matcher_info = ArgsMatcherInfo {
            item_struct,
            parent: fn_info,
        };

        return args_matcher_info;
    }
}

impl ArgsMatcherGenerator {
    const ARGS_MATCHER_STRUCT_SUFFIX: &'static str = "ArgsMatcher";

    fn try_convert_fn_arg_to_field(&self, fn_arg: &FnArg) -> Option<Field> {
        let pat_type = match fn_arg {
            FnArg::Receiver(_) => return None,
            FnArg::Typed(pat_type) => pat_type,
        };
        let ty = self.try_generate_wrapped_field_type(pat_type)?;
        let ident = self.generate_field_ident(pat_type);

        let result = self.field_factory.create(ident, ty);
        return Some(result);
    }

    fn try_generate_wrapped_field_type(&self, pat_type: &PatType) -> Option<Type> {
        let result = match &*pat_type.ty {
            Type::Path(inner_type_path) => Some(self.arg_type_factory.create(inner_type_path)),
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
