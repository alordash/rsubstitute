use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::Literal;
use quote::quote;
use syn::token::Paren;
use syn::*;

// TODO - replace all derive attributes with manual generation (because why parse code twice?)
pub(crate) fn generate(args_checker_struct: &ArgsCheckerStruct) -> ItemImpl {
    let fmt_args_impl = create_fmt_args_impl_item(&args_checker_struct.item_struct);
    let item_impl = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Default::default(),
        generics: generics::remove_default_values(args_checker_struct.item_struct.generics.clone()),
        trait_: Some((
            None,
            constants::I_ARGS_FORMATTER_TRAIT_PATH.clone(),
            Default::default(),
        )),
        self_ty: Box::new(r#type::create_from_struct(&args_checker_struct.item_struct)),
        brace_token: Default::default(),
        items: vec![fmt_args_impl],
    };
    return item_impl;
}

fn create_fmt_args_impl_item(item_struct: &ItemStruct) -> ImplItem {
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
    let block = create_fmt_args_block(item_struct);
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

fn create_fmt_args_block(item_struct: &ItemStruct) -> Block {
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
        .map(|field| {
            debug_string_expr::generate(
                field_access_expr::create(vec![
                    constants::SELF_IDENT.clone(),
                    field.get_required_ident(),
                ]),
                None,
            ) // TODO - pass something instead of None (after turning derive macro into part of main code generation)
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
