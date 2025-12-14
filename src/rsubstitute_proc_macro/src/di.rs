use crate::macros::{FnInfoExtractor, IMacroHandler, MacroHandler};
use crate::syntax::{AttributeFactory, IAttributeFactory};
use std::cell::LazyCell;
use std::rc::Rc;

pub(crate) const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub(crate) struct ServiceCollection {
    pub(crate) attribute_factory: Rc<dyn IAttributeFactory>,
    pub(crate) macro_handler: Rc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let attribute_factory = Rc::new(AttributeFactory);

    let fn_info_extractor = Rc::new(FnInfoExtractor);
    let macro_handler = Rc::new(MacroHandler { fn_info_extractor });

    let services = ServiceCollection {
        attribute_factory,
        macro_handler,
    };

    return services;
}
