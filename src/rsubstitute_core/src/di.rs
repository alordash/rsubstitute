use std::rc::Rc;
use crate::error_printer::IErrorPrinter;

pub(crate) struct ServiceCollection {
    pub(crate) error_printer: Rc<dyn IErrorPrinter>
}