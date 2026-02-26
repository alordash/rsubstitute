use crate::args::ArgInfo;

// TODO - ignore PhantomData fields
pub trait IArgInfosProvider {
    fn get_arg_infos(&self) -> Vec<ArgInfo>;
}
