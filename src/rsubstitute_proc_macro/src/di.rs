use crate::macros::{FnInfoExtractor, IMacroHandler, MacroHandler};
use std::cell::LazyCell;
use std::rc::Rc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub macro_handler: Rc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let fn_info_extractor = Rc::new(FnInfoExtractor);
    let macro_handler = Rc::new(MacroHandler { fn_info_extractor });

    let services = ServiceCollection { macro_handler };

    return services;
}
