mod argument;

use super::models::*;
use crate::syntax::{generics, ident};
use crate::*;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::*;

pub(crate) struct Params<'a> {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub signature: Signature,
    pub is_default: bool,
    pub maybe_base_impl: Option<Box<Block>>,
    pub maybe_owner: Option<&'a dyn IFnOwner>,
}

pub(crate) fn prepare(
    Params {
        attributes,
        visibility,
        signature,
        is_default,
        maybe_base_impl,
        maybe_owner,
    }: Params,
) -> FnSyntax {
    let merged_generics = combine_generics(signature.generics.clone(), maybe_owner);
    let fn_ident = format_fn_ident(signature.ident.clone(), maybe_owner, &merged_generics);
    let spans = Spans {
        inputs: signature.inputs.span(),
    };
    let InputsSplit {
        maybe_self_type,
        arguments,
    } = split_inputs_into_maybe_self_type_and_arguments(signature.inputs.clone());
    let result = FnSyntax {
        attributes,
        source_signature: Box::new(signature),
        visibility,
        merged_generics,
        fn_ident,
        is_default,
        maybe_self_type,
        arguments,
        maybe_base_impl,
        spans,
    };
    return result;
}

fn format_fn_ident(
    fn_ident: Ident,
    maybe_owner: Option<&dyn IFnOwner>,
    generics: &Generics,
) -> Ident {
    let generics_suffixes = generics.params.iter().map(|x| match x {
        GenericParam::Lifetime(l) => l.lifetime.ident.clone(),
        GenericParam::Type(t) => t.ident.clone(),
        GenericParam::Const(c) => c.ident.clone(),
    });
    let ident_parts = maybe_owner
        .map(|x| x.ident().clone())
        .into_iter()
        .chain(core::iter::once(fn_ident))
        .chain(generics_suffixes);
    let result = ident::join(ident_parts, constants::IDENTS_SEPARATOR);
    return result;
}

// TODO - prove assumption - assuming fn_info generics and owner generics can not intersect
fn combine_generics(mut fn_generics: Generics, maybe_owner: Option<&dyn IFnOwner>) -> Generics {
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

    let maybe_self_type = match first_arg {
        FnArg::Receiver(receiver) => Some(receiver.clone()),
        FnArg::Typed(_) => None,
    };
    let arguments = inputs_iter
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
