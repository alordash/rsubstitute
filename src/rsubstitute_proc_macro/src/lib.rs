use crate::di::SERVICES;

mod derive_args_formatter_macro_handler;
mod di;
mod mock_macros;
mod syntax;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mock_macro_handler = &SERVICES.mock_macro_handler;

    return mock_macro_handler.handle(proc_macro_attribute, proc_macro_item);
}

#[proc_macro_derive(IArgsFormatter)]
pub fn derive_args_formatter(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_args_formatter_macro_handler = &SERVICES.derive_args_formatter_macro_handler;

    return derive_args_formatter_macro_handler.handle(item);
}
