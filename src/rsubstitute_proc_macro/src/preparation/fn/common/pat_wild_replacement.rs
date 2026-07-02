use crate::preparation::r#fn::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) fn replace_arg_pat_with_ident(mut pat_type: PatType, ident: Ident) -> PatType {
    *pat_type.pat = Pat::Ident(PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        ident,
        subpat: None,
    });
    return pat_type;
}

pub(crate) fn replace_arg_pats_with_idents(
    mut signature: Signature,
    arguments: &[Argument],
) -> Signature {
    signature.inputs = signature
        .inputs
        .into_iter()
        .take_while(|x| match x {
            FnArg::Receiver(_) => true,
            _ => false,
        })
        .chain(
            arguments
                .iter()
                .map(|x| replace_arg_pat_with_ident(x.source_pat_type.clone(), x.ident.clone()))
                .map(FnArg::Typed),
        )
        .collect();

    return signature;
}
