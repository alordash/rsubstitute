use crate::constants;
use crate::syntax::*;
use syn::*;

pub(crate) fn create_trait_impl_for_struct_as_self(
    item_struct: &ItemStruct,
    item_struct_ty: &Type,
) -> ItemImpl {
    let as_ref_fn = create_as_ref_fn(item_struct_ty);
    let trait_path = path::create_with_generic_type(
        constants::AS_REF_TRAIT_IDENT.clone(),
        item_struct_ty.clone(),
    );
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(item_struct.generics.clone()),
        trait_: Some((None, trait_path, Default::default())),
        self_ty: Box::new(item_struct_ty.clone()),
        brace_token: Default::default(),
        items: vec![as_ref_fn],
    };
    return item_impl;
}

fn create_as_ref_fn(item_struct_ty: &Type) -> ImplItem {
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig: Signature {
            constness: None,
            abi: None,
            unsafety: None,
            asyncness: None,
            fn_token: Default::default(),
            ident: constants::AS_REF_FN_IDENT.clone(),
            generics: Generics::default(),
            paren_token: Default::default(),
            inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
            variadic: None,
            output: ReturnType::Type(
                Default::default(),
                Box::new(r#type::reference(item_struct_ty.clone(), None)),
            ),
        },
        block: Block {
            brace_token: Default::default(),
            stmts: vec![Stmt::Expr(constants::SELF_EXPR.clone(), None)],
        },
    };
    return ImplItem::Fn(impl_item_fn);
}
