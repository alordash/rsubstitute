use crate::args_matching::ArgInfo;

pub trait IArgInfosProvider {
    fn get_arg_infos(&self) -> Vec<ArgInfo>;
}
