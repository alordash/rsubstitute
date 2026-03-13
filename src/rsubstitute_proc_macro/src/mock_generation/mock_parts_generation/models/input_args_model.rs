use syn::*;

pub(crate) struct InputArgs {
    pub fn_args: Vec<FnArg>,
    pub placeholder_fn_arg_lifetime_params: Vec<LifetimeParam>
}