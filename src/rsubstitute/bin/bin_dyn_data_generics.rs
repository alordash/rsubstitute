use rsubstitute::macros::*;
use std::fmt::Debug;

// #[mock]
// trait Trait {
//     fn work<T1: PartialOrd, T2: PartialOrd,, T3: PartialOrd, const B: bool, const N: usize>(&self, v: T1) -> T3;
// }

trait Trait<T1> {
    fn work<T2: PartialOrd, T3: PartialOrd, const B: bool, const N: usize>(
        &self,
        t1: T1,
        t2: T2,
    ) -> T3;
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
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::marker::PhantomData;

    #[derive(Clone, IGenericsHashKeyProvider, IArgInfosProvider)]
    // #[derive(IArgInfosProvider)]
    pub struct work_Call<
        T1: PartialOrd,
        T2: PartialOrd,
        T3: PartialOrd,
        const B: bool,
        const N: usize,
    > {
        t1: T1,
        t2: T2,
        _return_type: PhantomData<T3>,
    }

    #[derive(Debug, IGenericsHashKeyProvider, IArgsFormatter)]
    pub struct work_ArgsChecker<
        T1: PartialOrd,
        T2: PartialOrd,
        T3: PartialOrd,
        const B: bool,
        const N: usize,
    > {
        t1: Arg<T1>,
        t2: Arg<T2>,
        _return_type: PhantomData<T3>,
    }
    impl<
        'rs,
        T1: PartialOrd + 'rs,
        T2: PartialOrd + 'rs,
        T3: PartialOrd + 'rs,
        const B: bool,
        const N: usize,
    > IArgsChecker<'rs> for work_ArgsChecker<T1, T2, T3, B, N>
    {
        fn check(&self, dyn_call: &'rs Call<'rs>) -> Vec<ArgCheckResult> {
            let call: &work_Call<T1, T2, T3, B, N> = dyn_call.downcast_ref();
            vec![self.t1.check("t1", &call.t1), self.t2.check("t2", &call.t2)]
        }
    }

    #[derive(IMockData)]
    pub struct TraitMockData<T1> {
        work_data: FnData<'static, TraitMock<T1>>,
    }

    #[derive(Clone)]
    pub struct TraitMockSetup<T1> {
        data: Arc<TraitMockData<T1>>,
    }

    #[derive(Clone)]
    pub struct TraitMockReceived<T1> {
        data: Arc<TraitMockData<T1>>,
    }
    #[derive(Clone)]
    pub struct TraitMock<T1> {
        pub setup: TraitMockSetup<T1>,
        pub received: TraitMockReceived<T1>,
        data: Arc<TraitMockData<T1>>,
    }
    impl<T1: PartialOrd> Trait<T1> for TraitMock<T1> {
        fn work<T2: PartialOrd, T3: PartialOrd, const B: bool, const N: usize>(
            &self,
            t1: T1,
            t2: T2,
        ) -> T3 {
            let call: work_Call<T1, T2, T3, B, N> = unsafe {
                work_Call {
                    t1: std::mem::transmute(t1),
                    t2: std::mem::transmute(t2),
                    _return_type: PhantomData,
                }
            };
            return self.data.work_data.handle_returning(Call::new(call));
        }
    }
    impl<T1: PartialOrd> TraitMock<T1> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                work_data: FnData::new("work", &SERVICES),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<T1: PartialOrd> TraitMockSetup<T1> {
        pub fn work<
            'rs,
            T2: PartialOrd + 'rs,
            T3: PartialOrd + 'rs,
            const B: bool,
            const N: usize,
        >(
            &'rs self,
            t1: impl Into<Arg<T1>>,
            t2: impl Into<Arg<T2>>,
        ) -> SharedFnConfig<'rs, TraitMock<T1>, T3, Self> {
            let work_args_checker: work_ArgsChecker<T1, T2, T3, B, N> = work_ArgsChecker {
                t1: t1.into(),
                t2: t2.into(),
                _return_type: PhantomData,
            };
            let fn_config = self.data.work_data.as_local().add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<T1: PartialOrd> TraitMockReceived<T1> {
        pub fn work<T2: PartialOrd, T3: PartialOrd, const B: bool, const N: usize>(
            self,
            t1: impl Into<Arg<T1>>,
            t2: impl Into<Arg<T2>>,
            times: Times,
        ) -> Self {
            let work_args_checker: work_ArgsChecker<T1, T2, T3, B, N> = work_ArgsChecker {
                t1: t1.into(),
                t2: t2.into(),
                _return_type: PhantomData,
            };
            self.data
                .work_data
                .as_local()
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
        let mock = TraitMock::<i32>::new();

        let v1 = 11;
        let v2 = 22;
        let v3 = 33;
        let v4 = [10; 5];

        mock.setup
            .work::<_, _, true, 2>(10, "amogus")
            .returns(v1)
            .work::<_, _, true, 4>(10, "amogus")
            .returns(v2)
            .work::<_, _, false, 2>(10, "amogus")
            .returns(v3)
            .work::<_, _, false, 2>(10, "amogus")
            .returns(v4);

        let av3 = mock.work::<_, i32, false, 2>(10, "amogus");
        let av2 = mock.work::<_, i32, true, 4>(10, "amogus");
        let av1 = mock.work::<_, i32, true, 2>(10, "amogus");
        let av4 = mock.work::<_, [i32; 5], false, 2>(10, "amogus");

        assert_eq!(v1, av1);
        assert_eq!(v2, av2);
        assert_eq!(v3, av3);
        assert_eq!(v4, av4);

        mock.received
            .work::<_, i32, true, 2>(10, "amogus", Times::Once)
            .work::<_, i32, true, 4>(10, "amogus", Times::Once)
            .work::<_, i32, false, 2>(10, "amogus", Times::Once)
            .work::<_, [i32; 5], false, 2>(10, "amogus", Times::Once)
            // TODO - mock.received - value used after move
            .work::<_, i32, true, 2>(10, "quo vadis", Times::Never)
            .work::<_, i32, true, 4>(11, "amogus", Times::Never)
            .work::<_, i32, false, 2>(10, "quo vadis", Times::Never)
            .work::<_, i32, true, 2>(10, true, Times::Never)
            .no_other_calls();
        // TODO - write const generic parameters in error logs
    }
}

fn main() {}
