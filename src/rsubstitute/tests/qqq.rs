#[cfg(not(test))]
fn globals(number: i32) -> String {
    return format!("REAL number is: {number}");
}
#[cfg(test)]
use globals::globals;
#[allow(mismatched_lifetime_syntaxes)]
mod globals {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_globals(number: i32) -> String {
        return format!("REAL number is: {number}");
    }
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct globals_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        number: i32,
    }
    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct globals_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        number: Arg<'__rsubstitute_arg_field_lifetime, i32>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<globals_Call<'__rsubstitute_arg_field_lifetime>>
        for globals_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self,
            call: globals_Call<'__rsubstitute_arg_field_lifetime>,
        ) -> Vec<ArgCheckResult> {
            vec![self.number.check("number", call.number)]
        }
    }
    #[allow(non_camel_case_types)]
    pub struct globalsBaseCaller;
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<globals_Call<'__rsubstitute_arg_field_lifetime>, String> for globalsBaseCaller
    {
        fn call_base(&self, call: globals_Call<'__rsubstitute_arg_field_lifetime>) -> String {
            return base_globals(call.number);
        }
    }
    #[allow(non_camel_case_types)]
    pub struct globalsMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        globals_data: FnData<
            globals_Call<'__rsubstitute_arg_field_lifetime>,
            globals_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            String,
            globalsBaseCaller,
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct globalsMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<globalsMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct globalsMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<globalsMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    pub struct globalsMock<'__rsubstitute_arg_field_lifetime> {
        pub setup: globalsMockSetup<'__rsubstitute_arg_field_lifetime>,
        pub received: globalsMockReceived<'__rsubstitute_arg_field_lifetime>,
        data: Arc<globalsMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    unsafe impl<'__rsubstitute_arg_field_lifetime> Send
        for globalsMock<'__rsubstitute_arg_field_lifetime>
    {
    }
    unsafe impl<'__rsubstitute_arg_field_lifetime> Sync
        for globalsMock<'__rsubstitute_arg_field_lifetime>
    {
    }
    impl<'__rsubstitute_arg_field_lifetime> globalsMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
            number: Arg<'__rsubstitute_arg_field_lifetime, i32>,
        ) -> SharedFnConfig<
            'static,
            globals_Call<'static>,
            globals_ArgsChecker<'static>,
            String,
            globalsMockSetup<'static>,
            globalsBaseCaller,
        > {
            let globals_args_checker = globals_ArgsChecker {
                _phantom_lifetime: PhantomData,
                number,
            };
            let fn_config = self.data.globals_data.add_config(globals_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self, None);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> globalsMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            number: Arg<'__rsubstitute_arg_field_lifetime, i32>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let globals_args_checker = globals_ArgsChecker {
                _phantom_lifetime: PhantomData,
                number,
            };
            self.data
                .globals_data
                .verify_received(globals_args_checker, times);
            return self;
        }
    }
    #[allow(non_upper_case_globals)]
    static globals_MOCK: LazyLock<globalsMock> = LazyLock::new(|| {
        let data = Arc::new(globalsMockData {
            _phantom_lifetime: PhantomData,
            globals_data: FnData::new("globals_data", &SERVICES),
        });
        return globalsMock {
            setup: globalsMockSetup { data: data.clone() },
            received: globalsMockReceived { data: data.clone() },
            data,
        };
    });
    pub fn setup(
        number: Arg<'static, i32>,
    ) -> SharedFnConfig<
        'static,
        globals_Call<'static>,
        globals_ArgsChecker<'static>,
        String,
        globalsMockSetup<'static>,
        globalsBaseCaller,
    > {
        return globals_MOCK.setup.setup(number);
    }
    pub fn received(
        number: Arg<'static, i32>,
        times: Times,
    ) -> &'static globalsMockReceived<'static> {
        return globals_MOCK.received.received(number, times);
    }
    pub fn globals<'__rsubstitute_arg_anonymous>(number: i32) -> String {
        let call = unsafe {
            globals_Call {
                _phantom_lifetime: PhantomData,
                number: std::mem::transmute(number),
            }
        };
        return globals_MOCK.data.globals_data.handle_returning(call);
    }
}
