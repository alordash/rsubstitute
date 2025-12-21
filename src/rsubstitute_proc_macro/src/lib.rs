use crate::di::SERVICES;

mod syntax;
mod mock_macros;
mod derive_args_formatter_macros;
mod di;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mock_macro_handler = &SERVICES.mock_macro_handler;
    
    return mock_macro_handler.handle(proc_macro_attribute, proc_macro_item);
}
