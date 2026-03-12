use crate::constants;
use crate::syntax::*;
use syn::*;

pub(crate) trait IGetGlobalMockExprGenerator {
    fn generate(&self, ty: Type) -> Expr;
}

pub(crate) struct GetGlobalMockExprGenerator;

impl IGetGlobalMockExprGenerator for GetGlobalMockExprGenerator {
    fn generate(&self, ty: Type) -> Expr {
        let func = Expr::Path(ExprPath {
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
        });
        let global_mock_expr = expr_call::create_without_args(func);
        return global_mock_expr;
    }
}
