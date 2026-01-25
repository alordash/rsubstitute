use proc_macro2::Ident;
use syn::Field;

pub trait IFieldRequiredIdentGetter {
    fn get_required_ident(&self) -> Ident;
}

impl IFieldRequiredIdentGetter for Field {
    fn get_required_ident(&self) -> Ident {
        self.ident.clone().expect("Field should have ident!")
    }
}
