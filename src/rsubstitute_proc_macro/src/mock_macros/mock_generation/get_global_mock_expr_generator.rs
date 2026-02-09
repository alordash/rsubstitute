use crate::constants;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IGetGlobalMockExprGenerator {
    fn generate(&self, ty: Type) -> Expr;
}

pub(crate) struct GetGlobalMockExprGenerator;

impl IGetGlobalMockExprGenerator for GetGlobalMockExprGenerator {
    fn generate(&self, ty: Type) -> Expr {
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
                            args: [GenericArgument::Type(ty)].into_iter().collect(),
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
