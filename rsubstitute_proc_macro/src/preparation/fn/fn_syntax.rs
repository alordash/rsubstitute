mod argument;

use super::models::*;
use crate::common::*;
use crate::preparation::*;
use crate::syntax::*;
use proc_macro2::Span;
use quote::{ToTokens, format_ident};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) struct Params<'a> {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub signature: Signature,
    pub maybe_base_impl: Option<Box<Block>>,
    pub maybe_owner: Option<&'a dyn IFnOwner>,
}

pub(crate) fn prepare(
    Params {
        attributes,
        visibility,
        mut signature,
        maybe_base_impl,
        maybe_owner,
    }: Params,
) -> FnSyntax {
    let merged_generics = combine_generics(signature.generics.clone(), maybe_owner);
    let fn_ident = format_fn_ident(signature.ident.clone(), maybe_owner);
    let fn_data_name = format_fn_data_name(signature.ident.clone(), maybe_owner);
    let spans = Spans {
        inputs: signature.inputs.span(),
    };
    let InputsSplit {
        maybe_self_type,
        arguments,
    } = split_inputs_into_maybe_self_type_and_arguments(signature.inputs.clone());
    let generics_field = generics_field::new_field(
        signature.generics.span(),
        &merged_generics,
        Some(arguments.iter_generics_style_types().collect()),
    );
    let arg_refs_tuple = generate_arg_refs_tuple(spans.inputs, &arguments);
    let return_type = match &mut signature.output {
        ReturnType::Default => ReturnType::Default,
        ReturnType::Type(arrow_token, ty) => {
            let replace_impl_trait_result = normalization::replace_impl_trait_with_box_dyn_trait(
                *ty.clone()
            );
            if replace_impl_trait_result.is_impl_trait {
                *ty = Box::new(replace_impl_trait_result.ty.clone());
            }
            ReturnType::Type(arrow_token.clone(), Box::new(replace_impl_trait_result.ty))
        }
    };
    signature = r#fn::common::replace_arg_pats_with_idents(signature, &arguments);
    let result = FnSyntax {
        spans,
        attributes,
        source_signature: Box::new(signature),
        visibility,
        merged_generics,
        generics_field,
        fn_ident,
        fn_data_name,
        maybe_self_type,
        arguments,
        arg_refs_tuple,
        maybe_base_impl,
        return_type,
    };
    return result;
}

fn format_fn_ident(fn_ident: Ident, maybe_owner: Option<&dyn IFnOwner>) -> Ident {
    if let Some(owner_ident) = maybe_owner.map(|x| x.maybe_ident()).flatten() {
        return format_ident!("{owner_ident}_{fn_ident}");
    }
    return fn_ident;
}

fn format_fn_data_name(fn_ident: Ident, maybe_owner: Option<&dyn IFnOwner>) -> String {
    let fn_data_name_parts: Vec<_> = maybe_owner
        .map(|owner| owner.maybe_ident().map(|ident| ident.to_string()))
        .into_iter()
        .flatten()
        .chain(core::iter::once(fn_ident.to_string()))
        .collect();
    let result = fn_data_name_parts.join("::");
    return result;
}

// TODO - prove assumption - assuming fn_info generics and owner generics can not intersect
fn combine_generics(mut fn_generics: Generics, maybe_owner: Option<&dyn IFnOwner>) -> Generics {
    fn_generics = rsubstitute_lifetime::prepend_to_generics(fn_generics);
    let Some(owner_generics) = maybe_owner.map(IFnOwner::generics) else {
        return fn_generics;
    };

    fn_generics = generics::combine(fn_generics, owner_generics);
    return fn_generics;
}

struct InputsSplit {
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<Argument>,
}
fn split_inputs_into_maybe_self_type_and_arguments(
    inputs: Punctuated<FnArg, Token![,]>,
) -> InputsSplit {
    let mut inputs_iter = inputs.into_iter();
    let Some(first_arg) = inputs_iter.next() else {
        return InputsSplit {
            maybe_self_type: None,
            arguments: Vec::new(),
        };
    };

    let (maybe_self_type, maybe_first_arg) = match first_arg {
        FnArg::Receiver(receiver) => (Some(receiver.clone()), None),
        FnArg::Typed(_) => (None, Some(first_arg)),
    };
    let arguments = maybe_first_arg
        .into_iter()
        .chain(inputs_iter)
        .map(|fn_arg| match fn_arg {
            FnArg::Typed(pat_type) => pat_type,
            unexpected => panic!(
                "All arguments except first should be `FnArg::Typed`, received: {}.",
                unexpected.to_token_stream().to_string()
            ),
        })
        .enumerate()
        .map(argument::new)
        .collect();
    let result = InputsSplit {
        maybe_self_type,
        arguments,
    };
    return result;
}

fn generate_arg_refs_tuple(span: Span, arguments: &[Argument]) -> TypeTuple {
    let result = TypeTuple {
        attrs: Vec::new(),
        paren_token: token::Paren(span),
        elems: arguments
            .iter()
            .map(|x| {
                Type::Reference(TypeReference {
                    attrs: Vec::new(),
                    and_token: Token![&](span),
                    lifetime: Some(rsubstitute_lifetime::new(span)),
                    mutability: None,
                    elem: x.ref_style_type.clone(),
                })
            })
            .collect(),
    };
    return result;
}
