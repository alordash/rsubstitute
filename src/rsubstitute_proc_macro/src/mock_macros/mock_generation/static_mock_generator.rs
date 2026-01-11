use crate::constants;
use crate::mock_macros::mock_generation::models::*;
use crate::mock_macros::mock_generation::*;
use crate::mock_macros::models::*;
use crate::syntax::*;
use quote::format_ident;
use std::sync::Arc;
use syn::punctuated::Punctuated;
use syn::*;

pub trait IStaticMockGenerator {
    fn generate(
        &self,
        fn_decl: &FnDecl,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        base_caller_struct: &BaseCallerStruct
    ) -> StaticMock;
}

pub(crate) struct StaticMockGenerator {
    pub type_factory: Arc<dyn ITypeFactory>,
    pub path_factory: Arc<dyn IPathFactory>,
    pub mock_constructor_block_generator: Arc<dyn IMockConstructorBlockGenerator>,
}

impl IStaticMockGenerator for StaticMockGenerator {
    fn generate(
        &self,
        fn_decl: &FnDecl,
        mock_struct: &MockStruct,
        mock_data_struct: &MockDataStruct,
        mock_setup_struct: &MockSetupStruct,
        mock_received_struct: &MockReceivedStruct,
        base_caller_struct: &BaseCallerStruct
    ) -> StaticMock {
        let ident = format_ident!("{}_{}", fn_decl.ident, Self::IDENT_SUFFIX);
        let ty = self.type_factory.wrap_in(
            self.type_factory
                .create(mock_struct.item_struct.ident.clone()),
            constants::LAZY_LOCK_IDENT.clone(),
        );
        let block = self.mock_constructor_block_generator.generate_for_static(
            mock_struct,
            mock_data_struct,
            mock_setup_struct,
            mock_received_struct,
            base_caller_struct
        );
        let call = ExprCall {
            attrs: Vec::new(),
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: None,
                path: self.path_factory.create_from_parts(&[
                    constants::LAZY_LOCK_IDENT.clone(),
                    constants::NEW_IDENT.clone(),
                ]),
            })),
            paren_token: Default::default(),
            args: [Expr::Closure(ExprClosure {
                attrs: Vec::new(),
                lifetimes: None,
                constness: None,
                movability: None,
                asyncness: None,
                capture: None,
                or1_token: Default::default(),
                inputs: Punctuated::new(),
                or2_token: Default::default(),
                output: ReturnType::Default,
                body: Box::new(Expr::Block(ExprBlock {
                    attrs: Vec::new(),
                    label: None,
                    block,
                })),
            })]
            .into_iter()
            .collect(),
        };
        let item_static = ItemStatic {
            attrs: vec![constants::ALLOW_NON_UPPER_CASE_GLOBALS_ATTRIBUTE.clone()],
            vis: Visibility::Inherited,
            static_token: Default::default(),
            mutability: StaticMutability::None,
            ident,
            colon_token: Default::default(),
            ty: Box::new(ty),
            eq_token: Default::default(),
            expr: Box::new(Expr::Call(call)),
            semi_token: Default::default(),
        };
        let static_mock = StaticMock { item_static };
        return static_mock;
    }
}

impl StaticMockGenerator {
    const IDENT_SUFFIX: &'static str = "MOCK";
}
