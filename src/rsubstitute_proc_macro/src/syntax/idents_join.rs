use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn join_idents<'a, TIdents: Iterator<Item = &'a Ident>>(
    mut idents: TIdents,
    sep: char,
) -> Ident {
    let mut result = idents
        .next()
        .expect("Idents for joining should not be empty.")
        .clone();
    while let Some(next) = idents.next() {
        result = format_ident!("{result}{sep}{next}");
    }
    return result;
}
