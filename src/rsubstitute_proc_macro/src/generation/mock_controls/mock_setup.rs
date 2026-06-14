use super::*;
use crate::generation::r#fn::models::*;
use crate::generation::mock_controls::models::*;
use crate::generation::mock_controls::*;
use crate::generation::*;
use crate::preparation::models::Context;
use crate::syntax::*;
use proc_macro2::Span;
use quote::format_ident;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct Params<'a> {
    pub ctx: &'a Context,
    pub source_span: Span,
    pub target_ident: Ident,
    pub mock_type: Type,
    pub mock_data_ident: Ident,
    pub stores_mock_data: bool,
    pub fn_infos: &'a [FnInfo],
}

pub(crate) fn generate(
    Params {
        ctx,
        source_span,
        target_ident,
        mock_type,
        mock_data_ident,
        stores_mock_data,
        fn_infos,
    }: Params,
) -> MockSetup {
    let fields_named = FieldsNamed {
        brace_token: token::Brace(source_span),
        named: punctuated([Field {
            attrs: Vec::new(),
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new(constants::DATA_FIELD, source_span)),
            colon_token: Some(Token![:](source_span)),
            ty: Type::Path(r#type::arc_of(
                source_span,
                Type::Path(r#type::path::from_ident(mock_data_ident)),
            )),
        }]),
    };

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        struct_token: Token![struct](source_span),
        ident: format_ident!("{target_ident}Setup"),
        generics: Generics::default(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let r#type = Type::Path(r#type::path::from_ident(item_struct.ident.clone()));
    let clone_impl = clone_impl::new(source_span, r#type.clone());
    let r#impl = generate_impl(
        ctx,
        source_span,
        r#type.clone(),
        mock_type,
        stores_mock_data,
        fn_infos,
    );

    let result = MockSetup {
        r#type,
        item_struct,
        clone_impl,
        r#impl,
    };

    return result;
}

fn generate_impl(
    ctx: &Context,
    source_span: Span,
    target_type: Type,
    mock_type: Type,
    stores_mock_data: bool,
    fn_infos: &[FnInfo],
) -> ItemImpl {
    let items = fn_infos
        .into_iter()
        .map(|x| generate_impl_fn(ctx, mock_type.clone(), stores_mock_data, x))
        .collect();

    let result = ItemImpl {
        attrs: Vec::new(),
        defaultness: None,
        unsafety: None,
        impl_token: Token![impl](source_span),
        generics: Generics::default(),
        trait_: None,
        self_ty: Box::new(target_type),
        brace_token: token::Brace(source_span),
        items,
    };

    return result;
}

fn generate_impl_fn(
    ctx: &Context,
    mock_type: Type,
    stores_mock_data: bool,
    fn_info: &FnInfo,
) -> ImplItem {
    let span = fn_info.syntax.spans.inputs;

    let mut generics = fn_info.syntax.merged_generics.clone();
    generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam {
            attrs: Vec::new(),
            lifetime: anonymous_lifetime::new(span),
            colon_token: None,
            bounds: Punctuated::new(),
        }),
    );

    let inputs = core::iter::once(ref_self_fn_arg(span))
        .chain(
            fn_info
                .syntax
                .arguments
                .iter()
                .map(|x| x.control_fn_arg.clone()),
        )
        .collect();

    let return_type = fn_configurator::new(fn_configurator::Params {
        ctx,
        mock_type,
        stores_mock_data,
        fn_info,
    });

    let sig = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Token![fn](span),
        ident: fn_info.syntax.fn_ident.clone(),
        generics,
        paren_token: token::Paren(span),
        inputs,
        variadic: None,
        output: ReturnType::Type(Token!(->)(span), Box::new(Type::Path(return_type))),
    };

    let block = Block {
        brace_token: token::Brace(span),
        stmts: vec![todo!()],
    };

    let result = ImplItemFn {
        attrs: Vec::new(),
        vis: Visibility::Public(Token!(pub)(span)),
        defaultness: None,
        sig,
        block,
    };

    return ImplItem::Fn(result);
}
