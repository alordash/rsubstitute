use proc_macro2::*;
use syn::spanned::Spanned;

pub(crate) fn join<'a, TIdents: Iterator<Item = &'a Ident>>(
    idents: TIdents,
    separator: &str,
) -> Ident {
    let (idents_strings, idents_spans): (Vec<_>, Vec<_>) =
        idents.map(|x| (x.to_string(), x.span())).unzip();
    let ident_string = idents_strings.join(separator);
    let ident_span = idents_spans
        .into_iter()
        .reduce(|a, b| a.join(b).unwrap_or(a))
        .unwrap_or_else(|| Span::call_site());
    let ident = Ident::new(&ident_string, ident_span);
    return ident;
}
