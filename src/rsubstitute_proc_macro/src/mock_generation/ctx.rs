use crate::constants;
use crate::mock_generation::models::Ctx;
use crate::mock_generation::parameters::*;
use proc_macro::TokenStream;

pub(crate) fn create(proc_macro_attribute: TokenStream) -> Ctx {
    if proc_macro_attribute.is_empty() {
        return Default::default();
    }

    let proc_macro_attribute_str = proc_macro_attribute.to_string();
    let parameters: Vec<_> = proc_macro_attribute_str
        .split(',')
        .map(|x| x.trim())
        .collect();

    let support_base_calling = support_base_calling_from_parameters(&parameters);

    let ctx = Ctx {
        support_base_calling,
    };
    return ctx;
}

pub(crate) fn create_for_macro_mocked(mocked_macro_target: MockedMacroMode) -> Ctx {
    let support_base_calling = match mocked_macro_target {
        MockedMacroMode::Unspecified => DEFAULT_SUPPORT_BASE_CALLING,
        #[cfg(not(feature = "mock_base_by_default"))]
        MockedMacroMode::WithBase => true,
        #[cfg(feature = "mock_base_by_default")]
        MockedMacroMode::WithoutBase => false,
    };
    let ctx = Ctx {
        support_base_calling,
    };
    return ctx;
}

fn support_base_calling_from_parameters(parameters: &[&str]) -> bool {
    #[cfg(not(feature = "mock_base_by_default"))]
    return parameters
        .iter()
        .any(|parameter| *parameter == constants::SUPPORT_BASE_PARAMETER);

    #[cfg(feature = "mock_base_by_default")]
    return !parameters
        .iter()
        .any(|parameter| *parameter == constants::DO_NOT_SUPPORT_BASE_PARAMETER);
}

#[cfg(not(feature = "mock_base_by_default"))]
const DEFAULT_SUPPORT_BASE_CALLING: bool = false;

#[cfg(feature = "mock_base_by_default")]
const DEFAULT_SUPPORT_BASE_CALLING: bool = true;
