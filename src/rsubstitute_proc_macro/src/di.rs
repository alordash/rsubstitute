use crate::derive_args_formatter_macro_handler::*;
use crate::derive_args_infos_provider_macro_handler::*;
use crate::derive_args_tuple_provider_macro_handler::*;
use crate::derive_generics_hash_key_provider_macro_handler::*;
use crate::derive_mock_data_macro_handler::*;
use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::targets::*;
use crate::mock_macros::*;
use crate::syntax::*;
use std::cell::{LazyCell, OnceCell};
use std::sync::Arc;

pub(crate) const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub(crate) struct ServiceCollection {
    pub attribute_factory: Arc<dyn IAttributeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
    pub type_factory: Arc<dyn ITypeFactory>,
    pub expr_method_call_factory: Arc<dyn IExprMethodCallFactory>,
    pub expr_reference_factory: Arc<dyn IExprReferenceFactory>,
    pub derive_args_formatter_macro_handler: Arc<dyn IDeriveArgsFormatterMacroHandler>,
    pub derive_args_infos_provider_macro_handler: Arc<dyn IDeriveArgsInfosProviderMacroHandler>,
    pub derive_args_tuple_provider_macro_handler: Arc<dyn IDeriveArgsTupleProviderMacroHandler>,
    pub derive_mock_data_macro_handler: Arc<dyn IDeriveMockDataMacroHandler>,
    pub derive_generics_hash_key_provider_macro_handler:
        Arc<dyn IDeriveGenericsHashKeyProviderMacroHandler>,
    pub mock_macro_handler: Arc<dyn IMockMacroHandler>,
    pub struct_mock_syntax_parser: Arc<dyn IStructMockSyntaxParser>,
}

fn create_services() -> ServiceCollection {
    let fn_decl_extractor = Arc::new(FnDeclExtractor);
    let generic_argument_factory_cell = Arc::new(OnceCell::new());
    let path_factory = Arc::new(PathFactory {
        generic_argument_factory: generic_argument_factory_cell.clone(),
    });
    let type_factory = Arc::new(TypeFactory {
        path_factory: path_factory.clone(),
    });
    let generics_merger = Arc::new(GenericsMerger);
    let field_factory = Arc::new(FieldFactory {
        type_factory: type_factory.clone(),
    });
    let struct_factory = Arc::new(StructFactory);
    let reference_type_crawler = Arc::new(ReferenceTypeCrawler);
    let reference_normalizer = Arc::new(ReferenceNormalizer {
        reference_type_crawler: reference_type_crawler.clone(),
    });
    let arg_ident_extractor = Arc::new(ArgIdentExtractor);
    let call_struct_generator = Arc::new(CallStructGenerator {
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
        reference_normalizer: reference_normalizer.clone(),
        arg_ident_extractor: arg_ident_extractor.clone(),
    });
    let arg_type_factory = Arc::new(ArgTypeFactory);
    let args_checker_generator = Arc::new(ArgsCheckerGenerator {
        arg_type_factory: arg_type_factory.clone(),
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
        reference_normalizer: reference_normalizer.clone(),
        arg_ident_extractor: arg_ident_extractor.clone(),
    });
    let generic_argument_factory = Arc::new(GenericArgumentFactory {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
    });
    generic_argument_factory_cell.get_or_init(move || generic_argument_factory);
    let field_access_expr_factory = Arc::new(FieldAccessExprFactory {
        path_factory: path_factory.clone(),
    });
    let expr_reference_factory = Arc::new(ExprReferenceFactory);
    let expr_method_call_factory = Arc::new(ExprMethodCallFactory {
        path_factory: path_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
    });
    let debug_string_expr_generator = Arc::new(DebugStringExprGenerator {
        path_factory: path_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        expr_reference_factory: expr_reference_factory.clone(),
    });
    let args_checker_impl_generator = Arc::new(ArgsCheckerTraitImplGenerator {
        type_factory: type_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
        expr_reference_factory: expr_reference_factory.clone(),
    });
    let mock_generics_generator = Arc::new(MockGenericsGenerator {
        type_factory: type_factory.clone(),
        field_factory: field_factory.clone(),
        generics_merger: generics_merger.clone(),
    });
    let base_caller_impl_generator = Arc::new(BaseCallerImplGenerator {
        type_factory: type_factory.clone(),
        path_factory: path_factory.clone(),
    });
    let fn_info_generator = Arc::new(FnInfoGenerator {
        call_struct_generator: call_struct_generator.clone(),
        args_checker_generator: args_checker_generator.clone(),
        args_checker_impl_generator: args_checker_impl_generator.clone(),
        base_caller_impl_generator: base_caller_impl_generator.clone(),
    });
    let mock_data_struct_generator = Arc::new(MockDataStructGenerator {
        type_factory: type_factory.clone(),
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
    });
    let mock_setup_struct_generator = Arc::new(MockSetupStructGenerator {
        type_factory: type_factory.clone(),
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
    });
    let mock_received_struct_generator = Arc::new(MockReceivedStructGenerator {
        type_factory: type_factory.clone(),
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
    });
    let mock_type_generator = Arc::new(MockTypeGenerator {
        type_factory: type_factory.clone(),
    });
    let mock_struct_generator = Arc::new(MockStructGenerator {
        type_factory: type_factory.clone(),
        field_factory: field_factory.clone(),
        struct_factory: struct_factory.clone(),
        reference_normalizer: reference_normalizer.clone(),
    });
    let send_sync_impls_generator = Arc::new(SendSyncImplsGenerator {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
    });
    let field_value_factory = Arc::new(FieldValueFactory {
        expr_method_call_factory: expr_method_call_factory.clone(),
    });
    let std_mem_transmute_expr_factory = Arc::new(StdMemTransmuteExprFactory {
        path_factory: path_factory.clone(),
    });
    let get_global_mock_expr_generator = Arc::new(GetGlobalMockExprGenerator);
    let field_checker = Arc::new(FieldChecker);
    let local_factory = Arc::new(LocalFactory);
    let mock_fn_inputs_generator = Arc::new(MockFnInputsGenerator {
        arg_ident_extractor: arg_ident_extractor.clone(),
    });
    let mock_fn_block_generator = Arc::new(MockFnBlockGenerator {
        path_factory: path_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        std_mem_transmute_expr_factory: std_mem_transmute_expr_factory.clone(),
        field_value_factory: field_value_factory.clone(),
        get_global_mock_expr_generator: get_global_mock_expr_generator.clone(),
        field_checker: field_checker.clone(),
        local_factory: local_factory.clone(),
        expr_reference_factory: expr_reference_factory.clone(),
    });
    let mock_payload_impl_generator = Arc::new(MockPayloadImplGenerator {
        path_factory: path_factory.clone(),
        mock_fn_block_generator: mock_fn_block_generator.clone(),
        mock_fn_inputs_generator: mock_fn_inputs_generator.clone(),
    });
    let mock_constructor_block_generator = Arc::new(MockConstructorBlockGenerator {
        path_factory: path_factory.clone(),
        local_factory: local_factory.clone(),
    });
    let mock_impl_generator = Arc::new(MockImplGenerator {
        type_factory: type_factory.clone(),
        mock_constructor_block_generator: mock_constructor_block_generator.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        std_mem_transmute_expr_factory: std_mem_transmute_expr_factory.clone(),
    });
    let mock_struct_default_impl_generator = Arc::new(MockStructDefaultImplGenerator {
        type_factory: type_factory.clone(),
        mock_constructor_block_generator: mock_constructor_block_generator.clone(),
    });
    let input_args_generator = Arc::new(InputArgsGenerator {
        path_factory: path_factory.clone(),
        field_value_factory: field_value_factory.clone(),
        local_factory: local_factory.clone(),
        reference_normalizer: reference_normalizer.clone(),
        field_checker: field_checker.clone(),
    });
    let impl_factory = Arc::new(ImplFactory);
    let setup_output_generator = Arc::new(SetupOutputGenerator {
        type_factory: type_factory.clone(),
    });
    let mock_setup_impl_generator = Arc::new(MockSetupImplGenerator {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
        impl_factory: impl_factory.clone(),
        local_factory: local_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        input_args_generator: input_args_generator.clone(),
        setup_output_generator: setup_output_generator.clone(),
    });
    let received_signature_generator = Arc::new(ReceivedSignatureGenerator {
        type_factory: type_factory.clone(),
        input_args_generator: input_args_generator.clone(),
        reference_normalizer: reference_normalizer.clone(),
    });
    let mock_received_impl_generator = Arc::new(MockReceivedImplGenerator {
        type_factory: type_factory.clone(),
        impl_factory: impl_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        input_args_generator: input_args_generator.clone(),
        received_signature_generator: received_signature_generator.clone(),
    });
    let mod_generator = Arc::new(ModGenerator);

    let derive_args_formatter_macro_handler = Arc::new(DeriveArgsFormatterMacroHandler {
        type_factory: type_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
        field_checker: field_checker.clone(),
        debug_string_expr_generator: debug_string_expr_generator.clone(),
    });
    let derive_args_infos_provider_macro_handler = Arc::new(DeriveArgsInfosProviderMacroHandler {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
        expr_reference_factory: expr_reference_factory.clone(),
        debug_string_expr_generator: debug_string_expr_generator.clone(),
        field_checker: field_checker.clone(),
    });
    let derive_args_tuple_provider_macro_handler = Arc::new(DeriveArgsTupleProviderMacroHandler {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
        expr_reference_factory: expr_reference_factory.clone(),
        field_checker: field_checker.clone(),
    });
    let derive_mock_data_macro_handler = Arc::new(DeriveMockDataMacroHandler {
        path_factory: path_factory.clone(),
        type_factory: type_factory.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
    });
    let derive_generics_hash_key_provider_macro_handler =
        Arc::new(DeriveGenericsHashKeyProviderMacroHandler {
            type_factory: type_factory.clone(),
            path_factory: path_factory.clone(),
            expr_method_call_factory: expr_method_call_factory.clone(),
        });

    let fn_setup_generator = Arc::new(FnSetupGenerator {
        input_args_generator: input_args_generator.clone(),
        setup_output_generator: setup_output_generator.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        local_factory: local_factory.clone(),
        path_factory: path_factory.clone(),
        get_global_mock_expr_generator: get_global_mock_expr_generator.clone(),
    });
    let fn_received_generator = Arc::new(FnReceivedGenerator {
        received_signature_generator: received_signature_generator.clone(),
        expr_method_call_factory: expr_method_call_factory.clone(),
        get_global_mock_expr_generator: get_global_mock_expr_generator.clone(),
    });
    let static_fn_generator = Arc::new(StaticFnGenerator {
        mock_fn_block_generator: mock_fn_block_generator.clone(),
        mock_fn_inputs_generator: mock_fn_inputs_generator.clone(),
    });

    let mock_struct_trait_info_generator = Arc::new(MockStructTraitInfoGenerator {
        fn_decl_extractor: fn_decl_extractor.clone(),
        fn_info_generator: fn_info_generator.clone(),
    });

    let mock_struct_trait_generator = Arc::new(MockStructTraitGenerator {
        mock_setup_struct_generator: mock_setup_struct_generator.clone(),
        mock_received_struct_generator: mock_received_struct_generator.clone(),
        mock_setup_impl_generator: mock_setup_impl_generator.clone(),
        mock_received_impl_generator: mock_received_impl_generator.clone(),
    });

    let inner_data_struct_generator = Arc::new(InnerDataStructGenerator);
    let inner_data_impl_generator = Arc::new(InnerDataImplGenerator {
        type_factory: type_factory.clone(),
    });
    let inner_data_param_generator = Arc::new(InnerDataParamGenerator);
    let inner_data_deref_impl_generator = Arc::new(InnerDataDerefImplGenerator {
        type_factory: type_factory.clone(),
        path_factory: path_factory.clone(),
        field_access_expr_factory: field_access_expr_factory.clone(),
        expr_reference_factory: expr_reference_factory.clone(),
    });
    let ignored_impl_fixer = Arc::new(IgnoredImplFixer {
        generics_merger: generics_merger.clone(),
    });

    let item_trait_handler = Arc::new(ItemTraitHandler {
        fn_decl_extractor: fn_decl_extractor.clone(),
        mock_generics_generator: mock_generics_generator.clone(),
        mock_type_generator: mock_type_generator.clone(),
        fn_info_generator: fn_info_generator.clone(),
        mock_data_struct_generator: mock_data_struct_generator.clone(),
        mock_setup_struct_generator: mock_setup_struct_generator.clone(),
        mock_received_struct_generator: mock_received_struct_generator.clone(),
        mock_struct_generator: mock_struct_generator.clone(),
        mock_payload_impl_generator: mock_payload_impl_generator.clone(),
        mock_impl_generator: mock_impl_generator.clone(),
        mock_setup_impl_generator: mock_setup_impl_generator.clone(),
        mock_received_impl_generator: mock_received_impl_generator.clone(),
        mod_generator: mod_generator.clone(),
    });

    let item_fn_handler = Arc::new(ItemFnHandler {
        fn_decl_extractor: fn_decl_extractor.clone(),
        mock_generics_generator: mock_generics_generator.clone(),
        mock_type_generator: mock_type_generator.clone(),
        fn_info_generator: fn_info_generator.clone(),
        mock_data_struct_generator: mock_data_struct_generator.clone(),
        mock_setup_struct_generator: mock_setup_struct_generator.clone(),
        mock_received_struct_generator: mock_received_struct_generator.clone(),
        mock_struct_generator: mock_struct_generator.clone(),
        send_sync_impls_generator: send_sync_impls_generator.clone(),
        mock_struct_default_impl_generator: mock_struct_default_impl_generator.clone(),
        mock_setup_impl_generator: mock_setup_impl_generator.clone(),
        mock_received_impl_generator: mock_received_impl_generator.clone(),
        mod_generator: mod_generator.clone(),
        fn_setup_generator: fn_setup_generator.clone(),
        fn_received_generator: fn_received_generator.clone(),
        static_fn_generator: static_fn_generator.clone(),
    });

    let struct_mock_handler = Arc::new(StructMockHandler {
        mock_struct_trait_info_generator: mock_struct_trait_info_generator.clone(),
        fn_decl_extractor: fn_decl_extractor.clone(),
        mock_generics_generator: mock_generics_generator.clone(),
        mock_type_generator: mock_type_generator.clone(),
        fn_info_generator: fn_info_generator.clone(),
        mock_data_struct_generator: mock_data_struct_generator.clone(),
        mock_setup_struct_generator: mock_setup_struct_generator.clone(),
        mock_received_struct_generator: mock_received_struct_generator.clone(),
        inner_data_struct_generator: inner_data_struct_generator.clone(),
        inner_data_impl_generator: inner_data_impl_generator.clone(),
        inner_data_param_generator: inner_data_param_generator.clone(),
        mock_struct_generator: mock_struct_generator.clone(),
        inner_data_deref_impl_generator: inner_data_deref_impl_generator.clone(),
        mock_struct_trait_generator: mock_struct_trait_generator.clone(),
        mock_payload_impl_generator: mock_payload_impl_generator.clone(),
        mock_impl_generator: mock_impl_generator.clone(),
        mock_setup_impl_generator: mock_setup_impl_generator.clone(),
        mock_received_impl_generator: mock_received_impl_generator.clone(),
        ignored_impl_fixer: ignored_impl_fixer.clone(),
        mod_generator: mod_generator.clone(),
    });

    let mock_macro_handler = Arc::new(MockMacroHandler {
        item_trait_handler,
        item_fn_handler,
        struct_mock_handler,
    });

    let struct_mock_syntax_parser = Arc::new(StructMockSyntaxParser);

    let attribute_factory = Arc::new(AttributeFactory);

    let services = ServiceCollection {
        attribute_factory,
        path_factory,
        type_factory,
        expr_method_call_factory,
        expr_reference_factory,
        derive_args_formatter_macro_handler,
        derive_args_infos_provider_macro_handler,
        derive_args_tuple_provider_macro_handler,
        derive_mock_data_macro_handler,
        derive_generics_hash_key_provider_macro_handler,
        mock_macro_handler,
        struct_mock_syntax_parser,
    };

    return services;
}
