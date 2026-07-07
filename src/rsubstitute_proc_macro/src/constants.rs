#[cfg(not(feature = "mock_base_by_default"))]
pub(crate) const SUPPORT_BASE_PARAMETER: &'static str = "base";

#[cfg(feature = "mock_base_by_default")]
pub(crate) const DO_NOT_SUPPORT_BASE_PARAMETER: &'static str = "no_base";

pub(crate) const IDENTS_SEPARATOR: &'static str = "_";
