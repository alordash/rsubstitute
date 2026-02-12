use crate::args_matching::IArgInfosProvider;

pub trait IRawCall<'a>: IArgInfosProvider {
    fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a>;
}

impl<'a, T: IArgInfosProvider + Clone + 'a> IRawCall<'a> for T {
    fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a> {
        Box::new(self.clone())
    }
}
