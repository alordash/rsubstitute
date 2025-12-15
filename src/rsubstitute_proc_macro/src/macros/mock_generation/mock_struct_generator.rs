use crate::macros::fn_info_generation::models::FnInfo;
use crate::macros::mock_generation::models::MockStructInfo;
use syn::{Field, FieldMutability, Visibility};

pub trait IMockStructGenerator {
    fn generate(&self, fn_infos: &[FnInfo]) -> MockStructInfo;
}

pub struct MockStructGenerator;

impl IMockStructGenerator for MockStructGenerator {
    fn generate(&self, fn_infos: &[FnInfo]) -> MockStructInfo {
        todo!()
    }
}

impl MockStructGenerator {
    fn generate_field(&self, fn_info: &FnInfo) -> Field {
        let field = Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(fn_info.parent.ident.clone()),
            colon_token: Default::default(),
            ty: todo!(),
        };
        todo!();
    }
}
