#[cfg(not(feature = "mock_base_by_default"))]
pub(crate) const SUPPORT_BASE_PARAMETER: &'static str = "base";

#[cfg(feature = "mock_base_by_default")]
pub(crate) const DO_NOT_SUPPORT_BASE_PARAMETER: &'static str = "no_base";

pub(crate) const DEFAULT_ARG_LIFETIME: &'static str = "__rsa";
pub(crate) const DEFAULT_ARG_LIFETIME_IDENT_STR: &'static str = "'__rsa";
