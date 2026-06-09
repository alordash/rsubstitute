use crate::args::*;

pub trait ICall: IArgsProvider + IGenericsInfoProvider {}

impl<T: IArgsProvider + IGenericsInfoProvider> ICall for T {}
