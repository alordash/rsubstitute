use crate::args::*;
use crate::di::ServiceCollection;
use crate::mock_data::*;
use crate::*;

pub fn new<TMock, TCall, TReturnType, TArgsChecker>(fn_name: &'static str) -> DynFnDataHolder
where
    TCall: IGenericsHashKeyProvider + IArgInfosProvider,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnType: Clone,
{
    return DynFnDataHolder::new(fn_name);
}

pub trait IDynFnData {
    fn add_config(&self, dyn_args_checker: DynFnDataParam) -> DynFnDataParam;
    fn verify_received(&self, dyn_args_checker: DynFnDataParam, times: Times);
    fn get_unexpected_calls_error_msgs(&self) -> Vec<String>;
    fn handle(&self, dyn_call: DynFnDataParam);
    fn handle_returning(&self, dyn_call: DynFnDataParam) -> DynFnDataParam;
}

impl<TMock, TCall, TReturnType, TArgsChecker> IDynFnData
    for FnData<TMock, TCall, TReturnType, TArgsChecker>
where
    TCall: IGenericsHashKeyProvider + IArgInfosProvider,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnType: Clone,
{
    fn add_config(&self, dyn_args_checker: DynFnDataParam) -> DynFnDataParam {
        let args_checker: TArgsChecker = dyn_args_checker.downcast_into();
        let config = FnData::add_config(self, args_checker);
        return config.upcast_into();
    }

    fn verify_received(&self, dyn_args_checker: DynFnDataParam, times: Times) {
        let args_checker: TArgsChecker = dyn_args_checker.downcast_into();
        FnData::verify_received(self, args_checker, times);
    }

    fn get_unexpected_calls_error_msgs(&self) -> Vec<String> {
        FnData::get_unexpected_calls_error_msgs(self)
    }

    fn handle(&self, dyn_call: DynFnDataParam) {
        let call: TCall = dyn_call.downcast_into();
        FnData::handle(self, call);
    }

    fn handle_returning(&self, dyn_call: DynFnDataParam) -> DynFnDataParam {
        let call: TCall = dyn_call.downcast_into();
        let return_value = FnData::handle_returning(self, call);
        return return_value.upcast_into();
    }
}
