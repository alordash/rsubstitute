use rsubstitute::macros::*;

// #[mock]
// trait Trait {
//     fn work(&self, v: i32) -> i32;
// }

trait Trait {
    fn work(&self, v: i32) -> i32;
}
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IGenericsHashKeyProvider, IArgsInfosProvider, IArgsTupleProvider)]
    pub struct work_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        v: i32,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct work_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        v: Arg<i32>,
    }
    impl<'rs> IArgsChecker<work_Call<'rs>> for work_ArgsChecker<'rs> {
        fn check(&self, call: &work_Call<'rs>) -> Vec<ArgCheckResult> {
            vec![self.v.check("v", &call.v)]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        work_data: FnData<TraitMock<'rs>, work_Call<'rs>, work_ArgsChecker<'rs>, i32>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct TraitMockSetup<'rs> {
        data: Arc<TraitMockData<'rs>>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct TraitMockReceived<'rs> {
        data: Arc<TraitMockData<'rs>>,
    }
    #[derive(Clone)]
    pub struct TraitMock<'rs> {
        setup: TraitMockSetup<'rs>,
        received: TraitMockReceived<'rs>,
        data: Arc<TraitMockData<'rs>>,
    }
    impl<'rs> Trait for TraitMock<'rs> {
        fn work(&self, v: i32) -> i32 {
            let call = unsafe {
                work_Call {
                    _phantom_lifetime: PhantomData,
                    v: core::mem::transmute(v),
                }
            };
            return self.data.work_data.handle_returning(call);
        }
    }
    impl<'rs> TraitMock<'rs> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_lifetime: PhantomData,
                work_data: FnData::new("work", &SERVICES),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
        pub fn setup<'__rsubstitute_config>(&self) -> TraitMockSetup<'__rsubstitute_config> {
            unsafe { core::mem::transmute(self.setup.clone()) }
        }
        pub fn received<'__rsubstitute_config>(&self) -> TraitMockReceived<'__rsubstitute_config> {
            unsafe { core::mem::transmute(self.received.clone()) }
        }
    }
    impl<'rs> TraitMockSetup<'rs> {
        pub fn work(
            &'rs self,
            v: impl Into<Arg<i32>>,
        ) -> FnTuner<'rs, TraitMock<'rs>, work_Call<'rs>, work_ArgsChecker<'rs>, i32, Self>
        {
            let work_args_checker = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                v: v.into(),
            };
            let fn_config = self.data.work_data.add_config(work_args_checker);
            let fn_tuner = FnTuner::new(fn_config, self);
            return fn_tuner;
        }
    }
    impl<'rs> TraitMockReceived<'rs> {
        pub fn work(self, v: impl Into<Arg<i32>>, times: Times) -> Self {
            let work_args_checker = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                v: v.into(),
            };
            self.data
                .work_data
                .verify_received(work_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'rs self) {
            self.data.verify_received_nothing_else();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args::Arg;

    #[test]
    fn my_test() {
        // Arrange
        let mock = TraitMock::new();
        let v1 = 10;
        let v2 = 20;
        let v3 = -31;
        let v4 = 1919;
        let v5 = 2022;

        let r1 = 111;
        let r2 = 222;
        let r3 = 333;
        let r45 = 50505;

        mock.setup
            .work(v1)
            .returns(r1)
            .work(v2)
            .returns(r2)
            .work(Arg::Is(|x| *x < 0))
            .returns(r3)
            .and_does(|args| println!("amogus received number: {}", args))
            .work(Arg::Any)
            .returns
            .always(r45);

        // Act
        let actual_r1 = mock.work(v1);
        let actual_r2 = mock.work(v2);
        let actual_r3 = mock.work(v3);
        let actual_r4 = mock.work(v4);
        let actual_r5 = mock.work(v5);

        // Assert
        assert_eq!(r1, actual_r1);
        assert_eq!(r2, actual_r2);
        assert_eq!(r3, actual_r3);
        assert_eq!(r45, actual_r4);
        assert_eq!(r45, actual_r5);

        mock.received
            .work(v1, Times::Once)
            .work(v2, Times::Once)
            .work(v3, Times::Once)
            .work(v4, Times::Once)
            .work(v5, Times::Once)
            .work(Arg::Any, Times::Exactly(5))
            .no_other_calls();
    }
}

fn main() {}
