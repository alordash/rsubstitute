use crate::args_matching::ArgInfo;

pub trait IArgInfosProvider {
    fn get_fn_name(&self) -> &'static str;
    
    fn get_arg_infos(&self) -> Vec<ArgInfo>;
}
