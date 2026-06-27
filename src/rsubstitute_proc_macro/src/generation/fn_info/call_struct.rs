mod call_impl;

use crate::generation::fn_info::models::*;
use crate::generation::fn_info::*;
use crate::preparation::r#fn::models::*;
use crate::syntax::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate(fn_syntax: &FnSyntax) -> CallStruct {
    let span = fn_syntax.spans.inputs;
    let fields_named = generate_fields(fn_syntax);

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token!(pub)(span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{}_Call", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let path = path::from_ident(item_struct.ident.clone());
    let r#type = Type::Path(TypePath {
        qself: None,
        path: path.clone(),
    });
    let generics_info_provider_impl =
        generics_info_provider_impl::generate(fn_syntax.merged_generics.clone(), r#type.clone());
    let call_impl = call_impl::generate(span, &fn_syntax.arguments, r#type.clone());

    let result = CallStruct {
        path,
        item_struct,
        generics_info_provider_impl,
        call_impl,
    };

    return result;
}

fn generate_fields(fn_syntax: &FnSyntax) -> FieldsNamed {
    let result = FieldsNamed {
        brace_token: token::Brace(fn_syntax.spans.inputs),
        named: fn_syntax
            .arguments
            .iter()
            .map(|argument| {
                let span = argument.ident.span();
                let result = Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(argument.ident.clone()),
                    colon_token: Some(Token![:](span)),
                    ty: *argument.ptr_style_type.clone(),
                };

                return result;
            })
            .collect(),
    };

    return result;
}
