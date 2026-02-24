pub trait IReturnValue<'rs> {
    fn clone_box(&self) -> Box<dyn IReturnValue<'rs> + 'rs>;
}

impl<'rs, T: Clone + 'rs> IReturnValue<'rs> for T {
    fn clone_box(&self) -> Box<dyn IReturnValue<'rs> + 'rs> {
        Box::new(self.clone())
    }
}
