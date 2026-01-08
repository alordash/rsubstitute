#[cfg(not(test))]
fn global(number: i32) -> String {
    return format!("REAL number is: {number}");
}
#[cfg(test)]
use __rsubstitute_generated_global::global;
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_global {
    use super::*;
    use rsubstitute::for_generated::*;
    
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct global_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        number: i32
    }
    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct global_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        number: Arg::<'__rsubstitute_arg_field_lifetime, i32>
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsChecker<global_Call<'__rsubstitute_arg_field_lifetime>> for global_ArgsChecker<'__rsubstitute_arg_field_lifetime> { fn check(&self, call: global_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> { vec![self.number.check("number", call.number)] } }
    
    #[allow(non_camel_case_types)]
    pub struct globalMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        global_data: FnData<global_Call<'__rsubstitute_arg_field_lifetime>, global_ArgsChecker<'__rsubstitute_arg_field_lifetime>, String>
    }
    #[allow(non_camel_case_types)]
    pub struct globalMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<globalMockData<'__rsubstitute_arg_field_lifetime>>
    }
    #[allow(non_camel_case_types)]
    pub struct globalMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<globalMockData<'__rsubstitute_arg_field_lifetime>>
    }
    impl<'__rsubstitute_arg_field_lifetime> globalMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn global(&'__rsubstitute_arg_field_lifetime self, number: Arg::<'__rsubstitute_arg_field_lifetime, i32>) -> SharedFnConfig<'__rsubstitute_arg_field_lifetime, global_Call, global_ArgsChecker, String, Self> {
            let global_args_checker = global_ArgsChecker { _phantom_lifetime: PhantomData, number };
            let fn_config = self.data.global_data.add_config(global_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> globalMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn global(&'__rsubstitute_arg_field_lifetime self, number: Arg::<'__rsubstitute_arg_field_lifetime, i32>, times: Times) -> &'__rsubstitute_arg_field_lifetime Self {
            let global_args_checker = global_ArgsChecker { _phantom_lifetime: PhantomData, number };
            self.data.global_data.verify_received(global_args_checker, times);
            return self;
        }
    }
    
    // TODO - add * globalMock
    // * impl Send/Sync for globalMock
    // * pub fn setup
    // * pub fn received
    // * static global_MOCK
    // * pub fn global()
}