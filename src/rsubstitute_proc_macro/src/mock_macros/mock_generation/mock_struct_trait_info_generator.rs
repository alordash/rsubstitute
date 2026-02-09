use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::models::MockStructTraitInfo;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use std::sync::Arc;

pub trait IMockStructTraitInfoGenerator {
    fn generate(&self, trait_impl: TraitImpl) -> MockStructTraitInfo;
}

pub(crate) struct MockStructTraitInfoGenerator {
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub mock_generics_generator: Arc<dyn IMockGenericsGenerator>,
    pub mock_type_generator: Arc<dyn IMockTypeGenerator>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
}

impl IMockStructTraitInfoGenerator for MockStructTraitInfoGenerator {
    fn generate(&self, trait_impl: TraitImpl) -> MockStructTraitInfo {
        let trait_ident_from_path = trait_impl.get_trait_ident_from_path();
        let generics = self.mock_generics_generator.generate(&trait_impl.item_impl.generics);
        let fn_decls = self
            .fn_decl_extractor
            .extract_struct_trait_impl_fns(&trait_impl);
        let mock_type = self
            .mock_type_generator
            .generate_for_trait(trait_ident_from_path.clone(), generics);
        let fn_infos: Vec<_> = fn_decls
            .into_iter()
            .map(|fn_decl| self.fn_info_generator.generate(fn_decl, &mock_type))
            .collect();
        let mock_struct_trait_info = MockStructTraitInfo {
            trait_ident_from_path,
            mock_type,
            fn_infos,
        };
        return mock_struct_trait_info;
    }
}
