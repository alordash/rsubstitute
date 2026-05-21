use crate::models::r#fn::*;
use crate::*;
use syn::punctuated::Punctuated;
use syn::*;

pub(crate) struct PrepareFnSyntaxArgs<'a> {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub signature: Signature,
    pub is_default: bool,
    pub maybe_base_impl: Option<Box<Block>>,
    pub maybe_owner: Option<&'a dyn IFnOwner>,
}

pub(crate) fn prepare_fn_syntax(
    PrepareFnSyntaxArgs {
        attributes,
        visibility,
        signature,
        is_default,
        maybe_base_impl,
        maybe_owner,
    }: PrepareFnSyntaxArgs,
) -> FnSyntax {
    let merged_generics = combine_generics(signature.generics, maybe_owner);
    let fn_ident = format_fn_ident(signature.ident, maybe_owner, &merged_generics);
    let InputsSplit {
        maybe_self_type,
        arguments,
    } = split_inputs_into_maybe_self_type_and_arguments(signature.inputs);
    let result = FnSyntax {
        attributes,
        visibility,
        merged_generics,
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
        GenericParam::Lifetime(l) => &l.lifetime.ident,
        GenericParam::Type(t) => &t.ident,
        GenericParam::Const(c) => &c.ident,
    });
    let ident_parts = maybe_owner
        .map(|x| x.ident())
        .into_iter()
        .chain(core::iter::once(&fn_ident))
        .chain(generics_suffixes);
    let result = syntax::ident::join(ident_parts, constants::IDENTS_SEPARATOR);
    return result;
}

// TODO - prove assumption - assuming fn generics and owner generics can not intersect
fn combine_generics(mut fn_generics: Generics, maybe_owner: Option<&dyn IFnOwner>) -> Generics {
    let Some(owner_generics) = maybe_owner.map(IFnOwner::generics) else {
        return fn_generics;
    };

    fn_generics = syntax::generics::combine(fn_generics, owner_generics);
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
