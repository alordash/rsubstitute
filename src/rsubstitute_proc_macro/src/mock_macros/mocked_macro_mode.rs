pub(crate) enum MockedMacroMode {
    Unspecified,
    #[cfg(not(feature = "support_base_by_default"))]
    WithBase,
    #[cfg(feature = "support_base_by_default")]
    WithoutBase,
}
