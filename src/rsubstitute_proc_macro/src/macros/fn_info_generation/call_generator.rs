use crate::macros::constants;
use crate::macros::models::FnInfo;
use proc_macro2::Ident;
use quote::{ToTokens, format_ident};
use syn::{
    AngleBracketedGenericArguments, Field, FieldMutability, Fields, FieldsNamed, FnArg,
    GenericArgument, Generics, ItemStruct, PatType, Path, PathArguments, PathSegment, Type,
    TypePath, Visibility,
};

pub trait ICallStructGenerator {
    fn generate(&self, fn_info: &FnInfo) -> ItemStruct;
}

pub struct CallStructGenerator;

impl ICallStructGenerator for CallStructGenerator {
    fn generate(&self, fn_info: &FnInfo) -> ItemStruct {
        let attrs = vec![
            constants::ALLOW_NON_CAMEL_CASE_TYPES_ATTRIBUTE.clone(),
            constants::DERIVE_CLONE_ATTRIBUTE.clone(),
        ];
        let ident = format_ident!("{}_{}", fn_info.ident, constants::CALL_STRUCT_SUFFIX);

        let struct_fields: Vec<_> = fn_info
            .arguments
            .iter()
            .flat_map(|x| self.try_convert_fn_arg_to_field(x))
            .collect();
        let fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: struct_fields.into_iter().collect(),
        });

        let result = ItemStruct {
            attrs,
            vis: Visibility::Public(Default::default()),
            struct_token: Default::default(),
            ident,
            generics: Generics::default(),
            fields,
            semi_token: Default::default(),
        };

        return result;
    }
}

impl CallStructGenerator {
    fn try_convert_fn_arg_to_field(&self, fn_arg: &FnArg) -> Option<Field> {
        let pat_type = match fn_arg {
            FnArg::Receiver(_) => return None,
            FnArg::Typed(pat_type) => pat_type,
        };
        let ty = self.try_generate_wrapped_field_type(pat_type)?;
        let ident = self.generate_field_ident(pat_type);

        let result = Field {
            attrs: Vec::new(),
            vis: Visibility::Public(Default::default()),
            mutability: FieldMutability::None,
            ident: Some(ident),
            colon_token: Some(Default::default()),
            ty,
        };
        return Some(result);
    }

    fn try_generate_wrapped_field_type(&self, pat_type: &PatType) -> Option<Type> {
        let result = match &*pat_type.ty {
            Type::Path(inner_type_path) => Some(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: Default::default(),
                    segments: [PathSegment {
                        ident: constants::ARG_WRAPPER_TYPE_IDENT.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(Default::default()),
                            lt_token: Default::default(),
                            args: [GenericArgument::Type(Type::Path(inner_type_path.clone()))]
                                .into_iter()
                                .collect(),
                            gt_token: Default::default(),
                        }),
                    }]
                    .into_iter()
                    .collect(),
                },
            })),
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
