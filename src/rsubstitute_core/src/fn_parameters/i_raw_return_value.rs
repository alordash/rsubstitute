pub trait IRawReturnValue<'a> {
    fn clone_box(&self) -> Box<dyn IRawReturnValue<'a> + 'a>;
}

impl<'a, T: Clone + 'a> IRawReturnValue<'a> for T {
    fn clone_box(&self) -> Box<dyn IRawReturnValue<'a> + 'a> {
        Box::new(self.clone())
    }
}
