use crate::args::*;
use crate::*;

pub trait ICall: IArgsInfosProvider + IGenericsInfoProvider + IArgsTupleProvider {}

impl<T: IArgsInfosProvider + IGenericsInfoProvider + IArgsTupleProvider> ICall for T {}
