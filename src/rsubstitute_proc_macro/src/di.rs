use crate::macros::{FnDeclExtractor, IMacroHandler, MacroHandler};
use crate::syntax::{
    AttributeFactory, IAttributeFactory, IPathFactory, ITypeFactory, PathFactory, TypeFactory,
};
use std::cell::LazyCell;
use std::rc::Rc;

pub(crate) const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub(crate) struct ServiceCollection {
    pub(crate) attribute_factory: Rc<dyn IAttributeFactory>,
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
    pub(crate) macro_handler: Rc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let attribute_factory = Rc::new(AttributeFactory);
    let path_factory = Rc::new(PathFactory);
    let type_factory = Rc::new(TypeFactory { path_factory });

    let fn_decl_extractor = Rc::new(FnDeclExtractor);
    let macro_handler = Rc::new(MacroHandler { fn_decl_extractor });

    let services = ServiceCollection {
        attribute_factory,
        type_factory,
        macro_handler,
    };

    return services;
}
