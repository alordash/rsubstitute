use crate::di::SERVICES;

mod syntax;
mod mock_macros;
mod di;

#[proc_macro_attribute]
pub fn mock(
    proc_macro_attribute: proc_macro::TokenStream,
    proc_macro_item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let macro_handler = &SERVICES.macro_handler;
    
    return macro_handler.handle(proc_macro_attribute, proc_macro_item);
}
