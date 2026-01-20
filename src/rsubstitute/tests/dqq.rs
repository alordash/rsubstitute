#[cfg(not(test))]
fn fsd<T>(value: T) -> T {
    return value;
}
#[cfg(test)]
use fsd::fsd;
#[allow(mismatched_lifetime_syntaxes)]
mod fsd {
    use super::*;
    use rsubstitute::for_generated::*;
    fn base_fsd<T>(value: T) -> T {
        return value;
    }
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct fsd_Call<
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
    > IArgInfosProvider for fsd_Call<'__rsubstitute_arg_field_lifetime, T>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new("value", self.value.clone())]
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct fsd_ArgsChecker<
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
    > IArgsChecker<fsd_Call<'__rsubstitute_arg_field_lifetime, T>>
        for fsd_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>
    {
        fn check(
            &self,
            call: fsd_Call<'__rsubstitute_arg_field_lifetime, T>,
        ) -> Vec<ArgCheckResult> {
            vec![self.value.check("value", call.value)]
        }
    }
    #[allow(non_camel_case_types)]
    pub struct fsdBaseCaller;
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > IBaseCaller<fsd_Call<'__rsubstitute_arg_field_lifetime, T>, T> for fsdBaseCaller
    {
        fn call_base(&self, call: fsd_Call<'__rsubstitute_arg_field_lifetime, T>) -> T {
            return base_fsd(call._phantom_T, call.value);
        }
    }
    #[allow(non_camel_case_types)]
    #[derive(IMockData)]
    pub struct fsdMockData<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        base_caller: Arc<RefCell<fsdBaseCaller>>,
        fsd_data: FnData<
            fsd_Call<'__rsubstitute_arg_field_lifetime, T>,
            fsd_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
            T,
            fsdBaseCaller,
        >,
    }
    #[allow(non_camel_case_types)]
    pub struct fsdMockSetup<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        data: Arc<fsdMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    #[allow(non_camel_case_types)]
    pub struct fsdMockReceived<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > {
        data: Arc<fsdMockData<'__rsubstitute_arg_field_lifetime, T>>,
    }
    #[allow(non_camel_case_types)]
    pub struct fsdMock<T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone + 'static> {
        pub setup: fsdMockSetup<'static, T>,
        pub received: fsdMockReceived<'static, T>,
        data: Arc<fsdMockData<'static, T>>,
    }
    unsafe impl<T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone> Send for fsdMock<T> {}
    unsafe impl<T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone> Sync for fsdMock<T> {}
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > fsdMockSetup<'__rsubstitute_arg_field_lifetime, T>
    {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<'__rsubstitute_arg_field_lifetime, T>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            fsd_Call<'__rsubstitute_arg_field_lifetime, T>,
            fsd_ArgsChecker<'__rsubstitute_arg_field_lifetime, T>,
            T,
            Self,
            fsdBaseCaller,
        > {
            let fsd_args_checker = fsd_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            let fn_config = self.data.fsd_data.add_config(fsd_args_checker);
            let shared_fn_config =
                SharedFnConfig::new(fn_config, self, Some(self.data.base_caller.clone()));
            return shared_fn_config;
        }
    }
    impl<
        '__rsubstitute_arg_field_lifetime,
        T: std::fmt::Debug + core::cmp::PartialOrd + core::clone::Clone,
    > fsdMockReceived<'__rsubstitute_arg_field_lifetime, T>
    {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<'__rsubstitute_arg_field_lifetime, T>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let fsd_args_checker = fsd_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: value.into(),
            };
            self.data.fsd_data.verify_received(fsd_args_checker, times);
            return self;
        }
        pub fn only(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    thread_local! {# [allow (non_upper_case_globals )]static fsd_MOCK : LazyLock < fsdMock > = LazyLock :: new (| | {let data = Arc :: new (fsdMockData {_phantom_lifetime : PhantomData , base_caller : Arc :: new (RefCell :: new (fsdBaseCaller )), fsd_data : FnData :: new ("fsd" , & SERVICES )}); return fsdMock {setup : fsdMockSetup {data : data . clone ()}, received : fsdMockReceived {data : data . clone ()}, data }; }); }
    pub fn setup(
        value: impl Into<Arg<'static, T>>,
    ) -> SharedFnConfig<
        'static,
        fsd_Call<'static, T>,
        fsd_ArgsChecker<'static, T>,
        T,
        fsdMockSetup<'static, T>,
        fsdBaseCaller,
    > {
        fsd_MOCK.as_static().data.fsd_data.reset();
        return fsd_MOCK.as_static().setup.setup(_phantom_T, value);
    }
    pub fn received(
        value: impl Into<Arg<'static, T>>,
        times: Times,
    ) -> &'static fsdMockReceived<'static, T> {
        return fsd_MOCK
            .as_static()
            .received
            .received(_phantom_T, value, times);
    }
    pub fn fsd<'__rsubstitute_arg_anonymous>(value: T) -> T {
        let call = unsafe {
            fsd_Call {
                _phantom_lifetime: PhantomData,
                _phantom_T: PhantomData,
                value: std::mem::transmute(value),
            }
        };
        return fsd_MOCK
            .as_static()
            .data
            .fsd_data
            .handle_base_returning(call);
    }
}
