use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::*;
use crate::syntax::*;
use std::cell::LazyCell;
use std::rc::Rc;

pub(crate) const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub(crate) struct ServiceCollection {
    pub(crate) attribute_factory: Rc<dyn IAttributeFactory>,
    pub(crate) path_factory: Rc<dyn IPathFactory>,
    pub(crate) type_factory: Rc<dyn ITypeFactory>,
    pub(crate) macro_handler: Rc<dyn IMacroHandler>,
}

fn create_services() -> ServiceCollection {
    let target_decl_extractor = Rc::new(TargetDeclExtractor);
    let fn_decl_extractor = Rc::new(FnDeclExtractor);

    let field_factory = Rc::new(FieldFactory);
    let struct_factory = Rc::new(StructFactory);
    let call_struct_generator = Rc::new(CallStructGenerator {
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
    });
    let arg_type_factory = Rc::new(ArgTypeFactory);
    let args_matcher_generator = Rc::new(ArgsMatcherGenerator {
        arg_type_factory: arg_type_factory.clone(),
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
    });
    let path_factory = Rc::new(PathFactory);
    let type_factory = Rc::new(TypeFactory {
        path_factory: path_factory.clone(),
    });
    let field_access_expr_factory = Rc::new(FieldAccessExprFactory {
        path_factory: path_factory.clone(),
    });
    let args_matcher_impl_generator = Rc::new(ArgsMatcherImplGenerator {
        type_factory: type_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
    });
    let fn_info_generator = Rc::new(FnInfoGenerator {
        call_struct_generator: call_struct_generator.clone(),
        args_matcher_generator: args_matcher_generator.clone(),
        args_matcher_impl_generator: args_matcher_impl_generator.clone(),
    });
    let mock_struct_generator = Rc::new(MockStructGenerator {
        type_factory: type_factory.clone(),
        struct_factory: struct_factory.clone(),
    });
    let field_value_factory = Rc::new(FieldValueFactory {
        path_factory: path_factory.clone(),
    });
    let expr_method_call_factory = Rc::new(ExprMethodCallFactory {
        path_factory: path_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
    });
    let mock_impl_generator = Rc::new(MockImplGenerator {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
        field_value_factory: field_value_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
    });
    let local_factory = Rc::new(LocalFactory);
    let internal_mock_impl_generator = Rc::new(InternalMockImplGenerator {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
        field_value_factory: field_value_factory.clone(),
        local_factory: local_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
    });
    let mod_generator = Rc::new(ModGenerator);

    let macro_handler = Rc::new(MacroHandler {
        target_decl_extractor: target_decl_extractor.clone(),
        fn_decl_extractor: fn_decl_extractor.clone(),
        fn_info_generator: fn_info_generator.clone(),
        mock_struct_generator: mock_struct_generator.clone(),
        mock_impl_generator: mock_impl_generator.clone(),
        internal_mock_impl_generator: internal_mock_impl_generator.clone(),
        mod_generator: mod_generator.clone(),
    });

    let attribute_factory = Rc::new(AttributeFactory);

    let services = ServiceCollection {
        attribute_factory,
        path_factory,
        type_factory,
        macro_handler,
    };

    return services;
}
