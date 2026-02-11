use crate::mock_macros::mock_generation::models::*;
use proc_macro2::Ident;
use syn::*;

pub struct InnerDataParam<'a> {
    pub inner_data_struct: &'a InnerDataStruct,
    pub constructor_arguments: Vec<(Ident, Type)>,
}
