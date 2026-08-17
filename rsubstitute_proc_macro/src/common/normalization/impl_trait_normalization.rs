use crate::syntax::*;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::visit_mut::VisitMut;
use syn::*;

pub(crate) struct Result {
    pub ty: Type,
    pub is_impl_trait: bool,
}
pub(crate) fn replace_impl_trait_with_box_dyn_trait(mut ty: Type) -> Result {
    let mut impl_trait_replacer = ImplTraitReplacer::new();
    impl_trait_replacer.visit_type_mut(&mut ty);
    let result = Result {
        ty,
        is_impl_trait: impl_trait_replacer.is_impl_trait,
    };
    return result;
}

struct ImplTraitReplacer {
    pub is_impl_trait: bool,
}

impl ImplTraitReplacer {
    pub fn new() -> Self {
        Self {
            is_impl_trait: false,
        }
    }
}

impl VisitMut for ImplTraitReplacer {
    fn visit_type_mut(&mut self, i: &mut Type) {
        if let Type::ImplTrait(type_impl_trait) = i {
            let span = type_impl_trait.span();
            let type_dyn_trait = TypeTraitObject {
                attrs: Vec::new(),
                dyn_token: Some(Token![dyn](span)),
                bounds: core::mem::take(&mut type_impl_trait.bounds),
            };
            let type_box_dyn_trait = TypePath {
                attrs: Vec::new(),
                qself: None,
                path: path::new_generics(
                    span,
                    ["Box"],
                    [GenericArgument::Type(Type::TraitObject(type_dyn_trait))],
                ),
            };
            *i = Type::Path(type_box_dyn_trait);
            self.is_impl_trait = true;
        }

        visit_mut::visit_type_mut(self, i);
    }
}

pub(crate) fn box_impl_trait_return_values(mut block: Block) -> Block {
    ImplTraitReturnValueBoxer.visit_block_mut(&mut block);
    return block;
}

struct ImplTraitReturnValueBoxer;

impl VisitMut for ImplTraitReturnValueBoxer {
    fn visit_block_mut(&mut self, i: &mut Block) {
        if let Some(Stmt::Expr(return_expr, None)) = i.stmts.last_mut() {
            let span = return_expr.span();
            let decoy_expr = Expr::Verbatim(TokenStream::new());
            let source_return_expr = core::mem::replace(return_expr, decoy_expr);
            *return_expr = Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(span, ["Box", "new"])),
                [source_return_expr],
            ));
        }

        visit_mut::visit_block_mut(self, i);
    }

    fn visit_expr_return_mut(&mut self, i: &mut ExprReturn) {
        if let Some(return_expr) = i.expr.as_mut() {
            let span = return_expr.span();
            let decoy_expr = Box::new(Expr::Verbatim(TokenStream::new()));
            let source_return_expr = core::mem::replace(return_expr, decoy_expr);
            *return_expr = Box::new(Expr::Call(expr::call::new(
                span,
                Expr::Path(expr::path::new(span, ["Box", "new"])),
                [*source_return_expr],
            )));
        }

        visit_mut::visit_expr_return_mut(self, i);
    }
}
