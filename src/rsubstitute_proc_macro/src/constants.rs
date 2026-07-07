#[cfg(not(feature = "mock_base_by_default"))]
pub(crate) const SUPPORT_BASE_PARAMETER: &'static str = "base";

#[cfg(feature = "mock_base_by_default")]
pub(crate) const DO_NOT_SUPPORT_BASE_PARAMETER: &'static str = "no_base";

pub(crate) const IDENTS_SEPARATOR: &'static str = "_";

pub(crate) const FN_SHOULD_HAVE_BASE_IMPL_MSG: &'static str =
    "`fn`s should have base implementation if mocking with `base`.";
