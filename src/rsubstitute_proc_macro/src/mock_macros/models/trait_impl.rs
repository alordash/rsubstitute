use syn::*;

pub(crate) struct TraitImpl<'a> {
    pub trait_path: Path,
    pub fns: Vec<&'a ImplItemFn>
}