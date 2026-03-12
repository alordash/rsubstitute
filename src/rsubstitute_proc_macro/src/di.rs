use crate::derive_args_formatter_macro_handler::*;
use crate::derive_args_infos_provider_macro_handler::*;
use crate::derive_args_tuple_provider_macro_handler::*;
use crate::derive_clone_for_r_substitute_macro_handler::*;
use crate::derive_generics_hash_key_provider_macro_handler::*;
use crate::derive_mock_data_macro_handler::*;
use crate::mock_macros::fn_info_generation::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::targets::*;
use crate::mock_macros::*;
use std::cell::LazyCell;
use std::sync::Arc;

pub(crate) const SERVICES: LazyCell<ServiceCollection> = LazyCell::new(create_services);

pub(crate) struct ServiceCollection {
    pub derive_args_formatter_macro_handler: Arc<dyn IDeriveArgsFormatterMacroHandler>,
    pub derive_args_infos_provider_macro_handler: Arc<dyn IDeriveArgsInfosProviderMacroHandler>,
    pub derive_args_tuple_provider_macro_handler: Arc<dyn IDeriveArgsTupleProviderMacroHandler>,
    pub derive_mock_data_macro_handler: Arc<dyn IDeriveMockDataMacroHandler>,
    pub derive_generics_hash_key_provider_macro_handler:
        Arc<dyn IDeriveGenericsHashKeyProviderMacroHandler>,
    pub derive_clone_for_rsubstitute_macro_handler: Arc<dyn IDeriveCloneForRSubstituteMacroHandler>,
    pub mock_macro_handler: Arc<dyn IMockMacroHandler>,
    pub(crate) struct_mock_syntax_parser: Arc<dyn IStructMockSyntaxParser>,
}

fn create_services() -> ServiceCollection {
    let fn_decl_extractor = Arc::new(FnDeclExtractor);
    let call_struct_generator = Arc::new(CallStructGenerator);
    let args_checker_generator = Arc::new(ArgsCheckerGenerator);
    let debug_string_expr_generator = Arc::new(DebugStringExprGenerator);
    let args_checker_impl_generator = Arc::new(ArgsCheckerTraitImplGenerator {
        debug_string_expr_generator: debug_string_expr_generator.clone(),
    });
    let mock_generics_generator = Arc::new(MockGenericsGenerator);
    let base_fn_ident_formatter = Arc::new(BaseFnIdentFormatter);
    let base_fn_generator = Arc::new(BaseFnGenerator {
        base_fn_ident_formatter: base_fn_ident_formatter.clone(),
    });
    let fn_info_generator = Arc::new(FnInfoGenerator {
        call_struct_generator: call_struct_generator.clone(),
        args_checker_generator: args_checker_generator.clone(),
        args_checker_impl_generator: args_checker_impl_generator.clone(),
    });
    let mock_data_struct_generator = Arc::new(MockDataStructGenerator);
    let implemented_trait_ident_formatter = Arc::new(ImplementedTraitIdentFormatter);
    let mock_setup_struct_generator = Arc::new(MockSetupStructGenerator {
        implemented_trait_ident_formatter: implemented_trait_ident_formatter.clone(),
    });
    let mock_received_struct_generator = Arc::new(MockReceivedStructGenerator {
        implemented_trait_ident_formatter: implemented_trait_ident_formatter.clone(),
    });
    let mock_type_generator = Arc::new(MockTypeGenerator);
    let mock_struct_generator = Arc::new(MockStructGenerator);
    let get_global_mock_expr_generator = Arc::new(GetGlobalMockExprGenerator);
    let mock_fn_inputs_generator = Arc::new(MockFnInputsGenerator);
    let mock_fn_block_generator = Arc::new(MockFnBlockGenerator {
        get_global_mock_expr_generator: get_global_mock_expr_generator.clone(),
        base_fn_ident_formatter: base_fn_ident_formatter.clone(),
    });
    let mock_payload_impl_generator = Arc::new(MockPayloadImplGenerator {
        mock_fn_block_generator: mock_fn_block_generator.clone(),
        mock_fn_inputs_generator: mock_fn_inputs_generator.clone(),
    });
    let mock_constructor_block_generator = Arc::new(MockConstructorBlockGenerator {
        implemented_trait_ident_formatter: implemented_trait_ident_formatter.clone(),
    });
    let mock_impl_generator = Arc::new(MockImplGenerator {
        mock_constructor_block_generator: mock_constructor_block_generator.clone(),
    });
    let mock_struct_default_impl_generator = Arc::new(MockStructDefaultImplGenerator {
        mock_constructor_block_generator: mock_constructor_block_generator.clone(),
    });
    let input_args_generator = Arc::new(InputArgsGenerator);
    let setup_output_generator = Arc::new(SetupOutputGenerator);
    let mock_setup_impl_generator = Arc::new(MockSetupImplGenerator {
        input_args_generator: input_args_generator.clone(),
        setup_output_generator: setup_output_generator.clone(),
    });
    let received_signature_generator = Arc::new(ReceivedSignatureGenerator {
        input_args_generator: input_args_generator.clone(),
    });
    let mock_received_impl_generator = Arc::new(MockReceivedImplGenerator {
        input_args_generator: input_args_generator.clone(),
        received_signature_generator: received_signature_generator.clone(),
    });
    let mod_generator = Arc::new(ModGenerator);

    let derive_args_formatter_macro_handler = Arc::new(DeriveArgsFormatterMacroHandler {
        debug_string_expr_generator: debug_string_expr_generator.clone(),
    });
    let derive_args_infos_provider_macro_handler = Arc::new(DeriveArgsInfosProviderMacroHandler {
        debug_string_expr_generator: debug_string_expr_generator.clone(),
    });
    let derive_args_tuple_provider_macro_handler = Arc::new(DeriveArgsTupleProviderMacroHandler);
    let derive_mock_data_macro_handler = Arc::new(DeriveMockDataMacroHandler);
    let derive_generics_hash_key_provider_macro_handler =
        Arc::new(DeriveGenericsHashKeyProviderMacroHandler);
    let derive_clone_for_rsubstitute_macro_handler =
        Arc::new(DeriveCloneForRSubstituteMacroHandler);

    let fn_setup_generator = Arc::new(FnSetupGenerator {
        input_args_generator: input_args_generator.clone(),
        setup_output_generator: setup_output_generator.clone(),
        get_global_mock_expr_generator: get_global_mock_expr_generator.clone(),
    });
    let fn_received_generator = Arc::new(FnReceivedGenerator {
        received_signature_generator: received_signature_generator.clone(),
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
    let inner_data_impl_generator = Arc::new(InnerDataImplGenerator);
    let inner_data_param_generator = Arc::new(InnerDataParamGenerator);
    let inner_data_deref_impl_generator = Arc::new(InnerDataDerefImplGenerator {});
    let ignored_impl_fixer = Arc::new(IgnoredImplFixer);

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
        base_fn_generator: base_fn_generator.clone(),
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
        mock_struct_default_impl_generator: mock_struct_default_impl_generator.clone(),
        mock_setup_impl_generator: mock_setup_impl_generator.clone(),
        mock_received_impl_generator: mock_received_impl_generator.clone(),
        fn_setup_generator: fn_setup_generator.clone(),
        fn_received_generator: fn_received_generator.clone(),
        static_fn_generator: static_fn_generator.clone(),
        base_fn_generator: base_fn_generator.clone(),
        mod_generator: mod_generator.clone(),
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
        base_fn_generator: base_fn_generator.clone(),
        mod_generator: mod_generator.clone(),
    });

    let ctx_factory = Arc::new(CtxFactory);

    let mock_macro_handler = Arc::new(MockMacroHandler {
        ctx_factory,
        item_trait_handler,
        item_fn_handler,
        struct_mock_handler,
    });

    let struct_mock_syntax_parser = Arc::new(StructMockSyntaxParser);

    let services = ServiceCollection {
        derive_args_formatter_macro_handler,
        derive_args_infos_provider_macro_handler,
        derive_args_tuple_provider_macro_handler,
        derive_mock_data_macro_handler,
        derive_generics_hash_key_provider_macro_handler,
        derive_clone_for_rsubstitute_macro_handler,
        mock_macro_handler,
        struct_mock_syntax_parser,
    };

    return services;
}
