use crate::mock_generation::mock_parts_generation::models::*;
use proc_macro2::Ident;
use syn::*;

pub(crate) struct InnerDataParam<'a> {
    pub inner_data_struct: &'a InnerDataStruct,
    pub constructor_arguments: Vec<(Ident, Type)>,
}
