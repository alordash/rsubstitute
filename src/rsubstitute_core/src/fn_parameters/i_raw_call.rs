use crate::args_matching::IArgInfosProvider;
use crate::IGenericsHashKeyProvider;

pub trait IRawCall<'a>: IArgInfosProvider + IGenericsHashKeyProvider {
    fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a>;
}

impl<'a, T: IArgInfosProvider + IGenericsHashKeyProvider + Clone + 'a> IRawCall<'a> for T {
    fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a> {
        Box::new(self.clone())
    }
}
