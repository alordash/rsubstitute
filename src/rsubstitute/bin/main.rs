use crate::generated::MyTraitMock;
use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;
use std::fmt::Debug;
use std::sync::Arc;

trait IFoo: Debug {
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

    fn standalone(number: i32) -> f32;

    fn standalone_with_ref(number: &i32) -> f32;
}

mod generated {
    use super::*;
    use rsubstitute::*;
    use std::cell::LazyCell;
    use std::fmt::Debug;

    // start - Calls
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct work_Call {
        pub value: i32,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub struct work_ArgsChecker {
        pub value: Arg<i32>,
    }

    impl IArgsFormatter for work_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("{:?}", self.value)
        }
    }

    impl IArgsChecker<work_Call> for work_ArgsChecker {
        fn matches(&self, call: work_Call) -> Vec<ArgMatchingResult> {
            vec![self.value.check("value", call.value)]
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
    #[derive(Debug)]
    pub struct another_work_ArgsChecker<'a> {
        pub string: Arg<&'a str>,
        pub something: Arg<&'a &'a [u8]>,
        pub dyn_obj: Arg<&'a dyn IFoo>,
        pub arc: Arg<Arc<dyn IFoo>>,
    }

    impl<'a> IArgsFormatter for another_work_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            format!(
                "string {:?}, something {:?}, dyn_obj {:?}, arc {:?}",
                self.string, self.something, self.dyn_obj, self.arc
            )
        }
    }

    impl<'a> IArgsChecker<another_work_Call<'a>> for another_work_ArgsChecker<'a> {
        fn matches(&self, call: another_work_Call<'a>) -> Vec<ArgMatchingResult<'a>> {
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
    pub struct get_Call;

    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub struct get_ArgsChecker;

    impl IArgsFormatter for get_ArgsChecker {
        fn fmt_args(&self) -> String {
            String::new()
        }
    }

    impl IArgsChecker<get_Call> for get_ArgsChecker {
        fn matches(&self, _call: get_Call) -> Vec<ArgMatchingResult> {
            Vec::new()
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct standalone_Call {
        number: i32,
    }

    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub struct standalone_ArgsChecker {
        number: Arg<i32>,
    }

    impl IArgsFormatter for standalone_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("number {:?}", self.number)
        }
    }

    impl IArgsChecker<standalone_Call> for standalone_ArgsChecker {
        fn matches(&self, call: standalone_Call) -> Vec<ArgMatchingResult> {
            vec![self.number.check("number", call.number)]
        }
    }

    // end - Calls
    // start - Mock

    pub struct MyTraitMock<'a> {
        work_data: FnData<work_Call, work_ArgsChecker, ()>,
        another_work_data: FnData<another_work_Call<'a>, another_work_ArgsChecker<'a>, Vec<u8>>,
        get_data: FnData<get_Call, get_ArgsChecker, i32>,
    }

    impl<'a> MyTrait for MyTraitMock<'a> {
        fn work(&self, value: i32) {
            let call = work_Call { value };
            return self.work_data.handle(call);
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

        fn standalone_with_ref(_number: &i32) -> f32 {
            todo!()
        }
    }

    impl<'a> MyTraitMock<'a> {
        pub fn new() -> Self {
            Self {
                work_data: FnData::new("work", SERVICES.error_printer.clone()),
                another_work_data: FnData::new("another_work", SERVICES.error_printer.clone()),
                get_data: FnData::new("get", SERVICES.error_printer.clone()),
            }
        }

        pub fn work(
            &'a self,
            value: Arg<i32>,
        ) -> SharedFnConfig<'a, work_Call, work_ArgsChecker, (), Self> {
            let work_args_checker = work_ArgsChecker { value };
            let fn_config = self.work_data.add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn received_work(&'a self, value: Arg<i32>, times: Times) -> &'a Self {
            let work_args_checker = work_ArgsChecker { value };
            self.work_data.verify_received(work_args_checker, times);
            return self;
        }

        pub fn another_work(
            &'a self,
            string: Arg<&'a str>,
            something: Arg<&'a &'a [u8]>,
            dyn_obj: Arg<&'a dyn IFoo>,
            arc: Arg<Arc<dyn IFoo>>,
        ) -> SharedFnConfig<'a, another_work_Call<'a>, another_work_ArgsChecker<'a>, Vec<u8>, Self>
        {
            let another_work_args_checker = another_work_ArgsChecker {
                string,
                something,
                dyn_obj,
                arc,
            };
            let fn_config = self.another_work_data.add_config(another_work_args_checker);
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
            let another_work_args_checker = another_work_ArgsChecker {
                string,
                something,
                dyn_obj,
                arc,
            };
            self.another_work_data
                .verify_received(another_work_args_checker, times);
            return self;
        }

        pub fn get(&'a self) -> SharedFnConfig<'a, get_Call, get_ArgsChecker, i32, Self> {
            let get_args_checker = get_ArgsChecker;
            let fn_config = self.get_data.add_config(get_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }

        pub fn received_get(&'a self, times: Times) -> &'a Self {
            let get_args_checker = get_ArgsChecker;
            self.get_data.verify_received(get_args_checker, times);
            return self;
        }

        #[allow(non_upper_case_globals)]
        const standalone_data: LazyCell<FnData<standalone_Call, standalone_ArgsChecker, f32>> =
            LazyCell::new(|| FnData::new("standalone", SERVICES.error_printer.clone()));
        pub fn standalone(number: Arg<i32>) -> f32 {
            let standalone_args_checker = standalone_ArgsChecker { number };
            let _fn_config = Self::standalone_data.add_config(standalone_args_checker);
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
        MyTrait::another_work(&my_trait_mock, "que", something, foo2, arc_foo2.clone());
    MyTrait::another_work(&my_trait_mock, "que", something, foo2, arc_foo2.clone());
    MyTrait::another_work(&my_trait_mock, "que", something, foo2, arc_foo2.clone());
    println!("second_another_work = {second_another_work:?}");

    let first_get = MyTrait::get(&my_trait_mock);
    println!("first_get = {first_get}");
    let second_get = MyTrait::get(&my_trait_mock);
    println!("second_get = {second_get}");

    MyTrait::work(&my_trait_mock, 11111);
    // let panics = MyTrait::get(&my_trait_mock);

    my_trait_mock.received_work(Arg::Eq(11111), Times::Once);
    my_trait_mock.received_work(Arg::Any, Times::Exactly(2));
    my_trait_mock.received_another_work(
        Arg::Eq("que"),
        Arg::Any,
        Arg::Is(|_| false),
        Arg::Eq(Arc::new(Foo(44))),
        Times::Exactly(22),
    );
    my_trait_mock.received_get(Times::Exactly(2));

    println!("Done");
}
