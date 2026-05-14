use crate::models::*;
use crate::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct ParseFnSyntaxArgs<'a> {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub signature: Signature,
    pub is_default: bool,
    pub maybe_base_impl: Option<Box<Block>>,
    pub maybe_owner: Option<&'a dyn IFnOwner>,
}

pub(crate) fn parse_fn_syntax(
    ParseFnSyntaxArgs {
        attributes,
        visibility,
        signature,
        is_default,
        maybe_base_impl,
        maybe_owner,
    }: ParseFnSyntaxArgs,
) -> FnSyntax {
    let generics = combine_generics(signature.generics, maybe_owner);
    let fn_ident = format_fn_ident(signature.ident, maybe_owner, &generics);
    let InputsSplit {
        maybe_self_type,
        arguments,
    } = split_inputs_into_maybe_self_type_and_arguments(signature.inputs);
    let result = FnSyntax {
        attributes,
        visibility,
        generics,
        fn_ident,
        is_default,
        maybe_self_type,
        arguments,
        return_type: signature.output,
        maybe_base_impl,
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
    let result = syntax::join_idents(ident_parts, constants::IDENTS_SEPARATOR);
    return result;
}

// TODO - prove assumption - assuming fn generics and owner generics can not intersect
fn combine_generics(mut fn_generics: Generics, maybe_owner: Option<&dyn IFnOwner>) -> Generics {
    let Some(owner_generics) = maybe_owner.map(IFnOwner::generics) else {
        return fn_generics;
    };

    fn_generics.params.extend(owner_generics.params.clone());
    if let Some(owner_generics_where_clause) = &owner_generics.where_clause {
        fn_generics
            .make_where_clause()
            .predicates
            .extend(owner_generics_where_clause.predicates.clone());
    }
    return fn_generics;
}

struct InputsSplit {
    pub maybe_self_type: Option<Receiver>,
    pub arguments: Vec<FnArg>,
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
    let arguments = inputs_iter.collect();
    let result = InputsSplit {
        maybe_self_type,
        arguments,
    };
    return result;
}
