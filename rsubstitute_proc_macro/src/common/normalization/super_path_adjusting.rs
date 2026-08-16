use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) fn normalize_super_paths(mut block: Block) -> Block {
    SuperPathsNormalizer.visit_block_mut(&mut block);
    return block;
}

struct SuperPathsNormalizer;

impl VisitMut for SuperPathsNormalizer {
    fn visit_use_path_mut(&mut self, i: &mut UsePath) {
        if i.ident == "super" {
            let decoy_tree = Box::new(UseTree::Glob(UseGlob {
                star_token: Default::default(),
            }));
            let mut base_tree = core::mem::replace(&mut i.tree, decoy_tree);
            visit_mut::visit_use_tree_mut(self, &mut base_tree);
            i.tree = Box::new(UseTree::Path(UsePath {
                ident: i.ident.clone(),
                colon2_token: i.colon2_token.clone(),
                tree: base_tree,
            }));
            return;
        }

        visit_mut::visit_use_path_mut(self, i);
    }
}
