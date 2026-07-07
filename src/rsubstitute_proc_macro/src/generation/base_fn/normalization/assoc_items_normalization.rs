use crate::generation::common::models::*;
use crate::syntax::self_type;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_associated_items(
    associated_items_info: &AssociatedItemsInfo,
    mut signature: Signature,
    mut block: Block,
) -> (Signature, Block) {
    let mut associated_items_normalizer = AssociatedItemsNormalizer {
        associated_items_info,
    };
    associated_items_normalizer.visit_signature_mut(&mut signature);
    associated_items_normalizer.visit_block_mut(&mut block);
    return (signature, block);
}

struct AssociatedItemsNormalizer<'a> {
    associated_items_info: &'a AssociatedItemsInfo,
}

impl<'a> VisitMut for AssociatedItemsNormalizer<'a> {
    fn visit_type_path_mut(&mut self, i: &mut TypePath) {
        if i.path.segments.get(1).is_some_and(|second_segment| {
            self.associated_items_info
                .associated_items_ident_strings
                .contains(&second_segment.ident.to_string())
        }) && let Some(first_segment) = i.path.segments.get_mut(0)
            && first_segment.ident == "Self"
        {
            let span = first_segment.span();
            *first_segment = self.associated_items_info.trait_path_segment.clone();
            i.qself = Some(QSelf {
                lt_token: Token![<](span),
                ty: Box::new(Type::Path(self_type(span))),
                position: 1,
                as_token: Some(Token![as](span)),
                gt_token: Token![>](span),
            })
        }
        visit_mut::visit_type_path_mut(self, i);
    }
}
