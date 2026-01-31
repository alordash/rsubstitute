trait Traitg {
    fn f(&self);
}
#[cfg(test)]
pub use __rsubstitute_generated_Traitg::*;
#[cfg(test)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Traitg {
    use super::*;
    use rsubstitute::for_generated::*;
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Clone)]
    pub struct f_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for f_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[derive(Debug, IArgsFormatter)]
    pub struct f_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsChecker<f_Call<'__rsubstitute_arg_field_lifetime>>
        for f_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: f_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }
    #[derive(IMockData)]
    pub struct TraitgMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        f_data: FnData<
            TraitgMock<'__rsubstitute_arg_field_lifetime>,
            f_Call<'__rsubstitute_arg_field_lifetime>,
            f_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    pub struct TraitgMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<TraitgMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    pub struct TraitgMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<TraitgMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct TraitgMock<'__rsubstitute_arg_field_lifetime> {
        pub setup: TraitgMockSetup<'__rsubstitute_arg_field_lifetime>,
        pub received: TraitgMockReceived<'__rsubstitute_arg_field_lifetime>,
        data: Arc<TraitgMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    impl<'__rsubstitute_arg_field_lifetime> Traitg for TraitgMock<'__rsubstitute_arg_field_lifetime> {
        fn f<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) {
            let call = unsafe {
                f_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            self.data.f_data.handle(call);
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> TraitgMock<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        pub fn new() -> Self {
            let data = Arc::new(TraitgMockData {
                _phantom_lifetime: PhantomData,
                f_data: FnData::new("f", &SERVICES),
            });
            return TraitgMock {
                setup: TraitgMockSetup { data: data.clone() },
                received: TraitgMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> TraitgMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn f(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            TraitgMock<'__rsubstitute_arg_field_lifetime>,
            f_Call<'__rsubstitute_arg_field_lifetime>,
            f_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let f_args_checker = f_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.f_data.add_config(f_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> TraitgMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn f(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let f_args_checker = f_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.f_data.verify_received(f_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
}
