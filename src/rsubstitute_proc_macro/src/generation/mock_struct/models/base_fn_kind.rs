use proc_macro2::Ident;

pub(crate) enum BaseFnKind {
    None,
    Static(Ident),
    Associated(Ident),
}
