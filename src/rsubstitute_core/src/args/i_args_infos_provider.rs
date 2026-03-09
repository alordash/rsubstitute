use crate::args::ArgInfo;

pub trait IArgsInfosProvider {
    fn get_arg_infos(&self) -> Vec<ArgInfo>;
}
