use crate::constants;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::Literal;
use quote::quote;
use syn::token::Paren;
use syn::*;

// TODO - replace all derive attributes with manual generation (because why parse code twice?)
pub(crate) fn generate(
    args_checker_struct: &ItemStruct,
    fields_maybe_actual_source_types: &[Option<Type>],
) -> ItemImpl {
    let fmt_args_impl =
        create_fmt_args_impl_item(&args_checker_struct, fields_maybe_actual_source_types);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(args_checker_struct.generics.clone()),
        trait_: Some((
            None,
            constants::I_ARGS_FORMATTER_TRAIT_PATH.clone(),
            Default::default(),
        )),
        self_ty: Box::new(r#type::create_from_struct(&args_checker_struct)),
        brace_token: Default::default(),
        items: vec![fmt_args_impl],
    };
    return item_impl;
}

fn create_fmt_args_impl_item(
    item_struct: &ItemStruct,
    fields_maybe_actual_source_types: &[Option<Type>],
) -> ImplItem {
    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: constants::I_ARGS_FORMATTER_FN_IDENT.clone(),
        generics: Generics::default(),
        paren_token: Default::default(),
        inputs: [constants::REF_SELF_ARG.clone()].into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(Default::default(), Box::new(constants::STRING_TYPE.clone())),
    };
    let block = create_fmt_args_block(item_struct, fields_maybe_actual_source_types);
    let impl_item_fn = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        defaultness: None,
        sig,
        block,
    };
    let impl_item = ImplItem::Fn(impl_item_fn);
    return impl_item;
}

fn create_fmt_args_block(
    item_struct: &ItemStruct,
    fields_maybe_actual_source_types: &[Option<Type>],
) -> Block {
    let literal_str = item_struct
        .fields
        .iter()
        .skip_while(|field| field::is_phantom_data(field))
        .map(|_| "{}")
        .collect::<Vec<_>>()
        .join(", ");
    let literal = Literal::string(&literal_str);
    let args: Vec<_> = item_struct
        .fields
        .iter()
        .skip_while(|field| field::is_phantom_data(field))
        .zip(fields_maybe_actual_source_types)
        .map(|(field, maybe_actual_source_type)| {
            let field_ident = field.get_required_ident();
            let receiver = if let Some(actual_source_type) = maybe_actual_source_type {
                transmute_lifetime_expr::create_for_arg(field_ident, actual_source_type.clone())
            } else {
                field_access_expr::create(vec![constants::SELF_IDENT.clone(), field_ident])
            };
            debug_string_expr::generate(receiver, None)
        })
        .collect();
    let tokens = quote! { #literal, #(#args),* };
    let format_stmt = Stmt::Expr(
        Expr::Macro(ExprMacro {
            attrs: Vec::new(),
            mac: Macro {
                path: constants::MACRO_FORMAT_PATH.clone(),
                bang_token: Default::default(),
                delimiter: MacroDelimiter::Paren(Paren::default()),
                tokens,
            },
        }),
        None,
    );
    let block = Block {
        brace_token: Default::default(),
        stmts: vec![format_stmt],
    };
    return block;
}
