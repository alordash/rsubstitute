use crate::IGenericsHashKeyProvider;
use crate::args::IArgInfosProvider;
use crate::fn_parameters::IArgsTupleProvider;

pub trait ICall: IArgInfosProvider + IGenericsHashKeyProvider + IArgsTupleProvider {}

impl<T: IArgInfosProvider + IGenericsHashKeyProvider + IArgsTupleProvider> ICall for T {}
