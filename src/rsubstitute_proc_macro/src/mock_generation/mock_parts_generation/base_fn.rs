use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::mock_generation::models::FnDecl;
use crate::mock_generation::parameters::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use quote::format_ident;
use std::cell::LazyCell;
use syn::*;

pub(crate) fn generate(
    mock_type: &MockType,
    fn_decl: &FnDecl,
    call_struct: &CallStruct,
    base_fn_block: Block,
) -> BaseFn {
    let (sig, block) = generate_call_base_fn_parts(
        fn_decl,
        call_struct,
        base_fn_block,
        mock_type,
        Target::Trait,
        None,
    );
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };

    let base_fn = BaseFn { impl_item_fn };
    return base_fn;
}

pub(crate) fn generate_struct_trait_fn(
    mock_type: &MockType,
    fn_decl: &FnDecl,
    call_struct: &CallStruct,
    base_fn_block: Block,
    trait_ident: &Ident,
) -> BaseFn {
    let (sig, block) = generate_call_base_fn_parts(
        fn_decl,
        call_struct,
        base_fn_block,
        mock_type,
        Target::Trait,
        Some(trait_ident),
    );
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };

    let base_fn = BaseFn { impl_item_fn };
    return base_fn;
}

pub(crate) fn generate_static(
    mock_type: &MockType,
    fn_decl: &FnDecl,
    call_struct: &CallStruct,
    base_fn_block: Block,
) -> StaticBaseFn {
    let (sig, block) = generate_call_base_fn_parts(
        fn_decl,
        call_struct,
        base_fn_block,
        mock_type,
        Target::Static,
        None,
    );
    let item_fn = ItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        sig,
        block: Box::new(block),
    };

    let static_base_fn = StaticBaseFn { item_fn };
    return static_base_fn;
}

const CALL_ARG_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("call"));

fn generate_call_base_fn_parts(
    fn_decl: &FnDecl,
    call_struct: &CallStruct,
    base_fn_block: Block,
    mock_type: &MockType,
    target: Target,
    maybe_containing_trait_ident: Option<&Ident>,
) -> (Signature, Block) {
    let mut generics = match target {
        Target::Static => fn_decl.merged_generics.clone(),
        Target::Trait => fn_decl.own_generics.clone(),
    };
    generics = add_lifetime_constraints_to_generic_types(generics);
    let first_arg = match target {
        Target::Static => FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Wild(PatWild {
                attrs: Vec::new(),
                underscore_token: Default::default(),
            })),
            colon_token: Default::default(),
            ty: Box::new(r#type::reference(mock_type.ty.clone(), None)),
        }),
        Target::Trait => constants::REF_SELF_ARG.clone(),
    };
    let mut call_struct_ty = call_struct.ty_path.clone();
    // call_struct_ty.set_first_generic_lifetime_argument(constants::ANONYMOUS_LIFETIME.clone());
    let call_arg = FnArg::Typed(PatType {
        attrs: Vec::new(),
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: CALL_ARG_IDENT.clone(),
            subpat: None,
        })),
        colon_token: Default::default(),
        ty: Box::new(Type::Path(call_struct_ty)),
    });
    let ident = match maybe_containing_trait_ident {
        Some(containing_trait_ident) => base_fn_ident::format(&format_ident!(
            "{}_{}",
            containing_trait_ident,
            fn_decl.fn_ident
        )),
        None => base_fn_ident::format(&fn_decl.fn_ident),
    };
    let inputs = [first_arg, call_arg];
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident,
        generics,
        paren_token: Default::default(),
        inputs: inputs.into_iter().collect(),
        variadic: None,
        output: fn_decl.return_value.clone(),
    };
    let block = generate_call_base_fn_block(
        fn_decl,
        call_struct,
        base_fn_block,
        mock_type.generics.get_phantom_fields_count(),
    );
    return (sig, block);
}

fn generate_call_base_fn_block(
    fn_decl: &FnDecl,
    call_struct: &CallStruct,
    base_fn_block: Block,
    phantom_fields_count: usize,
) -> Block {
    let call_struct_fields = call_struct
        .item_struct
        .fields
        .iter()
        .skip(fn_decl.get_internal_phantom_types_count() + phantom_fields_count);
    let mut fields_actual_types_assignment_statements = Vec::new();
    let fields = fn_decl
        .arguments
        .iter()
        .filter_map(|fn_arg| match fn_arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(typed) => Some(typed),
        })
        .zip(call_struct_fields)
        .zip(&call_struct.fields_maybe_actual_source_types)
        .map(
            |((typed_fn_arg, call_struct_field), maybe_actual_source_type)| {
                let field_ident = call_struct_field.get_required_ident();
                if let Some(actual_source_type) = maybe_actual_source_type {
                    fields_actual_types_assignment_statements.push(Stmt::Local(
                        local::create_with_type(
                            field_ident.clone(),
                            actual_source_type.clone(),
                            LocalInit {
                                eq_token: Default::default(),
                                expr: Box::new(transmute_lifetime_expr::create(
                                    field_ident.clone(),
                                )),
                                diverge: None,
                            },
                        ),
                    ))
                }
                FieldPat {
                    attrs: Vec::new(),
                    member: Member::Named(field_ident),
                    colon_token: Some(Default::default()),
                    pat: typed_fn_arg.pat.clone(),
                }
            },
        )
        .collect();
    let mut call_struct_generics = call_struct.item_struct.generics.clone();
    lifetime::set_all_lifetimes_in_generics(
        &mut call_struct_generics,
        &constants::ANONYMOUS_LIFETIME.clone(),
    );
    let deconstruct_call_stmt = Stmt::Local(Local {
        attrs: vec![
            constants::ALLOW_NON_SHORTHAND_FIELD_PATTERNS_ATTRIBUTE.clone(),
            constants::ALLOW_UNUSED_VARIABLES_ATTRIBUTE.clone(),
        ],
        let_token: Default::default(),
        pat: Pat::Struct(PatStruct {
            attrs: Vec::new(),
            qself: None,
            path: path::create_with_generics(
                call_struct.item_struct.ident.clone(),
                call_struct_generics,
            ),
            brace_token: Default::default(),
            fields,
            rest: Some(PatRest {
                attrs: Vec::new(),
                dot2_token: Default::default(),
            }),
        }),
        init: Some(LocalInit {
            eq_token: Default::default(),
            expr: Box::new(path::create_expr(CALL_ARG_IDENT.clone())),
            diverge: None,
        }),
        semi_token: Default::default(),
    });
    let stmts = std::iter::once(deconstruct_call_stmt)
        .chain(fields_actual_types_assignment_statements)
        .chain(base_fn_block.stmts)
        .collect();
    let block = Block {
        brace_token: Default::default(),
        stmts,
    };
    return block;
}

fn add_lifetime_constraints_to_generic_types(mut generics: Generics) -> Generics {
    let lifetime_param_bounds: Vec<_> = generics
        .lifetimes()
        .map(|lifetime_param| TypeParamBound::Lifetime(lifetime_param.lifetime.clone()))
        .collect();
    for type_param in generics.type_params_mut() {
        type_param.bounds.extend(lifetime_param_bounds.clone());
    }
    return generics;
}
