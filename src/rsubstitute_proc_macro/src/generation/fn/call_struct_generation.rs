mod args_provider_generation;

use crate::generation::r#fn::models::CallStruct;
use crate::generation::r#fn::*;
use crate::preparation::r#fn::models::*;
use args_provider_generation::*;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_call_struct(fn_syntax: &FnSyntax) -> CallStruct {
    let span = fn_syntax.spans.inputs;
    let fields = generate_fields(fn_syntax);

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Token![pub](span)),
        struct_token: Token![struct](span),
        ident: format_ident!("{}_Call", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields,
        semi_token: None,
    };

    let r#type = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: item_struct.ident.clone(),
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        },
    });
    let generics_info_provider_impl =
        generate_generics_info_provider_impl(fn_syntax.merged_generics.clone(), r#type.clone());
    let args_provider_impl =
        generate_args_provider_impl(&fn_syntax.arguments, r#type.clone(), span);

    let result = CallStruct {
        r#type,
        item_struct,
        generics_info_provider_impl,
        args_provider_impl,
    };

    return result;
}

fn generate_fields(fn_syntax: &FnSyntax) -> Fields {
    let fields_named = FieldsNamed {
        brace_token: token::Brace(fn_syntax.spans.inputs),
        named: fn_syntax.arguments.iter().map(generate_field).collect(),
    };
    let result = Fields::Named(fields_named);

    return result;
}

fn generate_field(argument: &Argument) -> Field {
    let span = argument.ident.span();
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(argument.ident.clone()),
        colon_token: Some(Token![:](span)),
        ty: *argument.inner_type.clone(),
    };

    return result;
}
