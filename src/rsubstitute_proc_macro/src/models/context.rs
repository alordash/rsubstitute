pub(crate) struct Context {
    pub support_base_calling: bool,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            support_base_calling: false,
        }
    }
}
