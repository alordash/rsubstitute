use crate::args::*;
use crate::*;

pub trait ICall: IArgsInfosProvider + IGenericsHashKeyProvider + IArgsTupleProvider {}

impl<T: IArgsInfosProvider + IGenericsHashKeyProvider + IArgsTupleProvider> ICall for T {}
