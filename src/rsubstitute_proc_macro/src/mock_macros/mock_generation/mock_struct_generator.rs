use crate::mock_macros::constants;
use crate::mock_macros::fn_info_generation::models::FnInfo;
use crate::mock_macros::mock_generation::models::MockStructInfo;
use crate::mock_macros::models::TargetDecl;
use crate::syntax::{IStructFactory, ITypeFactory};
use quote::format_ident;
use std::rc::Rc;
use syn::{
    AngleBracketedGenericArguments, Field, FieldMutability, Fields, FieldsNamed, GenericArgument,
    Path, PathArguments, PathSegment, Type, TypePath, Visibility,
};

pub trait IMockStructGenerator {
    fn generate(&self, target_decl: &TargetDecl, fn_infos: &[FnInfo]) -> MockStructInfo;
}

pub struct MockStructGenerator {
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
    pub(crate) struct_factory: Rc<dyn IStructFactory>,
}

impl IMockStructGenerator for MockStructGenerator {
    fn generate(&self, target_decl: &TargetDecl, fn_infos: &[FnInfo]) -> MockStructInfo {
        let attrs = Vec::new();
        let ident = format_ident!("{}{}", target_decl.ident, Self::MOCK_STRUCT_IDENT_PREFIX);
        let fields = fn_infos.iter().map(|x| self.generate_field(x));
        let fields = Fields::Named(FieldsNamed {
            brace_token: Default::default(),
            named: fields.collect(),
        });
        let item_struct = self.struct_factory.create(attrs, ident, fields);
        let mock_struct_info = MockStructInfo { item_struct };
        return mock_struct_info;
    }
}

impl MockStructGenerator {
    const MOCK_STRUCT_IDENT_PREFIX: &'static str = "Mock";

    fn generate_field(&self, fn_info: &FnInfo) -> Field {
        let field = Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(fn_info.data_field_ident.clone()),
            colon_token: Default::default(),
            ty: Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [PathSegment {
                        ident: constants::FN_DATA_TYPE_IDENT.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: [
                                GenericArgument::Type(
                                    self.type_factory
                                        .create(fn_info.call_info.item_struct.ident.clone()),
                                ),
                                GenericArgument::Type(
                                    self.type_factory.create(
                                        fn_info.args_checker_info.item_struct.ident.clone(),
                                    ),
                                ),
                                GenericArgument::Type(fn_info.parent.get_return_value_type()),
                            ]
                            .into_iter()
                            .collect(),
                            gt_token: Default::default(),
                        }),
                    }]
                    .into_iter()
                    .collect(),
                },
            }),
        };
        return field;
    }
}
