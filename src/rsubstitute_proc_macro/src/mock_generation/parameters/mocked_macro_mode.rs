pub enum MockedMacroMode {
    Unspecified,
    #[cfg(not(feature = "mock_base_by_default"))]
    WithBase,
    #[cfg(feature = "mock_base_by_default")]
    WithoutBase,
}
