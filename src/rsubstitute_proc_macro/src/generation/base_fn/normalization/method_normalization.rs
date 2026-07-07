use crate::generation::base_fn::common::rsubstitute_self;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_method(mut signature: Signature, mut block: Block) -> (Signature, Block) {
    let mut self_keyword_normalizer = SelfKeywordNormalizer;
    self_keyword_normalizer.visit_signature_mut(&mut signature);
    self_keyword_normalizer.visit_block_mut(&mut block);
    return (signature, block);
}

struct SelfKeywordNormalizer;

impl VisitMut for SelfKeywordNormalizer {
    fn visit_path_segment_mut(&mut self, i: &mut PathSegment) {
        if i.ident == "self" {
            i.ident = rsubstitute_self(i.span());
        }
        visit_mut::visit_path_segment_mut(self, i);
    }
}
