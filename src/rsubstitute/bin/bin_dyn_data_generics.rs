use rsubstitute::macros::*;
use std::fmt::Debug;

// #[mock]
trait Trait<'rs, T1> {
    fn work<T2, T3, const B: bool, const N: usize>(&self, t1: T1, t2: &'rs T2) -> T3;
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

    #[derive(IGenericsHashKeyProvider, IArgsInfosProvider, IArgsTupleProvider)]
    pub struct work_Call<'rs, T1, T2, T3, const B: bool, const N: usize> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        _return_type: PhantomData<T3>,
        t1: T1,
        t2: &'rs T2,
    }

    #[derive(Debug, IGenericsHashKeyProvider, IArgsFormatter)]
    pub struct work_ArgsChecker<'rs, T1, T2, T3, const B: bool, const N: usize> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        _return_type: PhantomData<T3>,
        t1: Arg<T1>,
        t2: Arg<&'rs T2>,
    }
    impl<'rs, T1, T2, T3, const B: bool, const N: usize> IArgsChecker
        for work_ArgsChecker<'rs, T1, T2, T3, B, N>
    {
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &work_Call<T1, T2, T3, B, N> = dyn_call.downcast_ref();
            vec![self.t1.check("t1", &call.t1), self.t2.check("t2", &call.t2)]
        }
    }

    #[derive(IMockData)]
    pub struct TraitMockData<'rs, T1> {
        _phantom_T1: PhantomData<T1>,
        work_data: FnData<'rs, TraitMock<'rs, T1>, false>,
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
    impl<'rs, T1> Trait<'rs, T1> for TraitMock<'rs, T1> {
        fn work<T2, T3, const B: bool, const N: usize>(&self, t1: T1, t2: &'rs T2) -> T3 {
            let call: work_Call<T1, T2, T3, B, N> = unsafe {
                work_Call {
                    _phantom_lifetime: PhantomData,
                    _return_type: PhantomData,
                    t1: core::mem::transmute(t1),
                    t2: core::mem::transmute(t2),
                }
            };
            // dbg!(call.get_arg_infos()); // TODO remove
            return self.data.work_data.handle_returning(call);
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
    impl<'rs, T1: 'rs> TraitMockSetup<'rs, T1> {
        pub fn work<T2: 'rs, T3: 'rs, const B: bool, const N: usize>(
            &self,
            t1: impl Into<Arg<T1>>,
            t2: impl Into<Arg<&'rs T2>>,
        ) -> FnTuner<'rs, Self, (&T1, &&T2), T3, false> {
            let work_args_checker: work_ArgsChecker<T1, T2, T3, B, N> = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _return_type: PhantomData,
                t1: t1.into(),
                t2: t2.into(),
            };
            let fn_tuner: FnTuner<'_, _, (&T1, &&T2), T3, _> =
                self.data.work_data.add_config(work_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs, T1> TraitMockReceived<'rs, T1> {
        pub fn work<T2: 'rs, T3: Clone, const B: bool, const N: usize>(
            self,
            t1: impl Into<Arg<T1>>,
            t2: impl Into<Arg<&'rs T2>>,
            times: Times,
        ) -> Self {
            let work_args_checker: work_ArgsChecker<T1, T2, T3, B, N> = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                _return_type: PhantomData,
                t1: t1.into(),
                t2: t2.into(),
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
            .work::<_, _, true, 2>(10, &"amogus")
            .returns(v1)
            .and_does(|(number, string)| println!("Received number = {number}, string = {string}"))
            .work::<_, _, true, 4>(10, &"amogus")
            .returns(v2)
            .and_does(|_| println!("I don't care what was received"))
            .work::<_, _, false, 2>(10, &"amogus")
            .returns(v3)
            .work::<_, _, false, 2>(10, &"amogus")
            .returns(v4)
            .work::<Foo, _, false, 2>(23, Arg::Any)
            .returns(22);

        let av3 = mock.work::<_, i32, false, 2>(10, &"amogus");
        let av2 = mock.work::<_, i32, true, 4>(10, &"amogus");
        let av1 = mock.work::<_, i32, true, 2>(10, &"amogus");
        let av4 = mock.work::<_, [i32; 5], false, 2>(10, &"amogus");
        let av5 = mock.work::<_, i32, false, 2>(23, &Foo { amogus: 53.2f32 });

        // {
        //     let q = 12;
        //     let r = &q;
        //     mock.work::<_, i32, true, 2>(10, r);
        // }

        assert_eq!(v1, av1);
        assert_eq!(v2, av2);
        assert_eq!(v3, av3);
        assert_eq!(v4, av4);

        mock.received
            .work::<_, i32, true, 2>(10, &"amogus", Times::Once)
            .work::<_, i32, true, 4>(10, &"amogus", Times::Once)
            .work::<_, i32, false, 2>(10, &"amogus", Times::Once)
            .work::<_, [i32; 5], false, 2>(10, &"amogus", Times::Once)
            // TODO - mock.received - value used after move
            .work::<_, i32, true, 2>(10, &"quo vadis", Times::Never)
            .work::<_, i32, true, 4>(11, &"amogus", Times::Never)
            .work::<_, i32, false, 2>(10, &"quo vadis", Times::Never)
            .work::<_, i32, true, 2>(10, &true, Times::Never)
            .work::<Foo, i32, false, 2>(
                23,
                Arg::Is(|foo: &&Foo| foo.amogus == 53.2f32),
                Times::Once,
            )
            .no_other_calls();
        // TODO - write const generic parameters in error logs
    }
}

fn main() {}
