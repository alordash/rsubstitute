use crate::args::*;
use crate::mock_data::*;
use crate::*;

pub fn new(fn_name: &'static str) -> DynFnDataHolder {
    return DynFnDataHolder::new(fn_name);
}

pub trait IDynBaseFnData: IDynFnData {
    fn handle_base(&self, dyn_mock: DynFnDataParam, dyn_call: DynFnDataParam);
    fn handle_base_returning(
        &self,
        dyn_mock: DynFnDataParam,
        dyn_call: DynFnDataParam,
    ) -> DynFnDataParam;
}


impl<TMock, TCall, TReturnType, TArgsChecker> IDynBaseFnData
for FnData<TMock, TCall, TReturnType, TArgsChecker>
where
    TMock: IBaseCaller<TCall, TReturnType>,
    TCall: IGenericsHashKeyProvider + IArgInfosProvider + Clone,
    TArgsChecker: IArgsChecker<TCall>,
    TReturnType: Clone, {
    fn handle_base(&self, dyn_mock: DynFnDataParam, dyn_call: DynFnDataParam) {
        let mock: &TMock = dyn_mock.downcast_into();
        let call: TCall = dyn_call.downcast_into();
        FnData::handle_base(self, mock, call);
    }

    fn handle_base_returning(&self, dyn_mock: DynFnDataParam, dyn_call: DynFnDataParam) -> DynFnDataParam {
        let mock: &TMock = dyn_mock.downcast_into();
        let call: TCall = dyn_call.downcast_into();
        let return_value = FnData::handle_base(self, mock, call);
        return return_value.upcast_into();
    }
}