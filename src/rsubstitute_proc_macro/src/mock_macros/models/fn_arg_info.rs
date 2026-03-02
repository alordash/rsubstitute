use proc_macro2::Ident;
use syn::*;

pub(crate) enum FnArgInfo {
    Receiver(Receiver),
    Typed(TypedFnArgInfo),
}

pub(crate) struct TypedFnArgInfo {
    pub ident: Ident,
    pub ty: Type,
}

impl TypedFnArgInfo {
    pub fn clone_as_fn_arg(&self) -> FnArg {
        let fn_arg = FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: Box::new(Pat::Ident(PatIdent {
                attrs: Vec::new(),
                by_ref: None,
                mutability: None,
                ident: self.ident.clone(),
                subpat: None,
            })),
            colon_token: Default::default(),
            ty: Box::new(self.ty.clone()),
        });
        return fn_arg;
    }
}
