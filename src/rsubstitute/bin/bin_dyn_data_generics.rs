use rsubstitute::macros::*;
use std::fmt::Debug;

// #[mock]
// trait Trait<T1> {
//     fn work<T2, T3, const B: bool, const N: usize>(&self, v: T1) -> T3;
// }

trait Trait<T1> {
    fn work<T2, T3: Clone, const B: bool, const N: usize>(&self, t1: T1, t2: T2) -> T3;
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
    use rsubstitute_core::fn_parameters::DynCall;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::marker::PhantomData;

    #[derive(IGenericsHashKeyProvider)]
    // #[derive(IArgInfosProvider)]
    pub struct work_Call<T1, T2, T3: Clone, const B: bool, const N: usize> {
        t1: T1,
        t2: T2,
        _return_type: PhantomData<T3>,
    }
    impl<T1, T2, T3: Clone, const B: bool, const N: usize> IArgInfosProvider
        for work_Call<T1, T2, T3, B, N>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![
                ArgInfo::new("t1", &self.t1, (&ArgPrinter(&self.t1)).debug_string()),
                ArgInfo::new("t2", &self.t2, (&ArgPrinter(&self.t2)).debug_string()),
                ArgInfo::new(
                    "_return_type",
                    &self._return_type,
                    (&ArgPrinter(&self._return_type)).debug_string(),
                ),
            ]
        }
    }

    #[derive(Debug, IGenericsHashKeyProvider, IArgsFormatter)]
    pub struct work_ArgsChecker<T1, T2, T3: Clone, const B: bool, const N: usize> {
        t1: Arg<T1>,
        t2: Arg<T2>,
        _return_type: PhantomData<T3>,
    }
    impl<T1, T2, T3: Clone, const B: bool, const N: usize> IArgsChecker
        for work_ArgsChecker<T1, T2, T3, B, N>
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call<T1, T2, T3, B, N> = dyn_call.downcast_ref();
            vec![self.t1.check("t1", &call.t1), self.t2.check("t2", &call.t2)]
        }
    }

    #[derive(IMockData)]
    pub struct TraitMockData<'rs, T1> {
        _phantom_T1: PhantomData<T1>,
        work_data: FnData<'rs, TraitMock<'rs, T1>>,
    }

    #[derive(Clone)]
    pub struct TraitMockSetup<'rs, T1> {
        data: Arc<TraitMockData<'rs, T1>>,
    }

    #[derive(Clone)]
    pub struct TraitMockReceived<'rs, T1> {
        data: Arc<TraitMockData<'rs, T1>>,
    }
    #[derive(Clone)]
    pub struct TraitMock<'rs, T1> {
        pub setup: TraitMockSetup<'rs, T1>,
        pub received: TraitMockReceived<'rs, T1>,
        data: Arc<TraitMockData<'rs, T1>>,
    }
    impl<'rs, T1> Trait<T1> for TraitMock<'rs, T1> {
        fn work<T2, T3: Clone, const B: bool, const N: usize>(&self, t1: T1, t2: T2) -> T3 {
            let call: work_Call<T1, T2, T3, B, N> = unsafe {
                work_Call {
                    t1: std::mem::transmute(t1),
                    t2: std::mem::transmute(t2),
                    _return_type: PhantomData,
                }
            };
            // dbg!(call.get_arg_infos()); // TODO remove
            let return_value = self.data.work_data.handle_returning(call);
            return return_value.downcast_into();
        }
    }
    impl<'rs, T1> TraitMock<'rs, T1> {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_T1: PhantomData,
                work_data: FnData::new("work"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'rs, T1> TraitMockSetup<'rs, T1> {
        pub fn work<T2, T3: Clone, const B: bool, const N: usize>(
            &self,
            t1: impl Into<Arg<T1>>,
            t2: impl Into<Arg<T2>>,
        ) -> SharedFnConfig<'rs, Self, TraitMock<'rs, T1>> {
            let work_args_checker: work_ArgsChecker<T1, T2, T3, B, N> = work_ArgsChecker {
                t1: t1.into(),
                t2: t2.into(),
                _return_type: PhantomData,
            };
            let fn_config = self.data.work_data.add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return unsafe { std::mem::transmute(shared_fn_config) };
        }
    }
    impl<'rs, T1> TraitMockReceived<'rs, T1> {
        pub fn work<T2, T3: Clone, const B: bool, const N: usize>(
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
                .verify_received(work_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}

#[derive(Clone, Debug)]
struct Foo {
    amogus: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsubstitute_core::Times;
    use rsubstitute_core::args::Arg;

    #[test]
    fn my_test() {
        let mock = TraitMock::new();

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
            .returns(v4)
            .work::<Foo, _, false, 2>(23, Arg::Any)
            .returns(22);

        let av3 = mock.work::<_, i32, false, 2>(10, "amogus");
        let av2 = mock.work::<_, i32, true, 4>(10, "amogus");
        let av1 = mock.work::<_, i32, true, 2>(10, "amogus");
        let av4 = mock.work::<_, [i32; 5], false, 2>(10, "amogus");
        let av5 = mock.work::<_, i32, false, 2>(23, Foo { amogus: 53.2f32 });

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
            .work::<Foo, i32, false, 2>(23, Arg::Is(|_| false), Times::Once)
            .no_other_calls();
        // TODO - write const generic parameters in error logs
    }
}

fn main() {}
