use syn::*;

pub(crate) fn is_static(signature: &Signature) -> bool {
    signature.inputs.first().is_some_and(|x| match x {
        FnArg::Receiver(_) => true,
        _ => false,
    })
}
