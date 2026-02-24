use crate::IGenericsHashKeyProvider;
use crate::args::IArgInfosProvider;

pub trait ICall: IArgInfosProvider + IGenericsHashKeyProvider {}

impl<T: IArgInfosProvider + IGenericsHashKeyProvider> ICall for T {}
