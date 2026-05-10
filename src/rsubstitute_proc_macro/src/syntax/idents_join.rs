use proc_macro2::Ident;
use quote::format_ident;

pub(crate) fn join_idents<TIdents: Iterator<Item = Ident>>(
    mut idents: TIdents,
    sep: char,
) -> Ident {
    let mut result = idents
        .next()
        .expect("Idents for joining should not be empty.");
    while let Some(next) = idents.next() {
        result = format_ident!("{result}{sep}{next}");
    }
    return result;
}
