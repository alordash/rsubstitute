use crate::constants;
use crate::mock_generation::mock_parts_generation::get_global_mock_expr;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::syntax::extensions::*;
use crate::syntax::r#type;
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
                mock_type.ty.clone(),
                Some(constants::PLACEHOLDER_LIFETIME.clone()),
            )),
        ),
    };

    let block = Block {
        brace_token: Default::default(),
        stmts: vec![Stmt::Expr(
            get_global_mock_expr::generate(mock_type.ty.clone()),
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
