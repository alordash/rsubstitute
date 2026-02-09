use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::mock_macros::*;
use crate::syntax::*;
use std::sync::Arc;
use syn::*;

pub trait IMockStructTraitInfoGenerator {
    fn generate(
        &self,
        trait_impl: TraitImpl,
        struct_impl_generics: &Generics,
    ) -> MockStructTraitInfo;
}

pub(crate) struct MockStructTraitInfoGenerator {
    pub fn_decl_extractor: Arc<dyn IFnDeclExtractor>,
    pub generics_merger: Arc<dyn IGenericsMerger>,
    pub mock_type_generator: Arc<dyn IMockTypeGenerator>,
    pub fn_info_generator: Arc<dyn IFnInfoGenerator>,
}

impl IMockStructTraitInfoGenerator for MockStructTraitInfoGenerator {
    fn generate(
        &self,
        trait_impl: TraitImpl,
        struct_impl_generics: &Generics,
    ) -> MockStructTraitInfo {
        let trait_ident_from_path = trait_impl.get_trait_ident_from_path();
        let merged_generics = self
            .generics_merger
            .merge(&trait_impl.item_impl.generics, &struct_impl_generics);
        let mock_generics = MockGenerics {
            source_generics: trait_impl.item_impl.generics.clone(),
            impl_generics: merged_generics,
            phantom_type_fields: Vec::new(),
        };
        let fn_decls = self
            .fn_decl_extractor
            .extract_struct_trait_impl_fns(&trait_impl);
        let mock_type = self
            .mock_type_generator
            .generate_for_trait(trait_ident_from_path.clone(), mock_generics);
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
