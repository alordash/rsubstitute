pub(crate) struct Ctx {
    pub support_base_calling: bool,
}

impl Default for Ctx {
    fn default() -> Self {
        Self {
            support_base_calling: false,
        }
    }
}
