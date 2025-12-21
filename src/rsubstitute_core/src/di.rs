use crate::error_printer::*;
use std::cell::LazyCell;
use std::rc::Rc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub error_printer: Rc<dyn IErrorPrinter>,
}

fn create_services() -> ServiceCollection {
    let error_printer = Rc::new(ErrorPrinter);

    let services = ServiceCollection { error_printer };

    return services;
}
