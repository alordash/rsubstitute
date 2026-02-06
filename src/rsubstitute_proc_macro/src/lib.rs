use crate::di::SERVICES;

mod constants;
mod derive_args_formatter_macro_handler;
mod derive_mock_data_macro_handler;
mod di;
mod lifetime_ref;
mod mock_macros;
mod syntax;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mock_macro_handler = &SERVICES.mock_macro_handler;

    return mock_macro_handler.handle_attribute_macro(proc_macro_attribute, proc_macro_item);
}

#[proc_macro]
pub fn mocked(proc_macro_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mock_macro_handler = &SERVICES.mock_macro_handler;

    let result = mock_macro_handler.handle_macro(proc_macro_item);
    dbg!(result.to_string());
    return result;
}

#[proc_macro_derive(IArgsFormatter)]
pub fn derive_args_formatter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_args_formatter_macro_handler = &SERVICES.derive_args_formatter_macro_handler;

    return derive_args_formatter_macro_handler.handle(item);
}

#[proc_macro_derive(IMockData)]
pub fn derive_mock_data(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_mock_data_macro_handler = &SERVICES.derive_mock_data_macro_handler;

    return derive_mock_data_macro_handler.handle(item);
}