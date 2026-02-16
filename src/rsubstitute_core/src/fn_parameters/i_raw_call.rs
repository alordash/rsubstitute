use crate::IGenericsHashKeyProvider;
use crate::args::IArgInfosProvider;

pub trait IRawCall<'a>: IArgInfosProvider + IGenericsHashKeyProvider {
    fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a>;
}

impl<'a, T: IArgInfosProvider + IGenericsHashKeyProvider + Clone + 'a> IRawCall<'a> for T {
    fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a> {
        Box::new(self.clone())
    }
}
