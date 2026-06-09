use crate::generation::r#fn::generate_generics_info_provider_impl;
use crate::generation::r#fn::models::CallStruct;
use crate::preparation::r#fn::models::*;
use crate::syntax::r#type;
use quote::format_ident;
use syn::*;

pub(crate) fn generate_call_struct(fn_syntax: &FnSyntax) -> CallStruct {
    let fields = generate_fields(fn_syntax);

    let item_struct = ItemStruct {
        attrs: Vec::new(),
        vis: Visibility::Public(Default::default()),
        struct_token: Default::default(),
        ident: format_ident!("{}_Call", fn_syntax.fn_ident),
        generics: fn_syntax.merged_generics.clone(),
        fields,
        semi_token: None,
    };

    let r#type = Type::Path(r#type::path::new(
        [&item_struct.ident.to_string()],
        fn_syntax.fn_ident.span(),
    ));
    let generics_info_provider_impl =
        generate_generics_info_provider_impl(fn_syntax.merged_generics.clone(), r#type.clone());

    let result = CallStruct {
        r#type,
        item_struct,
        generics_info_provider_impl,
        args_infos_provider_impl: todo!(),
        args_tuple_provider_impl: todo!(),
    };

    return result;
}

fn generate_fields(fn_syntax: &FnSyntax) -> Fields {
    let fields_named = FieldsNamed {
        brace_token: Default::default(),
        named: fn_syntax.arguments.iter().map(generate_field).collect(),
    };
    let result = Fields::Named(fields_named);

    return result;
}

fn generate_field(argument: &Argument) -> Field {
    let result = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: FieldMutability::None,
        ident: Some(argument.ident.clone()),
        colon_token: Some(Default::default()),
        ty: *argument.inner_type.clone(),
    };

    return result;
}
