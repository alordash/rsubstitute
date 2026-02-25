use crate::fn_parameters::DynReturnValue;

pub enum ReturnValueSource<'rs> {
    SingleTime(DynReturnValue<'rs>),
    Perpetual(Box<dyn Fn() -> DynReturnValue<'rs> + 'rs>),
}
