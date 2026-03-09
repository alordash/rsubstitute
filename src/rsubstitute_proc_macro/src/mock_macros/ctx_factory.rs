use crate::constants;
use crate::mock_macros::MockedMacroMode;
use crate::mock_macros::models::Ctx;
use proc_macro::TokenStream;

pub trait ICtxFactory {
    fn create(&self, proc_macro_attribute: TokenStream) -> Ctx;

    fn create_for_macro_mocked(&self, mocked_macro_target: MockedMacroMode) -> Ctx;
}

pub(crate) struct CtxFactory;

impl ICtxFactory for CtxFactory {
    fn create(&self, proc_macro_attribute: TokenStream) -> Ctx {
        if proc_macro_attribute.is_empty() {
            return Default::default();
        }

        let proc_macro_attribute_str = proc_macro_attribute.to_string();
        let parameters: Vec<_> = proc_macro_attribute_str
            .split(',')
            .map(|x| x.trim())
            .collect();

        let support_base_calling = self.support_base_calling_from_parameters(&parameters);

        let ctx = Ctx {
            support_base_calling,
        };
        return ctx;
    }

    fn create_for_macro_mocked(&self, mocked_macro_target: MockedMacroMode) -> Ctx {
        let support_base_calling = match mocked_macro_target {
            MockedMacroMode::Unspecified => self.default_support_base_calling(),
            #[cfg(not(feature = "support_base_by_default"))]
            MockedMacroMode::WithBase => true,
            #[cfg(feature = "support_base_by_default")]
            MockedMacroMode::WithoutBase => false,
        };
        let ctx = Ctx {
            support_base_calling,
        };
        return ctx;
    }
}

impl CtxFactory {
    fn support_base_calling_from_parameters(&self, parameters: &[&str]) -> bool {
        #[cfg(not(feature = "support_base_by_default"))]
        return parameters
            .iter()
            .any(|parameter| *parameter == constants::SUPPORT_BASE_PARAMETER);

        #[cfg(feature = "support_base_by_default")]
        return !parameters
            .iter()
            .any(|parameter| *parameter == constants::DO_NOT_SUPPORT_BASE_PARAMETER);
    }

    #[cfg(not(feature = "support_base_by_default"))]
    fn default_support_base_calling(&self) -> bool {
        false
    }

    #[cfg(feature = "support_base_by_default")]
    fn default_support_base_calling(&self) -> bool {
        true
    }
}
