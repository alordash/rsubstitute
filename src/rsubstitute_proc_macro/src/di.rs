use crate::macros::{IMacroHandler, MacroHandler};
use std::cell::LazyCell;
use std::rc::Rc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub macro_handler: Rc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let macro_handler = Rc::new(MacroHandler);

    let services = ServiceCollection { macro_handler };

    return services;
}
