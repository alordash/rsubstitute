#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(noop_method_call)]
#![allow(suspicious_double_ref_op)]

use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;
use std::cell::{LazyCell, RefCell};
use std::fmt::Debug;
use std::sync::Arc;

pub trait IFoo: Debug {
    fn get_value(&self) -> i32;
}

#[derive(Debug)]
struct Foo(i32);

impl IFoo for Foo {
    fn get_value(&self) -> i32 {
        self.0
    }
}

const DEFAULT_MY_TRAIT_GET: i32 = 10;
trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(
        &self,
        string: &str,
        something: &&[u8],
        dyn_obj: &dyn IFoo,
        arc: Arc<dyn IFoo>,
    ) -> Vec<u8>;

    // TODO - support this (pass as base caller fn block)
    fn get(&self) -> i32 {
        let value = DEFAULT_MY_TRAIT_GET;
        self.work(value);
        return value;
    }
}

pub use generated::*;
use not_enough_asserts::panics::*;
use rsubstitute_proc_macro::mock;

mod generated {
    use super::*;
    use rsubstitute::*;
    use rsubstitute_proc_macro::IArgsFormatter;
    use rsubstitute_proc_macro::IMockData;
    use std::cell::LazyCell;
    use std::fmt::Debug;
    use std::marker::PhantomData;
    use std::sync::Arc;

    // start - Calls
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct work_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub value: i32,
    }

    impl<'a> IArgInfosProvider for work_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![ArgInfo::new("value", &self.value)];
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct work_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub value: Arg<i32>,
    }

    impl<'a> IArgsChecker<work_Call<'a>> for work_ArgsChecker<'a> {
        fn check(&'_ self, call: &work_Call) -> Vec<ArgCheckResult> {
            vec![self.value.check("value", &call.value)]
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct another_work_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub string: &'a str,
        pub something: &'a &'a [u8],
        pub dyn_obj: &'a dyn IFoo,
        pub arc: Arc<dyn IFoo>,
    }

    impl<'a> IArgInfosProvider for another_work_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![
                ArgInfo::new("string", &self.string),
                ArgInfo::new("something", &self.something),
                ArgInfo::new("dyn_obj", &self.dyn_obj),
                ArgInfo::new("arc", &self.arc),
            ];
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct another_work_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub string: Arg<&'a str>,
        pub something: Arg<&'a &'a [u8]>,
        pub dyn_obj: Arg<&'a dyn IFoo>,
        pub arc: Arg<Arc<dyn IFoo>>,
    }

    impl<'a> IArgsChecker<another_work_Call<'a>> for another_work_ArgsChecker<'a> {
        fn check(&self, call: &another_work_Call<'a>) -> Vec<ArgCheckResult> {
            vec![
                self.string.check("string", &call.string),
                self.something.check_ref("something", &call.something),
                self.dyn_obj.check_ref("dyn_obj", &call.dyn_obj),
                self.arc.check_arc("arc", &call.arc),
            ]
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct get_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
    }

    impl<'a> IArgInfosProvider for get_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![];
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct get_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
    }

    impl<'a> IArgsChecker<get_Call<'a>> for get_ArgsChecker<'a> {
        fn check(&'_ self, _call: &get_Call) -> Vec<ArgCheckResult> {
            Vec::new()
        }
    }

    // end - Calls
    // start - Mock
    #[derive(IMockData)]
    struct MyTraitMockData<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        work_data: FnData<MyTraitMock<'a>, work_Call<'a>, work_ArgsChecker<'a>, ()>,
        another_work_data:
            FnData<MyTraitMock<'a>, another_work_Call<'a>, another_work_ArgsChecker<'a>, Vec<u8>>,
        get_data: FnData<MyTraitMock<'a>, get_Call<'a>, get_ArgsChecker<'a>, i32>,
    }

    pub struct MyTraitMockSetup<'a> {
        data: Arc<MyTraitMockData<'a>>,
    }

    pub struct MyTraitMockReceived<'a> {
        data: Arc<MyTraitMockData<'a>>,
    }

    pub struct MyTraitMock<'a> {
        pub setup: MyTraitMockSetup<'a>,
        pub received: MyTraitMockReceived<'a>,
        data: Arc<MyTraitMockData<'a>>,
    }

    impl<'a> IBaseCaller<get_Call<'a>, i32> for MyTraitMock<'a> {
        fn call_base(&self, call: get_Call<'a>) -> i32 {
            let get_Call { .. } = call;
            let value = DEFAULT_MY_TRAIT_GET;
            self.work(value);
            return value;
        }
    }

    impl<'a> MyTrait for MyTraitMock<'a> {
        fn work(&self, value: i32) {
            let call = work_Call {
                phantom_lifetime: PhantomData,
                value,
            };
            return self.data.work_data.handle(call);
        }

        fn another_work(
            &self,
            string: &str,
            something: &&[u8],
            dyn_obj: &dyn IFoo,
            arc: Arc<dyn IFoo>,
        ) -> Vec<u8> {
            let call = unsafe {
                another_work_Call {
                    phantom_lifetime: PhantomData,
                    string: std::mem::transmute(string),
                    something: std::mem::transmute(something),
                    dyn_obj: std::mem::transmute(dyn_obj),
                    arc,
                }
            };
            return self.data.another_work_data.handle_returning(call);
        }

        fn get(&self) -> i32 {
            let call = get_Call {
                phantom_lifetime: PhantomData,
            };
            return self.data.get_data.handle_base_returning(&self, call);
        }
    }

    impl<'a> MyTraitMock<'a> {
        pub fn new() -> Self {
            let data = Arc::new(MyTraitMockData {
                _phantom_lifetime: PhantomData,
                work_data: FnData::new("work", &SERVICES),
                another_work_data: FnData::new("another_work", &SERVICES),
                get_data: FnData::new("get", &SERVICES),
            });
            let mock = Self {
                setup: MyTraitMockSetup { data: data.clone() },
                received: MyTraitMockReceived { data: data.clone() },
                data,
            };
            return mock;
        }
    }

    impl<'a> MyTraitMockSetup<'a> {
        pub fn work(
            &'a self,
            value: impl Into<Arg<i32>>,
        ) -> SharedFnConfig<'a, MyTraitMock<'a>, work_Call<'a>, work_ArgsChecker<'a>, (), Self>
        {
            let work_args_checker = work_ArgsChecker {
                phantom_lifetime: PhantomData,
                value: value.into(),
            };
            let fn_config = self.data.work_data.add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn another_work(
            &'a self,
            string: impl Into<Arg<&'a str>>,
            something: impl Into<Arg<&'a &'a [u8]>>,
            dyn_obj: impl Into<Arg<&'a dyn IFoo>>,
            arc: impl Into<Arg<Arc<dyn IFoo>>>,
        ) -> SharedFnConfig<
            'a,
            MyTraitMock<'a>,
            another_work_Call<'a>,
            another_work_ArgsChecker<'a>,
            Vec<u8>,
            Self,
        > {
            let another_work_args_checker = another_work_ArgsChecker {
                phantom_lifetime: PhantomData,
                string: string.into(),
                something: something.into(),
                dyn_obj: dyn_obj.into(),
                arc: arc.into(),
            };
            let fn_config = self
                .data
                .another_work_data
                .add_config(another_work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn get(
            &'a self,
        ) -> SharedFnConfig<'a, MyTraitMock<'a>, get_Call<'a>, get_ArgsChecker<'a>, i32, Self>
        {
            let get_args_checker = get_ArgsChecker {
                phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.get_data.add_config(get_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }

    impl<'a> MyTraitMockReceived<'a> {
        pub fn work(&'a self, value: impl Into<Arg<i32>>, times: Times) -> &'a Self {
            let work_args_checker = work_ArgsChecker {
                phantom_lifetime: PhantomData,
                value: value.into(),
            };
            self.data
                .work_data
                .verify_received(work_args_checker, times);
            return self;
        }

        pub fn another_work(
            &'a self,
            string: impl Into<Arg<&'a str>>,
            something: impl Into<Arg<&'a &'a [u8]>>,
            dyn_obj: impl Into<Arg<&'a dyn IFoo>>,
            arc: impl Into<Arg<Arc<dyn IFoo>>>,
            times: Times,
        ) -> &'a Self {
            let another_work_args_checker = another_work_ArgsChecker {
                phantom_lifetime: PhantomData,
                string: string.into(),
                something: something.into(),
                dyn_obj: dyn_obj.into(),
                arc: arc.into(),
            };
            self.data
                .another_work_data
                .verify_received(another_work_args_checker, times);
            return self;
        }

        pub fn get(&'a self, times: Times) -> &'a Self {
            let get_args_checker = get_ArgsChecker {
                phantom_lifetime: PhantomData,
            };
            self.data.get_data.verify_received(get_args_checker, times);
            return self;
        }

        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }

    // end - Mock
}

#[test]
fn received_nothing_else_Ok() {
    // Arrange
    let mock = MyTraitMock::new();
    let returned_value = 11;
    mock.setup.get().returns(returned_value);

    // Act
    let actual_returned_value = mock.get();

    // Assert
    assert_eq!(returned_value, actual_returned_value);

    mock.received.get(Times::Once).no_other_calls();
}

#[test]
fn received_nothing_else_Panics() {
    // Arrange
    let mock = MyTraitMock::new();
    let returned_value = 11;
    let work_arguments = 45;
    mock.setup.get().returns(returned_value);
    mock.setup.work(Arg::Any);

    // Act
    let actual_returned_value = mock.get();
    mock.work(work_arguments);

    // Assert
    assert_eq!(returned_value, actual_returned_value);

    assert_panics(
        || mock.received.get(Times::Once).no_other_calls(),
        format!(
            "Did not expect to receive any other calls. Received 1 unexpected call:
1. work({work_arguments})"
        ),
    );
}

#[test]
fn get_CallBase_Ok() {
    // Arrange
    let mock = MyTraitMock::new();
    mock.setup.get().call_base();

    // Act
    let actual_value = mock.get();

    // Assert
    assert_eq!(DEFAULT_MY_TRAIT_GET, actual_value);
    mock.received
        .get(Times::Once)
        .work(actual_value, Times::Once)
        .no_other_calls();
}

fn main() {
    let string = &String::from("amogus");
    let bytes = vec![1u8, 2, 3, 44];
    let something = &&bytes[..];
    let my_trait_mock = MyTraitMock::new();

    let foo1: &dyn IFoo = &Foo(52);
    let foo2: &dyn IFoo = &Foo(8998);
    let arc_foo1: Arc<dyn IFoo> = Arc::new(Foo(10));
    let arc_foo2: Arc<dyn IFoo> = Arc::new(Foo(144));
    my_trait_mock
        .setup
        .work(Arg::Is(|value| *value == 32))
        .does(|| println!("work mock called"))
        .another_work(
            Arg::Eq(string as &str),
            Arg::Eq(something),
            Arg::Any,
            Arg::Eq(arc_foo1.clone()),
        )
        .returns(vec![4, 5, 6])
        .another_work(
            Arg::Any,
            Arg::Any,
            Arg::Any,
            Arg::Is(|foo: &Arc<dyn IFoo>| foo.get_value() == 144),
        )
        .returns(vec![7, 70, 77])
        .get()
        .returns_and_does(112, || println!("first get call!"))
        .get()
        .returns(900000);
    my_trait_mock.work(32);

    let first_another_work = my_trait_mock.another_work(string, something, foo1, arc_foo1);
    println!("first_another_work = {first_another_work:?}");
    let second_another_work = my_trait_mock.another_work("que", something, foo2, arc_foo2.clone());
    my_trait_mock.another_work("que", something, foo2, arc_foo2.clone());
    my_trait_mock.another_work("que", something, foo2, arc_foo2.clone());
    println!("second_another_work = {second_another_work:?}");

    let first_get = MyTrait::get(&my_trait_mock);
    println!("first_get = {first_get}");
    let second_get = MyTrait::get(&my_trait_mock);
    println!("second_get = {second_get}");

    my_trait_mock.work(11111);
    // let panics = MyTrait::get(&my_trait_mock);

    my_trait_mock.received.work(Arg::Eq(11111), Times::Once);
    my_trait_mock.received.work(Arg::Any, Times::Exactly(2));
    my_trait_mock.received.another_work(
        "que",
        Arg::Any,
        Arg::Is(|_| false),
        Arc::new(Foo(44)) as Arc<dyn IFoo>,
        Times::Exactly(22),
    );
    my_trait_mock.received.get(Times::Exactly(2));

    println!("Done");
}
