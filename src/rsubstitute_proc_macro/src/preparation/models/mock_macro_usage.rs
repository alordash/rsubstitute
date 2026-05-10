pub(crate) enum MockMacroUsage {
    Simple,
    #[cfg(not(feature = "mock_base_by_default"))]
    WithBase,
    #[cfg(feature = "mock_base_by_default")]
    WithoutBase,
}
