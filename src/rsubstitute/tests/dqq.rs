#[cfg(not(test))]
fn qqwe<T>(value: T) -> T {
    return value;
}
#[cfg(test)]
use qqwe::qqwe;
#[allow(mismatched_lifetime_syntaxes)]
mod qqwe {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_qqwe<T>(value: T) -> T {
        return value;
    }
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct qqwe_Call<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        _phantom_T: PhantomData<T>,
        value: T,
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IArgInfosProvider for qqwe_Call<'__rsubstitute_arg_field_lifetime, T>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new("value", self.value.clone())]
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct qqwe_ArgsChecker<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        _phantom_T: PhantomData<T>,
        value: Arg<'__rsubstitute_arg_field_lifetime, T>,
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IArgsChecker<qqwe_Call<'__rsubstitute_arg_field_lifetime, T>>
        for qqwe_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>
    {
        fn check(
            &self,
            call: qqwe_Call<'__rsubstitute_arg_field_lifetime, T>,
        ) -> Vec<ArgCheckResult> {
            vec![self.value.check("value", call.value)]
        }
    }
    #[allow(non_camel_case_types)]
    pub struct qqweBaseCaller<T> {
        _phantom_T: PhantomData<T>,
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IBaseCaller<qqwe_Call<'__rsubstitute_arg_field_lifetime, T>, T> for qqweBaseCaller<T>
    {
        fn call_base(&self, call: qqwe_Call<'__rsubstitute_arg_field_lifetime, T>) -> T {
            return base_qqwe(call.value);
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct qqweMockData<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        base_caller: Arc<RefCell<qqweBaseCaller<T>>>,
        qqwe_data: FnData<
            qqwe_Call<'__rsubstitute_arg_field_lifetime, T>,
            qqwe_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
            T,
            qqweBaseCaller<T>,
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct qqweMockSetup<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        data: Arc<qqweMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    #[allow(non_camel_case_types)]
    pub struct qqweMockReceived<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        data: Arc<qqweMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    #[allow(non_camel_case_types)]
    pub struct qqweMock<T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone> {
        pub setup: qqweMockSetup<'static, T>,
        pub received: qqweMockReceived<'static, T>,
        data: Arc<qqweMockData<'static, T>>,
    }
    unsafe impl<T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone> Send for qqweMock<T> {}
    unsafe impl<T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone> Sync for qqweMock<T> {}
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > qqweMockSetup<'__rsubstitute_arg_field_lifetime, T>
    {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<'__rsubstitute_arg_field_lifetime, T>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            qqwe_Call<'__rsubstitute_arg_field_lifetime, T>,
            qqwe_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
            T,
            Self,
            qqweBaseCaller<T>,
        > {
            let qqwe_args_checker = qqwe_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            let fn_config = self.data.qqwe_data.add_config(qqwe_args_checker);
            let shared_fn_config =
                SharedFnConfig::new(fn_config, self, Some(self.data.base_caller.clone()));
            return shared_fn_config;
        }
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > qqweMockReceived<'__rsubstitute_arg_field_lifetime, T>
    {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<'__rsubstitute_arg_field_lifetime, T>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let qqwe_args_checker = qqwe_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            self.data
                .qqwe_data
                .verify_received(qqwe_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    >(
        value: impl Into<Arg<'static, T>>,
    ) -> SharedFnConfig<
        'static,
        qqwe_Call<'static, T>,
        qqwe_ArgsChecker<'static, T>,
        T,
        qqweMockSetup<'static, T>,
        qqweBaseCaller<T>,
    > {
        let mock = get_global_mock::<qqweMock<T>>();
        mock.data.qqwe_data.reset();
        return mock.setup.setup(value);
    }
    pub fn received<T>(
        value: impl Into<Arg<'static, T>>,
        times: Times,
    ) -> &'static qqweMockReceived<'static, T> {
        return get_global_mock::<qqweMock<T>>().received.received(value, times);
    }
    pub fn qqwe<
        '__rsubstitute_arg_anonymous,
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    >(
        value: T,
    ) -> T {
        let call = unsafe {
            qqwe_Call {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: std::mem::transmute(value),
            }
        };
        return get_global_mock::<qqweMock<T>>()
            .data
            .qqwe_data
            .handle_base_returning(call);
    }
}
