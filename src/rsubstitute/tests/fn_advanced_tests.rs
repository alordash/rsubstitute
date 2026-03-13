#![allow(unused)]
use rsubstitute::macros::mock;

#[mock]
fn accept_ref<'a>(r: &'a i32) -> &'a i32 {
    unreachable!()
}

#[mock]
fn return_mut_ref<'a>() -> &'a mut i32 {
    unreachable!()
}

#[mock]
fn accept_mut_ref<'a>(mut r: i32) -> i32 {
    r += 1;
    return r;
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

#[mock]
fn accept_foo(Foo { mut number }: Foo) {
    let q = Foo { number };

    println!("number: ???");
}

// #[mock]
// fn accept_many_ref<'a, 'b>(mut r: &'a &'b &'a &i32, _em: &()) -> &'a &'b &'a &'b i32 {
//     unreachable!()
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn compile() {}

    #[test]
    fn accept_ref_test() {
        let v1 = 1;
        let v2 = 2;
        let r1 = &v1;
        {
            let r2 = &v2;

            accept_ref::setup(r1).returns(r2);

            accept_ref(r1);

            accept_ref::received(r1, Times::Once).no_other_calls();
        }
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

#[cfg(not(test))]
fn accept_many_ref<'a, 'b>(mut r: &'a &'b &'a &i32, _em: &()) -> &'a &'b &'a &'b i32 {
    unreachable!()
}
#[cfg(test)]
pub use accept_many_ref::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod accept_many_ref {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct accept_many_ref_Call<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        r: &'a &'b &'a &'__rs i32,
        _em: &'__rs (),
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct accept_many_ref_ArgsChecker<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        r: Arg<'__rs, &'a &'b &'a &'__rs i32>,
        _em: Arg<'__rs, &'__rs ()>,
    }
    impl<'__rs, 'a, 'b> IArgsChecker for accept_many_ref_ArgsChecker<'__rs, 'a, 'b> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &accept_many_ref_Call<'__rs, 'a, 'b> = dyn_call.downcast_ref();
            vec![
                self.r
                    .check_ref("r", &call.r, (&ArgPrinter(&&call.r)).debug_string()),
                self._em
                    .check_ref("_em", &call._em, (&ArgPrinter(&&call._em)).debug_string()),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct accept_many_refMockData<'__rs, 'a, 'b> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom_a: PhantomData<&'a ()>,
        _phantom_b: PhantomData<&'b ()>,
        pub accept_many_ref: FnData<'__rs, accept_many_refMock<'__rs, 'a, 'b>, false, false>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct accept_many_refMockSetup<'__rs, 'a, 'b> {
        data: Arc<accept_many_refMockData<'__rs, 'a, 'b>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct accept_many_refMockReceived<'__rs, 'a, 'b> {
        data: Arc<accept_many_refMockData<'__rs, 'a, 'b>>,
    }
    #[doc(hidden)]
    pub struct accept_many_refMock<'__rs, 'a, 'b> {
        pub setup: accept_many_refMockSetup<'__rs, 'a, 'b>,
        pub received: accept_many_refMockReceived<'__rs, 'a, 'b>,
        pub data: Arc<accept_many_refMockData<'__rs, 'a, 'b>>,
    }
    impl<'__rs, 'a, 'b> Default for accept_many_refMock<'__rs, 'a, 'b> {
        fn default() -> Self {
            let data = Arc::new(accept_many_refMockData {
                _phantom_lifetime: PhantomData,
                _phantom_a: PhantomData,
                _phantom_b: PhantomData,
                accept_many_ref: FnData::new("accept_many_ref"),
            });
            return accept_many_refMock {
                setup: accept_many_refMockSetup { data: data.clone() },
                received: accept_many_refMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rs, 'a, 'b> accept_many_refMockSetup<'__rs, 'a, 'b> {
        pub fn setup<'__rsa: 'a + 'b>(
            &self,
            r: impl Into<Arg<'__rsa, &'a &'b &'a &'__rsa i32>>,
            _em: impl Into<Arg<'__rsa, &'__rsa ()>>,
        ) -> FnTuner<
            '_,
            accept_many_refMock<'__rs, 'a, 'b>,
            Self,
            (&'__rs &'a &'b &'a &'__rs i32, &'__rs &'__rs ()),
            &'a &'b &'a &'b i32,
            false,
            false,
        > {
            let accept_many_ref_args_checker: accept_many_ref_ArgsChecker<'_, 'a, 'b> =
                accept_many_ref_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    r: r.into(),
                    _em: _em.into(),
                };
            let fn_tuner: FnTuner<
                '_,
                accept_many_refMock<'__rs, 'a, 'b>,
                Self,
                (&'__rs &'a &'b &'a &'__rs i32, &'__rs &'__rs ()),
                &'a &'b &'a &'b i32,
                false,
                false,
            > = self
                .data
                .accept_many_ref
                .add_config(accept_many_ref_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs, 'a, 'b> accept_many_refMockReceived<'__rs, 'a, 'b> {
        pub fn received<'__rsa: 'a + 'b>(
            &self,
            r: impl Into<Arg<'__rsa, &'a &'b &'a &'__rsa i32>>,
            _em: impl Into<Arg<'__rsa, &'__rsa ()>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rs &'a &'b &'a &'__rs i32, &'__rs &'__rs ())> {
            let accept_many_ref_args_checker: accept_many_ref_ArgsChecker<'_, 'a, 'b> =
                accept_many_ref_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom_a: PhantomData,
                    _phantom_b: PhantomData,
                    r: r.into(),
                    _em: _em.into(),
                };
            self.data
                .accept_many_ref
                .verify_received(accept_many_ref_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub fn setup<'__rs, 'a, 'b>(
        r: impl Into<Arg<'__rs, &'a &'b &'a &'__rs i32>>,
        _em: impl Into<Arg<'__rs, &'__rs ()>>,
    ) -> FnTuner<
        '__rs,
        accept_many_refMock<'__rs, 'a, 'b>,
        accept_many_refMockSetup<'__rs, 'a, 'b>,
        (&'__rs &'a &'b &'a &'__rs i32, &'__rs &'__rs ()),
        &'a &'b &'a &'b i32,
        false,
        false,
    > {
        let mock = get_global_mock::<accept_many_refMock<'__rs, 'a, 'b>>();
        mock.data.accept_many_ref.reset();
        return mock.setup.setup(r, _em);
    }
    pub fn received<'__rsa: 'a + 'b, '__rs, 'a, 'b>(
        r: impl Into<Arg<'__rsa, &'a &'b &'a &'__rsa i32>>,
        _em: impl Into<Arg<'__rsa, &'__rsa ()>>,
        times: Times,
    ) -> FnVerifier<
        accept_many_refMockReceived<'__rs, 'a, 'b>,
        (&'__rs &'a &'b &'a &'__rs i32, &'__rs &'__rs ()),
    > {
        return get_global_mock::<accept_many_refMock<'__rs, 'a, 'b>>()
            .received
            .clone()
            .received(r, _em, times);
    }
    pub fn accept_many_ref<'__rsa, '__rs, 'a, 'b>(
        r: &'a &'b &'a &i32,
        _em: &(),
    ) -> &'a &'b &'a &'b i32 {
        let call: accept_many_ref_Call<'_, 'a, 'b> = accept_many_ref_Call {
            _phantom_lifetime: PhantomData,
            _phantom_a: PhantomData,
            _phantom_b: PhantomData,
            r: transmute_lifetime!(r),
            _em: transmute_lifetime!(_em),
        };
        let mock = get_global_mock::<accept_many_refMock<'__rs, 'a, 'b>>();
        return mock.data.accept_many_ref.handle_returning(&mock, call);
    }
}
