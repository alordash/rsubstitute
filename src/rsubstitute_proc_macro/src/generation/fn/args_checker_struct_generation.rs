mod args_checker_impl_generation;

use crate::generation::common::*;
use crate::generation::r#fn::models::*;
use crate::generation::r#fn::*;
use crate::preparation::r#fn::models::*;
use crate::syntax::r#type;
use args_checker_impl_generation::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_args_checker_struct(
    fn_syntax: &FnSyntax,
    call_struct_type: Type,
) -> ArgsCheckerStruct {
    let span = fn_syntax.spans.inputs;
    let fields_named = generate_fields(fn_syntax);
    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        struct_token: Token![struct](span),
        ident: format_ident!("{}_ArgsChecker", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields: Fields::Named(fields_named),
        semi_token: None,
    };

    let r#type = Type::Path(r#type::path::from_ident(item_struct.ident.clone()));
    let generics_info_provider_impl =
        generate_generics_info_provider_impl(fn_syntax.merged_generics.clone(), r#type.clone());
    let args_checker_impl =
        generate_args_checker_impl(span, &fn_syntax.arguments, r#type.clone(), call_struct_type);

    let result = ArgsCheckerStruct {
        r#type,
        item_struct,
        generics_info_provider_impl,
        args_checker_impl,
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
                let ty = arg_type::of(span, *argument.inner_type.clone());

                let result = Field {
                    attrs: Vec::new(),
                    vis: Visibility::Inherited,
                    mutability: FieldMutability::None,
                    ident: Some(argument.ident.clone()),
                    colon_token: Some(Token![:](argument.ident.span())),
                    ty: Type::Path(ty),
                };

                return result;
            })
            .collect(),
    };

    return result;
}
