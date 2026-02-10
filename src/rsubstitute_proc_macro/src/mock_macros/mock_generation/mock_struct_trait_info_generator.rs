use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use std::sync::Arc;

pub trait IMockStructTraitInfoGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        trait_impl: TraitImpl,
    ) -> MockStructTraitInfo;
}

pub(crate) struct MockStructTraitInfoGenerator {
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
}

impl IMockStructTraitInfoGenerator for MockStructTraitInfoGenerator {
    fn generate(
        &self,
        mock_type: &MockType,
        trait_impl: TraitImpl,
    ) -> MockStructTraitInfo {
        let trait_ident_from_path = trait_impl.get_trait_ident_from_path();
        let fn_decls = self
            .fn_decl_extractor
            .extract_struct_trait_impl_fns(&trait_impl);
        let fn_infos: Vec<_> = fn_decls
            .into_iter()
            .map(|fn_decl| self.fn_info_generator.generate(fn_decl, &mock_type))
            .collect();
        let mock_struct_trait_info = MockStructTraitInfo {
            trait_ident_from_path,
            mock_type: mock_type.clone(),
            fn_infos,
        };
        return mock_struct_trait_info;
    }
}
