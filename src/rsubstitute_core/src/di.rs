use crate::error_printer::*;
use std::cell::LazyCell;
use std::sync::Arc;

pub const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub struct ServiceCollection {
    pub error_printer: Arc<dyn IErrorPrinter>,
}

fn create_services() -> ServiceCollection {
    let error_printer = Arc::new(ErrorPrinter);

    let services = ServiceCollection { error_printer };

    return services;
}
