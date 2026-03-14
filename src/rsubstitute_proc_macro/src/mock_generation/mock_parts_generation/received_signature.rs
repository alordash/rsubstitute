use crate::constants;
use crate::mock_generation::fn_info_generation::models::*;
use crate::mock_generation::mock_parts_generation::models::*;
use crate::mock_generation::mock_parts_generation::*;
use crate::mock_generation::parameters::*;
use crate::syntax::extensions::*;
use crate::syntax::*;
use proc_macro2::Ident;
use quote::format_ident;
use std::cell::LazyCell;
use std::iter;
use syn::*;

pub(crate) fn get_times_arg_ident() -> Ident {
    format_ident!("times")
}

pub(crate) fn generate_for_trait(
    fn_info: &FnInfo,
    mock_type: &MockType,
    output_type_generics: OutputTypeGenerics,
) -> Signature {
    let result = generate(
        fn_info,
        fn_info.parent.fn_ident.clone(),
        Target::Trait,
        constants::SELF_TYPE.clone(),
        mock_type,
        output_type_generics,
    );
    return result;
}

pub(crate) fn generate_for_static(
    fn_info: &FnInfo,
    mock_received_struct: &MockReceivedStruct,
    mock_type: &MockType,
) -> Signature {
    let mut owner_type = mock_received_struct.ty.clone();
    lifetime::staticify_anonymous_lifetimes(&mut owner_type);
    let result = generate(
        fn_info,
        constants::MOCK_RECEIVED_FIELD_IDENT.clone(),
        Target::Static,
        owner_type,
        mock_type,
        OutputTypeGenerics::UseMock,
    );
    return result;
}

const TIMES_TYPE_IDENT: LazyCell<Ident> = LazyCell::new(|| format_ident!("Times"));

fn generate(
    fn_info: &FnInfo,
    fn_ident: Ident,
    target: Target,
    owner_type: Type,
    mock_type: &MockType,
    output_type_generics: OutputTypeGenerics,
) -> Signature {
    let times_arg = FnArg::Typed(PatType {
        attrs: Vec::new(),
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: get_times_arg_ident(),
            subpat: None,
        })),
        colon_token: Default::default(),
        ty: Box::new(r#type::create(TIMES_TYPE_IDENT.clone())),
    });
    let own_inputs = input_args::generate_input_args(
        fn_info,
        fn_info.parent.get_internal_phantom_types_count()
            + mock_type.generics.get_phantom_fields_count(),
    );
    let output_type = generate_output_type(fn_info.parent.arg_refs_tuple.clone(), owner_type);

    let mut generics = match output_type_generics {
        OutputTypeGenerics::UseFnOwn => fn_info.parent.own_generics.clone(),
        OutputTypeGenerics::UseMock => mock_type.generics.impl_generics.clone(),
        OutputTypeGenerics::DoNotUse => Default::default(),
    };
    generics = generics.with_head_lifetime_param(constants::PLACEHOLDER_LIFETIME_PARAM.clone());
    generics = referenced_generic_types_lifetimes_filler::fill(generics, mock_type, &own_inputs);

    let mut inputs: Vec<_> = own_inputs
        .into_iter()
        .chain(iter::once(times_arg))
        .collect();
    match target {
        Target::Trait => inputs.insert(0, constants::REF_SELF_ARG.clone()),
        _ => (),
    }

    let signature = Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: fn_ident,
        generics,
        paren_token: Default::default(),
        inputs: inputs.into_iter().collect(),
        variadic: None,
        output: ReturnType::Type(Default::default(), Box::new(output_type)),
    };
    return signature;
}

fn generate_output_type(mut arg_refs_tuple: Type, owner_type: Type) -> Type {
    lifetime::normalize_anonymous_lifetimes(&mut arg_refs_tuple);
    let result = Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident: constants::FN_VERIFIER_TYPE_IDENT.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: [
                        GenericArgument::Type(owner_type),
                        GenericArgument::Type(arg_refs_tuple),
                    ]
                    .into_iter()
                    .collect(),
                    gt_token: Default::default(),
                }),
            }]
            .into_iter()
            .collect(),
        },
    });
    return result;
}
