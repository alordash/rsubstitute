#![allow(unused)]
use rsubstitute::macros::mock;

// #[mock]
// fn accept_ref<'a>(r: &'a i32) -> &'a i32 {
//     todo!()
// }

// #[mock]
// fn return_mut_ref<'a>() -> &'a mut i32 {
//     todo!()
// }

// TODO - support it
// #[mock]
// fn accept_mut_ref<'a>(mut r: i32) {
//     todo!()
// }

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}
#[cfg(not(test))]
fn accept_foo(Foo { number }: Foo) {
    let q = Foo { number };

    println!("number: ???");
}
#[cfg(test)]
pub use accept_foo::*;
#[cfg(test)]
#[allow(dead_code)]
#[allow(unused)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod accept_foo {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct accept_foo_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        arg_0: Foo,
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for accept_foo_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new("arg_0", self.arg_0.clone())]
        }
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct accept_foo_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        arg_0: Arg<Foo>,
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<accept_foo_Call<'__rsubstitute_arg_field_lifetime>>
        for accept_foo_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self,
            call: &accept_foo_Call<'__rsubstitute_arg_field_lifetime>,
        ) -> Vec<ArgCheckResult> {
            vec![self.arg_0.check("arg_0", &call.arg_0)]
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<accept_foo_Call<'__rsubstitute_arg_field_lifetime>, ()>
        for accept_fooMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: accept_foo_Call<'__rsubstitute_arg_field_lifetime>) {
            let accept_foo_Call {
                arg_0: Foo { number },
                ..
            } = call;
            let q = Foo { number };

            println!("number: ???");
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct accept_fooMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        accept_foo_data: FnData<
            accept_fooMock<'__rsubstitute_arg_field_lifetime>,
            accept_foo_Call<'__rsubstitute_arg_field_lifetime>,
            accept_foo_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
    }
    #[doc(hidden)]
    pub struct accept_fooMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<accept_fooMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[doc(hidden)]
    pub struct accept_fooMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<accept_fooMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[doc(hidden)]
    pub struct accept_fooMock<'__rsubstitute_arg_field_lifetime> {
        pub setup: accept_fooMockSetup<'__rsubstitute_arg_field_lifetime>,
        pub received: accept_fooMockReceived<'__rsubstitute_arg_field_lifetime>,
        data: Arc<accept_fooMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    unsafe impl<'__rsubstitute_arg_field_lifetime> Send
        for accept_fooMock<'__rsubstitute_arg_field_lifetime>
    {
    }
    unsafe impl<'__rsubstitute_arg_field_lifetime> Sync
        for accept_fooMock<'__rsubstitute_arg_field_lifetime>
    {
    }
    impl<'__rsubstitute_arg_field_lifetime> Default
        for accept_fooMock<'__rsubstitute_arg_field_lifetime>
    {
        fn default() -> Self {
            let data = Arc::new(accept_fooMockData {
                _phantom_lifetime: PhantomData,
                accept_foo_data: FnData::new("accept_foo", &SERVICES),
            });
            return accept_fooMock {
                setup: accept_fooMockSetup { data: data.clone() },
                received: accept_fooMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> accept_fooMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(unused)]
        pub fn setup(
            &'__rsubstitute_arg_field_lifetime self,
            arg_0: impl Into<Arg<Foo>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            accept_fooMock<'__rsubstitute_arg_field_lifetime>,
            accept_foo_Call<'__rsubstitute_arg_field_lifetime>,
            accept_foo_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
            Self,
        > {
            let accept_foo_args_checker = accept_foo_ArgsChecker {
                _phantom_lifetime: PhantomData,
                arg_0: arg_0.into(),
            };
            let fn_config = self
                .data
                .accept_foo_data
                .add_config(accept_foo_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> accept_fooMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(unused)]
        pub fn received(
            &'__rsubstitute_arg_field_lifetime self,
            arg_0: impl Into<Arg<Foo>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let accept_foo_args_checker = accept_foo_ArgsChecker {
                _phantom_lifetime: PhantomData,
                arg_0: arg_0.into(),
            };
            self.data
                .accept_foo_data
                .verify_received(accept_foo_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rsubstitute_arg_field_lifetime>(
        arg_0: impl Into<Arg<Foo>>,
    ) -> SharedFnConfig<
        '__rsubstitute_arg_field_lifetime,
        accept_fooMock<'__rsubstitute_arg_field_lifetime>,
        accept_foo_Call<'__rsubstitute_arg_field_lifetime>,
        accept_foo_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
        (),
        accept_fooMockSetup<'__rsubstitute_arg_field_lifetime>,
    > {
        let mock = get_global_mock::<accept_fooMock<'__rsubstitute_arg_field_lifetime>>();
        mock.data.accept_foo_data.reset();
        return mock.setup.setup(arg_0);
    }
    pub fn received<'__rsubstitute_arg_field_lifetime>(
        arg_0: impl Into<Arg<Foo>>,
        times: Times,
    ) -> &'__rsubstitute_arg_field_lifetime accept_fooMockReceived<'__rsubstitute_arg_field_lifetime>
    {
        return get_global_mock::<accept_fooMock<'__rsubstitute_arg_field_lifetime>>()
            .received
            .received(arg_0, times);
    }
    pub fn accept_foo<'__rsubstitute_arg_anonymous, '__rsubstitute_arg_field_lifetime>(
        Foo { number }: Foo,
    ) {
        let call = unsafe {
            accept_foo_Call {
                _phantom_lifetime: PhantomData,
                arg_0: std::mem::transmute(arg_0),
            }
        };
        let mock = get_global_mock::<accept_fooMock<'__rsubstitute_arg_field_lifetime>>();
        mock.data.accept_foo_data.handle_base(&mock, call);
    }
}

#[mock]
fn accept_many_ref<'a, 'b>(r: &'a &'b &'a &i32, _em: &()) -> &'a &'b &'a &'b i32 {
    todo!()
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
    fn accept_ref_test() {
        // let v1 = 1;
        // let v2 = 2;
        // let r1 = &v1;
        // {
        //     let r2 = &v2;
        //
        //     accept_ref::setup(r1).returns(r2);
        //
        //     accept_ref(r1);
        //
        //     accept_ref::received(r1, Times::Once).no_other_calls();
        // }
    }

    #[test]
    fn accept_many_ref_test() {
        let v1 = 11;
        let r1 = &&&&v1;
        {
            let v2 = 23;
            let r2 = &&&&v2;
            accept_many_ref::setup(r2, Arg::Any).returns(r1);

            let r = accept_many_ref(r2, &());

            assert_eq!(r1, r);

            accept_many_ref::received(r2, Arg::Any, Times::Once).no_other_calls();
        }
    }
}
