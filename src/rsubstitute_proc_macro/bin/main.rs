use crate::generated::MyTraitMock;
use rsubstitute_core::Times;
use rsubstitute_core::arguments_matching::Arg;
use std::sync::Arc;

trait IFoo {
    fn get_value(&self) -> i32;
}
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

    fn standalone(number: i32) -> f32;

    fn standalone_with_ref(number: &i32) -> f32;
}

mod generated {
    use super::*;
    use rsubstitute_core::arguments_matching::IArgsMatcher;
    use rsubstitute_core::{FnData, SharedFnConfig, Times, arguments_matching::Arg};
    use std::cell::LazyCell;

    // start - Calls
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct work_Call {
        pub value: i32,
    }

    #[allow(non_camel_case_types)]
    pub struct work_ArgsMatcher {
        pub value: Arg<i32>,
    }

    impl IArgsMatcher<work_Call> for work_ArgsMatcher {
        fn matches(&self, call: work_Call) -> bool {
            self.value.matches(call.value)
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct another_work_Call<'a> {
        pub string: &'a str,
        pub something: &'a &'a [u8],
        pub dyn_obj: &'a dyn IFoo,
        pub arc: Arc<dyn IFoo>,
    }

    #[allow(non_camel_case_types)]
    pub struct another_work_ArgsMatcher<'a> {
        pub string: Arg<&'a str>,
        pub something: Arg<&'a &'a [u8]>,
        pub dyn_obj: Arg<&'a dyn IFoo>,
        pub arc: Arg<Arc<dyn IFoo>>,
    }

    impl<'a> IArgsMatcher<another_work_Call<'a>> for another_work_ArgsMatcher<'a> {
        fn matches(&self, call: another_work_Call<'a>) -> bool {
            self.string.matches(call.string)
                && self.something.matches(call.something)
                && self.dyn_obj.matches_ref(call.dyn_obj)
                && self.arc.matches_arc(call.arc)
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct get_Call;

    #[allow(non_camel_case_types)]
    pub struct get_ArgsMatcher;

    impl IArgsMatcher<get_Call> for get_ArgsMatcher {
        fn matches(&self, _call: get_Call) -> bool {
            true
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct standalone_Call {
        number: i32,
    }

    #[allow(non_camel_case_types)]
    pub struct standalone_ArgsMatcher {
        number: Arg<i32>,
    }

    impl IArgsMatcher<standalone_Call> for standalone_ArgsMatcher {
        fn matches(&self, call: standalone_Call) -> bool {
            self.number.matches(call.number)
        }
    }

    // end - Calls
    // start - Mock

    pub struct MyTraitMock<'a> {
        work_data: FnData<work_Call, work_ArgsMatcher, ()>,
        another_work_data: FnData<another_work_Call<'a>, another_work_ArgsMatcher<'a>, Vec<u8>>,
        get_data: FnData<get_Call, get_ArgsMatcher, i32>,
    }

    impl<'a> MyTrait for MyTraitMock<'a> {
        fn work(&self, value: i32) {
            let call = work_Call { value };
            self.work_data.handle(call);
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
                    string: std::mem::transmute(string),
                    something: std::mem::transmute(something),
                    dyn_obj: std::mem::transmute(dyn_obj),
                    arc,
                }
            };
            return self.another_work_data.handle_returning(call);
        }

        fn get(&self) -> i32 {
            let call = get_Call;
            return self.get_data.handle_returning(call);
        }

        fn standalone(number: i32) -> f32 {
            let call = standalone_Call { number };
            return Self::standalone_data.handle_returning(call);
        }

        fn standalone_with_ref(number: &i32) -> f32 {
            todo!()
        }
    }

    impl<'a> MyTraitMock<'a> {
        pub fn new() -> Self {
            Self {
                work_data: Default::default(),
                another_work_data: Default::default(),
                get_data: Default::default(),
            }
        }

        pub fn work(
            &'a self,
            value: Arg<i32>,
        ) -> SharedFnConfig<'a, work_Call, work_ArgsMatcher, (), Self> {
            let work_args_matcher = work_ArgsMatcher { value };
            let fn_config = self.work_data.add_config(work_args_matcher);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn received_work(&'a self, value: Arg<i32>, times: Times) -> &'a Self {
            let work_args_matcher = work_ArgsMatcher { value };
            self.work_data.verify_received(work_args_matcher, &times);
            return self;
        }

        pub fn another_work(
            &'a self,
            string: Arg<&'a str>,
            something: Arg<&'a &'a [u8]>,
            dyn_obj: Arg<&'a dyn IFoo>,
            arc: Arg<Arc<dyn IFoo>>,
        ) -> SharedFnConfig<'a, another_work_Call<'a>, another_work_ArgsMatcher<'a>, Vec<u8>, Self>
        {
            let another_work_args_matcher = another_work_ArgsMatcher {
                string,
                something,
                dyn_obj,
                arc,
            };
            let fn_config = self.another_work_data.add_config(another_work_args_matcher);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn received_another_work(
            &'a self,
            string: Arg<&'a str>,
            something: Arg<&'a &'a [u8]>,
            dyn_obj: Arg<&'a dyn IFoo>,
            arc: Arg<Arc<dyn IFoo>>,
            times: Times,
        ) -> &'a Self {
            let another_work_args_matcher = another_work_ArgsMatcher {
                string,
                something,
                dyn_obj,
                arc,
            };
            self.another_work_data
                .verify_received(another_work_args_matcher, &times);
            return self;
        }

        pub fn get(&'a self) -> SharedFnConfig<'a, get_Call, get_ArgsMatcher, i32, Self> {
            let get_args_matcher = get_ArgsMatcher;
            let fn_config = self.get_data.add_config(get_args_matcher);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn received_get(&'a self, times: Times) -> &'a Self {
            let get_args_matcher = get_ArgsMatcher;
            self.get_data.verify_received(get_args_matcher, &times);
            return self;
        }

        #[allow(non_upper_case_globals)]
        const standalone_data: LazyCell<FnData<standalone_Call, standalone_ArgsMatcher, f32>> =
            LazyCell::new(Default::default);
        pub fn standalone(number: Arg<i32>) -> f32 {
            let standalone_args_matcher = standalone_ArgsMatcher {number};
            let fn_config = Self::standalone_data.add_config(standalone_args_matcher);
            // let shared_fn_config = SharedFnConfig::new()
            todo!()
        }
    }

    // end - Mock
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
            Arg::Is(|foo| foo.get_value() == 144),
        )
        .returns(vec![7, 70, 77])
        .get()
        .returns_and_does(112, || println!("first get call!"))
        .get()
        .returns(900000);
    MyTrait::work(&my_trait_mock, 32);

    let first_another_work =
        MyTrait::another_work(&my_trait_mock, string, something, foo1, arc_foo1);
    println!("first_another_work = {first_another_work:?}");
    let second_another_work =
        MyTrait::another_work(&my_trait_mock, "que", something, foo2, arc_foo2);
    println!("second_another_work = {second_another_work:?}");

    let first_get = MyTrait::get(&my_trait_mock);
    println!("first_get = {first_get}");
    let second_get = MyTrait::get(&my_trait_mock);
    println!("second_get = {second_get}");

    MyTrait::work(&my_trait_mock, 11111);
    // let panics = MyTrait::get(&my_trait_mock);

    my_trait_mock.received_work(Arg::Eq(11111), Times::Once);
    my_trait_mock.received_work(Arg::Any, Times::Exactly(2));
    my_trait_mock.received_another_work(Arg::Any, Arg::Any, Arg::Any, Arg::Any, Times::Exactly(2));
    my_trait_mock.received_get(Times::Exactly(2));

    println!("Done");
}
