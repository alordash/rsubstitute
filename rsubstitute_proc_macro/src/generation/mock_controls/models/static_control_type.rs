use syn::*;

pub(crate) enum StaticControlType {
    Setup {
        mock_generic_argument: GenericArgument,
    },
    Received,
}
