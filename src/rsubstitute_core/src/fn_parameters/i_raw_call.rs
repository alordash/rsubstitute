use crate::args::IArgInfosProvider;
use crate::IGenericsHashKeyProvider;

pub trait IRawCall<'a>: IArgInfosProvider + IGenericsHashKeyProvider {
}

impl<'a, T: IArgInfosProvider + IGenericsHashKeyProvider + 'a> IRawCall<'a> for T {
    // fn clone_box(&self) -> Box<dyn IRawCall<'a> + 'a> {
    //     Box::new(self.clone())
    // }
}
