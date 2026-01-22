use crate::constants;
use crate::syntax::*;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IGetGlobalMockExprGenerator {
    fn generate(&self, item_struct: &ItemStruct) -> Expr;
}

pub(crate) struct GetGlobalMockExprGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
}

impl IGetGlobalMockExprGenerator for GetGlobalMockExprGenerator {
    fn generate(&self, item_struct: &ItemStruct) -> Expr {
        let global_mock_expr = Expr::Call(ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [PathSegment {
                        ident: constants::GET_GLOBAL_MOCK_FN_IDENT.clone(),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(Default::default()),
                            lt_token: Default::default(),
                            args: [GenericArgument::Type(
                                self.type_factory.create_from_struct(item_struct),
                            )]
                            .into_iter()
                            .collect(),
                            gt_token: Default::default(),
                        }),
                    }]
                    .into_iter()
                    .collect(),
                },
            })),
            paren_token: Default::default(),
            args: Punctuated::new(),
        });
        return global_mock_expr;
    }
}
