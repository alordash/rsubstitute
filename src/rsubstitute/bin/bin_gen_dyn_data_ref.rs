use rsubstitute::macros::*;

// mod sandbox {
//     use rsubstitute::macros::*;
//     #[mock]
//     trait Trait<'b> {
//         fn work<'a: 'b>(&self, v: &'a i32) -> &'a i32;
//     }
// }

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
            .and_does(|args| println!("accepted v2, v2 arg = {}", args))
            .work(Arg::Is(|x: &&i32| **x < 0))
            .returns(r3);

        // {
        //     let q = 12;
        //     let r = &q;
        //     mock.work(r);
        // }

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

#[cfg(not(test))]
trait Trait<'b> {
    fn work<'a: 'b>(&self, v: &'a i32) -> &'a i32;
}
#[cfg(test)]
trait Trait<'b> {
    fn work<'a: 'b>(&self, v: &'a i32) -> &'a i32;
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
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct work_Call<'rs, 'b, 'a: 'b> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        v: &'a i32,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct work_ArgsChecker<'rs, 'b, 'a: 'b> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        v: Arg<'rs, &'a i32>,
    }
    impl<'rs, 'b, 'a: 'b> IArgsChecker for work_ArgsChecker<'rs, 'b, 'a> {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call<'rs, 'b, 'a> = dyn_call.downcast_ref();
            vec![self.v.check_ref("v", &call.v)]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<'rs, 'b> {
        work_data: FnData<'rs, TraitMock<'rs, 'b>, false>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct TraitMockSetup<'rs, 'b> {
        data: Arc<TraitMockData<'rs, 'b>>,
    }
    #[doc(hidden)]
    #[derive(Clone)]
    pub struct TraitMockReceived<'rs, 'b> {
        data: Arc<TraitMockData<'rs, 'b>>,
    }
    #[derive(Clone)]
    pub struct TraitMock<'rs, 'b> {
        pub setup: TraitMockSetup<'rs, 'b>,
        pub received: TraitMockReceived<'rs, 'b>,
        data: Arc<TraitMockData<'rs, 'b>>,
    }
    impl<'rs, 'b> Trait<'b> for TraitMock<'rs, 'b> {
        fn work<'a: 'b>(&self, v: &'a i32) -> &'a i32 {
            let call = unsafe {
                work_Call {
                    _phantom_lifetime: PhantomData,
                    v: core::mem::transmute(v),
                }
            };
            return self.data.work_data.handle_returning(call);
        }
    }
    impl<'rs, 'b> TraitMock<'rs, 'b> {
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
    impl<'rs, 'b> TraitMockSetup<'rs, 'b> {
        pub fn work<'a: 'b>(
            &self,
            v: impl Into<Arg<'rs, &'a i32>>,
        ) -> FnTuner<'rs, Self, (&&'a i32,), &'a i32, false> {
            let work_args_checker = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                v: v.into(),
            };
            let fn_tuner: FnTuner<'_, Self, (&&'a i32,), &'a i32, false> =
                self.data.work_data.add_config(work_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs, 'b> TraitMockReceived<'rs, 'b> {
        pub fn work<'a: 'b>(self, v: impl Into<Arg<'rs, &'a i32>>, times: Times) -> Self {
            let work_args_checker = work_ArgsChecker {
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
