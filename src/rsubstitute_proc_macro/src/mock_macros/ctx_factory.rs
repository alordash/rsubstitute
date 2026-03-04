use crate::constants;
use crate::mock_macros::models::Ctx;
use proc_macro::TokenStream;

pub trait ICtxFactory {
    fn create(&self, proc_macro_attribute: TokenStream) -> Ctx;

    fn create_for_macro_mocked(&self) -> Ctx;

    #[cfg(not(feature = "support_base_by_default"))]
    fn create_for_macro_mocked_base(&self) -> Ctx;

    #[cfg(feature = "support_base_by_default")]
    fn create_for_macro_mocked_no_base(&self) -> Ctx;
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

        let support_base_calling = self.support_base_calling(&parameters);

        let ctx = Ctx {
            support_base_calling,
        };
        return ctx;
    }

    fn create_for_macro_mocked(&self) -> Ctx {
        #[cfg(not(feature = "support_base_by_default"))]
        let support_base_calling = false;
        #[cfg(feature = "support_base_by_default")]
        let support_base_calling = true;

        let ctx = Ctx {
            support_base_calling,
        };
        return ctx;
    }

    #[cfg(not(feature = "support_base_by_default"))]
    fn create_for_macro_mocked_base(&self) -> Ctx {
        Ctx {
            support_base_calling: true,
        }
    }

    #[cfg(feature = "support_base_by_default")]
    fn create_for_macro_mocked_no_base(&self) -> Ctx {
        Ctx {
            support_base_calling: false,
        }
    }
}

impl CtxFactory {
    fn support_base_calling(&self, parameters: &[&str]) -> bool {
        #[cfg(not(feature = "support_base_by_default"))]
        return parameters
            .iter()
            .any(|parameter| *parameter == constants::SUPPORT_BASE_PARAMETER);

        #[cfg(feature = "support_base_by_default")]
        return !parameters
            .iter()
            .any(|parameter| *parameter == constants::DO_NOT_SUPPORT_BASE_PARAMETER);
    }
}
