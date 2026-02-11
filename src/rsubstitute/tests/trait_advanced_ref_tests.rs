use rsubstitute::macros::mock;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

trait Trait<'a, 'b> {
    // fn accept_ref(&self, r: &'a &'b &'a &i32) -> &'b &'a &'b &'a i32;

    fn fooo<'c: 'a, 'd: 'b + 'a>(
        &mut self,
        Foo { mut number }: Foo,
        mut qq: &'d mut &'b mut &'a &'a &'c &'d mut i32,
    ) {
        println!("number: {number:?}")
    }
}
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct fooo_Call<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        arg_1: Foo,
        qq: *mut &'b mut &'a &'a &'c &'d mut i32,
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a> IArgInfosProvider
        for fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new("arg_1", self.arg_1.clone()),
                ArgInfo::new("qq", self.qq.clone()),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct fooo_ArgsChecker<
        '__rsubstitute_arg_field_lifetime: 'a + 'b,
        'a,
        'b,
        'c: 'a,
        'd: 'b + 'a,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        arg_1: Arg<Foo>,
        qq: Arg<&'d mut &'b mut &'a &'a &'c &'d mut i32>,
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a>
        IArgsChecker<fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>>
        for fooo_ArgsChecker<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        fn check(
            &self,
            call: &fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
        ) -> Vec<ArgCheckResult> {
            vec![
                self.arg_1.check("arg_1", &call.arg_1),
                self.qq.check_mut("qq", &call.qq),
            ]
        }
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a>
        IBaseCaller<fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>, ()>
        for TraitMock<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        fn call_base(&self, call: fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>) {
            let fooo_Call {
                arg_1: Foo { mut number },
                qq: mut qq,
                ..
            } = call;
            println!("number: {number:?}")
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<
        '__rsubstitute_arg_field_lifetime: 'a + 'b,
        'a,
        'b,
        'c: 'a,
        'd: 'b + 'a,
    > {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        fooo_data: FnData<
            TraitMock<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
            fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
            fooo_ArgsChecker<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
            (),
        >,
    }
    #[doc(hidden)]
    pub struct TraitMockSetup<
        '__rsubstitute_arg_field_lifetime: 'a + 'b,
        'a,
        'b,
        'c: 'a,
        'd: 'b + 'a,
    > {
        data: Arc<TraitMockData<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>>,
    }
    #[doc(hidden)]
    pub struct TraitMockReceived<
        '__rsubstitute_arg_field_lifetime: 'a + 'b,
        'a,
        'b,
        'c: 'a,
        'd: 'b + 'a,
    > {
        data: Arc<TraitMockData<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>>,
    }
    pub struct TraitMock<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a> {
        pub setup: TraitMockSetup<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
        pub received: TraitMockReceived<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
        data: Arc<TraitMockData<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>>,
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a> Trait<'a, 'b>
        for TraitMock<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        fn fooo(&mut self, arg_1: Foo, qq: &'d mut &'b mut &'a &'a &'c &'d mut i32) {
            let call = unsafe {
                fooo_Call {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    arg_1: std::mem::transmute(arg_1),
                    qq: std::mem::transmute(qq),
                }
            };
            self.data.fooo_data.handle_base(&self, call);
        }
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a>
        TraitMock<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        #[allow(unused)]
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                fooo_data: FnData::new("fooo", &SERVICES),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a>
        TraitMockSetup<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        #[allow(unused)]
        pub fn fooo(
            &'__rsubstitute_arg_field_lifetime self,
            arg_1: impl Into<Arg<Foo>>,
            qq: impl Into<Arg<&'d mut &'b mut &'a &'a &'c &'d mut i32>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            TraitMock<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
            fooo_Call<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
            fooo_ArgsChecker<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>,
            (),
            Self,
        > {
            let fooo_args_checker = fooo_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                arg_1: arg_1.into(),
                qq: qq.into(),
            };
            let fn_config = self.data.fooo_data.add_config(fooo_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime: 'a + 'b, 'a, 'b, 'c: 'a, 'd: 'b + 'a>
        TraitMockReceived<'__rsubstitute_arg_field_lifetime, 'a, 'b, 'c, 'd>
    {
        #[allow(unused)]
        pub fn fooo(
            &'__rsubstitute_arg_field_lifetime self,
            arg_1: impl Into<Arg<Foo>>,
            qq: impl Into<Arg<&'d mut &'b mut &'a &'a &'c &'d mut i32>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let fooo_args_checker = fooo_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                arg_1: arg_1.into(),
                qq: qq.into(),
            };
            self.data
                .fooo_data
                .verify_received(fooo_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use not_enough_asserts::panics::*;
    use rsubstitute::*;

    #[test]
    fn compile() {}

    #[test]
    fn flex() {
        let trait_mock = TraitMock::new();

        let v1 = 11;
        let r1 = &&&&v1;
        {
            let v2 = 23;
            let r2 = &&&&v2;

            trait_mock.setup.accept_ref(r2).returns(r1);

            let r = trait_mock.accept_ref(r2);
            assert_eq!(r1, r);

            trait_mock
                .received
                .accept_ref(r2, Times::Once)
                .no_other_calls();
        }
    }
}
