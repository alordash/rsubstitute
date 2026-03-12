use proc_macro2::Ident;
use syn::Field;

pub(crate) trait IFieldRequiredIdentExtension {
    fn get_required_ident(&self) -> Ident;
}

impl IFieldRequiredIdentExtension for Field {
    fn get_required_ident(&self) -> Ident {
        self.ident.clone().expect("Field should have ident!")
    }
}
