#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;
use std::cell::{LazyCell, RefCell};
use std::fmt::Debug;
use std::sync::Arc;

trait MyTrait<T> {
    fn work(&self, value: T);

    fn another_work(&self, string: &str) -> T;

    fn get(&self) -> T;

    // fn standalone(number: i32) -> f32;
    //
    // fn standalone_with_ref(number: &i32) -> f32;
}

pub use generated::*;

mod generated {
    use super::*;
    use rsubstitute::*;
    use rsubstitute_proc_macro::IArgsFormatter;
    use std::cell::LazyCell;
    use std::fmt::Debug;
    use std::marker::PhantomData;
    use std::sync::Arc;

    // start - Calls
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct work_Call<'a, T: Debug + PartialOrd + Clone> {
        phantom_lifetime: PhantomData<&'a ()>,
        phantom_T: PhantomData<T>,
        pub value: T,
    }

    impl<'a, T: Debug + PartialOrd + Clone> IArgInfosProvider for work_Call<'a, T> {
        fn get_fn_name(&self) -> &'static str {
            return "work";
        }

        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![ArgInfo::new("value", self.value.clone())];
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct work_ArgsChecker<'a, T: Debug + PartialOrd + Clone> {
        phantom_lifetime: PhantomData<&'a ()>,
        phantom_T: PhantomData<T>,
        pub value: Arg<'a, T>,
    }

    impl<'a, T: Debug + PartialOrd + Clone> IArgsChecker<work_Call<'a, T>> for work_ArgsChecker<'a, T> {
        fn check(&'_ self, call: work_Call<'a, T>) -> Vec<ArgCheckResult<'_>> {
            vec![self.value.check("value", call.value)]
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct another_work_Call<'a, T: Debug + PartialOrd + Clone> {
        phantom_lifetime: PhantomData<&'a ()>,
        phantom_T: PhantomData<T>,
        pub string: &'a str,
    }

    impl<'a, T: Debug + PartialOrd + Clone> IArgInfosProvider for another_work_Call<'a, T> {
        fn get_fn_name(&self) -> &'static str {
            return "another_work";
        }

        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![ArgInfo::new("string", self.string.clone())];
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct another_work_ArgsChecker<'a, T: Debug + PartialOrd + Clone> {
        phantom_lifetime: PhantomData<&'a ()>,
        phantom_T: PhantomData<T>,
        pub string: Arg<'a, &'a str>,
    }

    impl<'a, T: Debug + PartialOrd + Clone> IArgsChecker<another_work_Call<'a, T>>
        for another_work_ArgsChecker<'a, T>
    {
        fn check(&self, call: another_work_Call<'a, T>) -> Vec<ArgCheckResult<'a>> {
            vec![self.string.check("string", call.string)]
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct get_Call<'a, T: Debug + PartialOrd + Clone> {
        phantom_lifetime: PhantomData<&'a ()>,
        phantom_T: PhantomData<T>,
    }

    impl<'a, T: Debug + PartialOrd + Clone> IArgInfosProvider for get_Call<'a, T> {
        fn get_fn_name(&self) -> &'static str {
            return "get";
        }

        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![];
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct get_ArgsChecker<'a, T: Debug + PartialOrd + Clone> {
        phantom_lifetime: PhantomData<&'a ()>,
        phantom_T: PhantomData<T>,
    }

    impl<'a, T: Debug + PartialOrd + Clone> IArgsChecker<get_Call<'a, T>> for get_ArgsChecker<'a, T> {
        fn check(&'_ self, _call: get_Call<'a, T>) -> Vec<ArgCheckResult<'_>> {
            Vec::new()
        }
    }

    // end - Calls
    // start - Mock
    struct MyTraitMockData<'a, T: Debug + PartialOrd + Clone> {
        work_data: FnData<work_Call<'a, T>, work_ArgsChecker<'a, T>, (), ()>,
        another_work_data: FnData<another_work_Call<'a, T>, another_work_ArgsChecker<'a, T>, T, ()>,
        get_data: FnData<get_Call<'a, T>, get_ArgsChecker<'a, T>, T, ()>,
    }

    pub struct MyTraitMockSetup<'a, T: Debug + PartialOrd + Clone> {
        data: Arc<MyTraitMockData<'a, T>>,
    }

    pub struct MyTraitMockReceived<'a, T: Debug + PartialOrd + Clone> {
        data: Arc<MyTraitMockData<'a, T>>,
    }

    pub struct MyTraitMock<'a, T: Debug + PartialOrd + Clone> {
        pub setup: MyTraitMockSetup<'a, T>,
        pub received: MyTraitMockReceived<'a, T>,
        data: Arc<MyTraitMockData<'a, T>>,
    }

    impl<'a, T: Debug + PartialOrd + Clone> MyTrait<T> for MyTraitMock<'a, T> {
        fn work(&self, value: T) {
            let call = work_Call {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
                value,
            };
            return self.data.work_data.handle(call);
        }

        fn another_work(&self, string: &str) -> T {
            let call = unsafe {
                another_work_Call {
                    phantom_lifetime: PhantomData,
                    phantom_T: PhantomData,
                    string: std::mem::transmute(string),
                }
            };
            return self.data.another_work_data.handle_returning(call);
        }

        fn get(&self) -> T {
            let call = get_Call {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
            };
            return self.data.get_data.handle_returning(call);
        }

        // fn standalone(number: i32) -> f32 {
        //     let call = standalone_Call {
        //         phantom_lifetime: PhantomData,
        //         number,
        //     };
        //     return Self::standalone_data.handle_returning(call);
        // }
        //
        // fn standalone_with_ref(_number: &i32) -> f32 {
        //     todo!()
        // }
    }

    impl<'a, T: Debug + PartialOrd + Clone> MyTraitMock<'a, T> {
        pub fn new() -> Self {
            let data = Arc::new(MyTraitMockData {
                work_data: FnData::new("work", &SERVICES),
                another_work_data: FnData::new("another_work", &SERVICES),
                get_data: FnData::new("get", &SERVICES),
            });
            return Self {
                setup: MyTraitMockSetup { data: data.clone() },
                received: MyTraitMockReceived { data: data.clone() },
                data,
            };
        }
    }

    impl<'a, T: Debug + PartialOrd + Clone> MyTraitMockSetup<'a, T> {
        pub fn work(
            &'a self,
            value: impl Into<Arg<'a, T>>,
        ) -> SharedFnConfig<'a, work_Call<'a, T>, work_ArgsChecker<'a, T>, (), Self, ()> {
            let work_args_checker = work_ArgsChecker {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
                value: value.into(),
            };
            let fn_config = self.data.work_data.add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self, None);
            return shared_fn_config;
        }

        pub fn another_work(
            &'a self,
            string: impl Into<Arg<'a, &'a str>>,
        ) -> SharedFnConfig<
            'a,
            another_work_Call<'a, T>,
            another_work_ArgsChecker<'a, T>,
            T,
            Self,
            (),
        > {
            let another_work_args_checker = another_work_ArgsChecker {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
                string: string.into(),
            };
            let fn_config = self
                .data
                .another_work_data
                .add_config(another_work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self, None);
            return shared_fn_config;
        }

        pub fn get(
            &'a self,
        ) -> SharedFnConfig<'a, get_Call<'a, T>, get_ArgsChecker<'a, T>, T, Self, ()> {
            let get_args_checker = get_ArgsChecker {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
            };
            let fn_config = self.data.get_data.add_config(get_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self, None);
            return shared_fn_config;
        }
    }

    impl<'a, T: Debug + PartialOrd + Clone> MyTraitMockReceived<'a, T> {
        pub fn work(&'a self, value: impl Into<Arg<'a, T>>, times: Times) -> &'a Self {
            let work_args_checker = work_ArgsChecker {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
                value: value.into(),
            };
            self.data
                .work_data
                .verify_received(work_args_checker, times);
            return self;
        }

        pub fn another_work(
            &'a self,
            string: impl Into<Arg<'a, &'a str>>,
            times: Times,
        ) -> &'a Self {
            let another_work_args_checker = another_work_ArgsChecker {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
                string: string.into(),
            };
            self.data
                .another_work_data
                .verify_received(another_work_args_checker, times);
            return self;
        }

        pub fn get(&'a self, times: Times) -> &'a Self {
            let get_args_checker = get_ArgsChecker {
                phantom_lifetime: PhantomData,
                phantom_T: PhantomData,
            };
            self.data.get_data.verify_received(get_args_checker, times);
            return self;
        }

        // #[allow(non_upper_case_globals)]
        // const standalone_data: LazyCell<
        //     FnData<standalone_Call<'a>, standalone_ArgsChecker<'a>, f32>,
        // > = LazyCell::new(|| FnData::new("standalone", &SERVICES));
        // pub fn standalone(number: Arg<i32>) -> f32 {
        //     let standalone_args_checker = standalone_ArgsChecker {
        //         phantom_lifetime: PhantomData,
        //         number,
        //     };
        //     // let _fn_config = Self::standalone_data.add_config(standalone_args_checker);
        //     // let shared_fn_config = SharedFnConfig::new()
        //     todo!()
        // }
    }

    // end - Mock
}

fn main() {
    let string = &String::from("amogus");
    let bytes = vec![1u8, 2, 3, 44];
    let my_trait_mock = MyTraitMock::new();

    my_trait_mock
        .setup
        .work(Arg::Is(|value| true))
        .does(|| println!("work mock called"))
        .another_work(Arg::Eq(string as &str))
        .returns(vec![4, 5, 6])
        .another_work(Arg::Any)
        .returns(vec![7, 70, 77])
        .get()
        .returns_and_does(vec![333], || println!("first get call!"))
        .get()
        .returns(vec![43]);
    my_trait_mock.work(vec![111]);

    let first_another_work = my_trait_mock.another_work(string);
    println!("first_another_work = {first_another_work:?}");
    let second_another_work = my_trait_mock.another_work("que");
    my_trait_mock.another_work("que");
    my_trait_mock.another_work("que");
    println!("second_another_work = {second_another_work:?}");

    let first_get = MyTrait::get(&my_trait_mock);
    println!("first_get = {first_get:?}");
    let second_get = MyTrait::get(&my_trait_mock);
    println!("second_get = {second_get:?}");

    my_trait_mock.work(vec![64]);
    // let panics = MyTrait::get(&my_trait_mock);

    my_trait_mock.received.work(Arg::Eq(vec![66]), Times::Once);
    my_trait_mock.received.work(Arg::Any, Times::Exactly(2));
    my_trait_mock
        .received
        .another_work("que", Times::Exactly(22));
    my_trait_mock.received.get(Times::Exactly(2));

    println!("Done");
}
