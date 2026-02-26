use rsubstitute::macros::*;

// #[mock]
// trait Trait {
//     fn work<'a>(&self, v: &'a i32) -> &'a i32;
// }

trait Trait {
    fn work<'a>(&self, v: &'a i32) -> &'a i32;
}
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;

    #[derive(Clone, IGenericsHashKeyProvider, IArgInfosProvider)]
    pub struct work_Call<'rs, 'a> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        v: &'a i32,
    }

    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct work_ArgsChecker<'rs, 'a> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        v: Arg<&'a i32>,
    }
    impl<'rs, 'a> IArgsChecker for work_ArgsChecker<'rs, 'a> {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call = dyn_call.downcast_ref();
            vec![self.v.check("v", &call.v)]
        }
    }

    #[derive(IMockData)]
    pub struct TraitMockData<'rs> {
        work_data: FnData<'rs, TraitMock<'rs>>,
    }

    #[derive(Clone)]
    pub struct TraitMockSetup<'rs> {
        data: Arc<TraitMockData<'rs>>,
    }

    #[derive(Clone)]
    pub struct TraitMockReceived<'rs> {
        data: Arc<TraitMockData<'rs>>,
    }
    #[derive(Clone)]
    pub struct TraitMock<'rs> {
        pub setup: TraitMockSetup<'rs>,
        pub received: TraitMockReceived<'rs>,
        data: Arc<TraitMockData<'rs>>,
    }
    impl<'rs> Trait for TraitMock<'rs> {
        fn work<'a>(&self, v: &'a i32) -> &'a i32 {
            let call = unsafe {
                work_Call {
                    _phantom_lifetime: PhantomData,
                    v: std::mem::transmute(v),
                }
            };
            return self.data.work_data.handle_returning(call);
        }
    }
    impl<'rs> TraitMock<'rs> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                work_data: FnData::new("work"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'rs> TraitMockSetup<'rs> {
        pub fn work<'a: 'rs>(
            // Notice: 'a: 'rs only in setup, not needed in received
            &self,
            v: impl Into<Arg<&'a i32>>,
        ) -> FnTuner<'rs, Self, TraitMock, &'a i32> {
            let work_args_checker = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                v: v.into(),
            };
            let fn_tuner: FnTuner<'_, _, _, &'a i32> =
                self.data.work_data.add_config(work_args_checker, self);
            return unsafe { std::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs> TraitMockReceived<'rs> {
        pub fn work<'a>(self, v: impl Into<Arg<&'a i32>>, times: Times) -> Self {
            let work_args_checker: work_ArgsChecker<'rs, 'a> = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                v: v.into(),
            };
            self.data
                .work_data
                .verify_received(work_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&self) {
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
        let v1 = &10;
        let v2 = &20;
        let v3 = &-31;

        let r1 = &111;
        let r2 = &222;
        let r3 = &333;

        mock.setup
            .work(v1)
            .returns(r1)
            .work(v2)
            .returns(r2)
            .work(Arg::Is(|x: &&i32| **x < 0))
            .returns(r3);

        // Act
        let actual_r1 = mock.work(v1);
        let actual_r2 = mock.work(v2);
        let actual_r3 = mock.work(v3);

        // Assert
        assert_eq!(r1, actual_r1);
        assert_eq!(r2, actual_r2);
        assert_eq!(r3, actual_r3);

        mock.received
            .work(v1, Times::Once)
            .work(v2, Times::Once)
            .work(v3, Times::Once)
            .work(Arg::Any, Times::Exactly(3))
            .no_other_calls();
    }
}

fn main() {}
