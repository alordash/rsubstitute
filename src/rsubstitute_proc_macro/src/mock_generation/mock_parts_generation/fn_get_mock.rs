use crate::constants;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use syn::punctuated::Punctuated;
use syn::*;

pub fn generate(mock_type: &MockType) -> ItemFn {
    let mut generics = mock_type.generics.impl_generics.clone();
    generics = generics.with_head_lifetime_param(constants::PLACEHOLDER_LIFETIME_PARAM.clone());

    // TODO - support asyncness and unsafety (and abi?)
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: constants::GET_MOCK_FN_IDENT.clone(),
        generics,
        paren_token: Default::default(),
        inputs: Punctuated::new(),
        variadic: None,
        output: ReturnType::Type(
            Default::default(),
            Box::new(r#type::reference(
                Type::Path(mock_type.ty_path.clone()),
                Some(constants::PLACEHOLDER_LIFETIME.clone()),
            )),
        ),
    };

    let block = Block {
        brace_token: Default::default(),
        stmts: vec![Stmt::Expr(
            generate_get_global_mock_expr(Type::Path(mock_type.ty_path.clone())),
            None,
        )],
    };

    let item_fn = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Default::default()),
        sig,
        block: Box::new(block),
    };

    return item_fn;
}

fn generate_get_global_mock_expr(ty: Type) -> Expr {
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
    let global_mock_expr = call::create_without_args(func);
    return global_mock_expr;
}
