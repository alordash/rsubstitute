use crate::args::*;

pub trait ICall: IArgsInfosProvider + IGenericsInfoProvider + IArgsTupleProvider {}

impl<T: IArgsInfosProvider + IGenericsInfoProvider + IArgsTupleProvider> ICall for T {}
