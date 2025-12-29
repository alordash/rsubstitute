#![allow(unused)]
use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;
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

trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(
        &self,
        string: &str,
        something: &&[u8],
        dyn_obj: &dyn IFoo,
        arc: Arc<dyn IFoo>,
    ) -> Vec<u8>;

    fn get(&self) -> i32;

    // fn standalone(number: i32) -> f32;
    //
    // fn standalone_with_ref(number: &i32) -> f32;
}

pub use generated::*;
use rsubstitute::qweee;

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
    pub struct work_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub value: i32,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct work_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub value: Arg<'a, i32>,
    }

    impl<'a> IArgsChecker<work_Call<'a>> for work_ArgsChecker<'a> {
        fn check(&'_ self, call: work_Call) -> Vec<ArgCheckResult<'_>> {
            vec![self.value.check("value", call.value)]
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

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct another_work_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub string: Arg<'a, &'a str>,
        pub something: Arg<'a, &'a &'a [u8]>,
        pub dyn_obj: Arg<'a, &'a dyn IFoo>,
        pub arc: Arg<'a, Arc<dyn IFoo>>,
    }

    impl<'a> IArgsChecker<another_work_Call<'a>> for another_work_ArgsChecker<'a> {
        fn check(&self, call: another_work_Call<'a>) -> Vec<ArgCheckResult<'a>> {
            vec![
                self.string.check("string", call.string),
                self.something.check_ref("something", call.something),
                self.dyn_obj.check_ref("dyn_obj", call.dyn_obj),
                self.arc.check_arc("arc", call.arc),
            ]
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct get_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct get_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
    }

    impl<'a> IArgsChecker<get_Call<'a>> for get_ArgsChecker<'a> {
        fn check(&'_ self, _call: get_Call) -> Vec<ArgCheckResult<'_>> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct standalone_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        number: i32,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct standalone_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        number: Arg<'a, i32>,
    }

    impl<'a> IArgsChecker<standalone_Call<'a>> for standalone_ArgsChecker<'a> {
        fn check(&'_ self, call: standalone_Call) -> Vec<ArgCheckResult<'_>> {
            vec![self.number.check("number", call.number)]
        }
    }

    // end - Calls
    // start - Mock
    struct MyTraitMockData<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        work_data: FnData<work_Call<'a>, work_ArgsChecker<'a>, ()>,
        another_work_data: FnData<another_work_Call<'a>, another_work_ArgsChecker<'a>, Vec<u8>>,
        get_data: FnData<get_Call<'a>, get_ArgsChecker<'a>, i32>,
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
            // TODO - this is solution for lifetimes
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

    impl<'a> MyTraitMock<'a> {
        pub fn new() -> Self {
            let data = Arc::new(MyTraitMockData {
                _phantom_lifetime: PhantomData,
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

    impl<'a> MyTraitMockSetup<'a> {
        pub fn work(
            &'a self,
            value: Arg<'a, i32>,
        ) -> SharedFnConfig<'a, work_Call<'a>, work_ArgsChecker<'a>, (), Self> {
            let work_args_checker = work_ArgsChecker {
                phantom_lifetime: PhantomData,
                value,
            };
            let fn_config = self.data.work_data.add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn another_work(
            &'a self,
            string: Arg<'a, &'a str>,
            something: Arg<'a, &'a &'a [u8]>,
            dyn_obj: Arg<'a, &'a dyn IFoo>,
            arc: Arg<'a, Arc<dyn IFoo>>,
        ) -> SharedFnConfig<'a, another_work_Call<'a>, another_work_ArgsChecker<'a>, Vec<u8>, Self>
        {
            let another_work_args_checker = another_work_ArgsChecker {
                phantom_lifetime: PhantomData,
                string,
                something,
                dyn_obj,
                arc,
            };
            let fn_config = self
                .data
                .another_work_data
                .add_config(another_work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn get(&'a self) -> SharedFnConfig<'a, get_Call<'a>, get_ArgsChecker<'a>, i32, Self> {
            let get_args_checker = get_ArgsChecker {
                phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.get_data.add_config(get_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }

    impl<'a> MyTraitMockReceived<'a> {
        pub fn work(&'a self, value: Arg<'a, i32>, times: Times) -> &'a Self {
            let work_args_checker = work_ArgsChecker {
                phantom_lifetime: PhantomData,
                value,
            };
            self.data
                .work_data
                .verify_received(work_args_checker, times);
            return self;
        }

        pub fn another_work(
            &'a self,
            string: Arg<'a, &'a str>,
            something: Arg<'a, &'a &'a [u8]>,
            dyn_obj: Arg<'a, &'a dyn IFoo>,
            arc: Arg<'a, Arc<dyn IFoo>>,
            times: Times,
        ) -> &'a Self {
            let another_work_args_checker = another_work_ArgsChecker {
                phantom_lifetime: PhantomData,
                string,
                something,
                dyn_obj,
                arc,
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

        #[allow(non_upper_case_globals)]
        const standalone_data: LazyCell<
            FnData<standalone_Call<'a>, standalone_ArgsChecker<'a>, f32>,
        > = LazyCell::new(|| FnData::new("standalone", &SERVICES));
        pub fn standalone(number: Arg<i32>) -> f32 {
            let standalone_args_checker = standalone_ArgsChecker {
                phantom_lifetime: PhantomData,
                number,
            };
            // let _fn_config = Self::standalone_data.add_config(standalone_args_checker);
            // let shared_fn_config = SharedFnConfig::new()
            todo!()
        }
    }

    // end - Mock
}

#[cfg(not(test))]
fn global(number: i32) -> String {
    return format!("actual number: {number}");
}

#[cfg(test)]
use globals::global;
#[cfg(test)]
mod globals {
    use super::*;
    use rsubstitute::IArgsFormatter;
    use rsubstitute_core::args_matching::{ArgCheckResult, IArgsChecker};
    use rsubstitute_core::{FnData, SERVICES, SharedFnConfig};
    use rsubstitute_proc_macro::IArgsFormatter;
    use std::cell::LazyCell;
    use std::marker::PhantomData;
    use std::sync::Arc;
    use std::sync::LazyLock;

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct global_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub number: i32,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug, IArgsFormatter)]
    pub struct global_ArgsChecker<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub number: Arg<'a, i32>,
    }

    impl<'a> IArgsChecker<global_Call<'a>> for global_ArgsChecker<'a> {
        fn check(&self, call: global_Call<'a>) -> Vec<ArgCheckResult<'_>> {
            vec![self.number.check("number", call.number)]
        }
    }

    #[allow(non_camel_case_types)]
    pub struct global_Data<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        data: FnData<global_Call<'a>, global_ArgsChecker<'a>, String>,
    }

    unsafe impl<'a> Send for global_Data<'a> {}
    unsafe impl<'a> Sync for global_Data<'a> {}

    #[allow(non_camel_case_types)]
    pub struct global_Setup<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        global_data: Arc<global_Data<'a>>,
    }

    unsafe impl<'a> Send for global_Setup<'a> {}
    unsafe impl<'a> Sync for global_Setup<'a> {}

    impl<'a> global_Setup<'a> {
        pub fn setup(
            &'a self,
            number: Arg<'a, i32>,
        ) -> SharedFnConfig<'a, global_Call<'a>, global_ArgsChecker<'a>, String, Self> {
            let global_args_checker = global_ArgsChecker {
                phantom_lifetime: PhantomData,
                number,
            };
            let fn_config = self.global_data.data.add_config(global_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }

    #[allow(non_upper_case_globals)]
    static global_DATA: LazyLock<Arc<global_Data>> = LazyLock::new(|| {
        println!("new global_DATA");
        Arc::new(global_Data {
            _phantom_lifetime: PhantomData,
            data: FnData::new("global", &SERVICES),
        })
    });

    #[allow(non_upper_case_globals)]
    static global_SETUP: LazyLock<&'static global_Setup> = LazyLock::new(|| {
        println!("new global_SETUP");
        Box::leak(Box::new(global_Setup {
            _phantom_lifetime: PhantomData,
            global_data: (*global_DATA).clone(),
        }))
    });

    pub fn setup(
        number: Arg<'static, i32>,
    ) -> SharedFnConfig<
        'static,
        global_Call<'static>,
        global_ArgsChecker<'static>,
        String,
        global_Setup<'static>,
    > {
        let global_args_checker = global_ArgsChecker {
            phantom_lifetime: PhantomData,
            number,
        };
        let fn_config = global_DATA.data.add_config(global_args_checker);
        let shared_fn_config = SharedFnConfig::new(fn_config, *global_SETUP);
        return shared_fn_config;
    }

    pub fn global(number: i32) -> String {
        let call = global_Call {
            phantom_lifetime: PhantomData,
            number,
        };
        return global_DATA.data.handle_returning(call);
    }
}

#[cfg(test)]
mod tests {
    use crate::globals;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    pub fn global_test() {
        globals::setup(Arg::Eq(2)).returns("MOCK: 2".to_string());
        let result = globals::global(2);
        assert_eq!("MOCK: 2", result);
    }
}

fn main() {
    qweee();

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
        .work(Arg::Is(|value| value == 32))
        .does(|| println!("work mock called"))
        .another_work(
            Arg::Eq(string),
            Arg::Eq(something),
            Arg::Any,
            Arg::Eq(arc_foo1.clone()),
        )
        .returns(vec![4, 5, 6])
        .another_work(
            Arg::Any,
            Arg::Any,
            Arg::Any,
            Arg::Is(|foo: Arc<dyn IFoo>| foo.get_value() == 144),
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
        Arg::Eq("que"),
        Arg::Any,
        Arg::Is(|_| false),
        Arg::Eq(Arc::new(Foo(44))),
        Times::Exactly(22),
    );
    my_trait_mock.received.get(Times::Exactly(2));

    println!("Done");
}
