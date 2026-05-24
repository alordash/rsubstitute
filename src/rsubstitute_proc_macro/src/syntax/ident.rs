use proc_macro2::*;

pub(crate) fn join<'a, TIdents: Iterator<Item = &'a Ident>>(
    idents: TIdents,
    separator: &str,
) -> Ident {
    let idents_strings: Vec<_> = idents.map(|x| x.to_string()).collect();
    let ident_string = idents_strings.join(separator);
    let ident = Ident::new(&ident_string, Span::call_site());
    return ident;
}
