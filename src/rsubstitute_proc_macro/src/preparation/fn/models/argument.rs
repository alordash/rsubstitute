use syn::*;

pub(crate) struct Argument {
    pub source_pat_type: PatType,
    pub ident: Ident,
    pub ptr_style_type: Box<Type>,
    pub ref_style_type: Box<Type>,
    pub control_fn_arg: FnArg,
}
