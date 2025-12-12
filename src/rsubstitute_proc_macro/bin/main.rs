use crate::generated::MyTraitMock;
use rsubstitute_core::arguments_matching::Arg;
use std::sync::Arc;

trait Foo {}

trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(
        &self,
        string: &str,
        something: &&[u8],
        dyn_obj: &dyn Foo,
        arc: Arc<dyn Foo>,
    ) -> Vec<u8>;

    fn get(&self) -> i32;
}

mod generated {
    use crate::{Foo, MyTrait};
    use rsubstitute_core::arguments_matching::IArgsMatcher;
    use rsubstitute_core::{FnData, SharedFnConfig, arguments_matching::Arg};
    use std::default::Default;
    use std::ops::Deref;
    use std::sync::Arc;

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
        pub dyn_obj: &'a dyn Foo,
        pub arc: Arc<dyn Foo>,
    }

    #[allow(non_camel_case_types)]
    pub struct another_work_ArgsMatcher<'a> {
        pub string: Arg<&'a str>,
        pub something: Arg<&'a &'a [u8]>,
        pub dyn_obj: Arg<&'a dyn Foo>,
        pub arc: Arg<Arc<dyn Foo>>,
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
        fn matches(&self, call: get_Call) -> bool {
            todo!()
        }
    }

    pub struct MyTraitMock<'a> {
        work_data: FnData<work_Call, work_ArgsMatcher, ()>,
        another_work_data: FnData<another_work_Call<'a>, another_work_ArgsMatcher<'a>, Vec<u8>>,
        get_data: FnData<get_Call, get_ArgsMatcher, i32>,
    }

    impl<'a> MyTrait for MyTraitMock<'a> {
        fn work(&self, value: i32) {
            self.work_data.register_call(work_Call { value });
            // if let Some(fn_config) = self.work_data.
        }

        fn another_work(
            &self,
            string: &str,
            something: &&[u8],
            dyn_obj: &dyn Foo,
            arc: Arc<dyn Foo>,
        ) -> Vec<u8> {
            self.another_work_data.register_call(unsafe {
                another_work_Call {
                    string: std::mem::transmute(string),
                    something: std::mem::transmute(something),
                    dyn_obj: std::mem::transmute(dyn_obj),
                    arc,
                }
            });
            return vec![1, 2, 3];
        }

        fn get(&self) -> i32 {
            self.get_data.register_call(get_Call);
            // return self.get_data.get_return_value();
            todo!();
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
            let shared_fn_config = self.work_data.add_config(work_args_matcher);
            let work_config = SharedFnConfig::new(shared_fn_config, self);
            return work_config;
        }

        pub fn another_work(
            &'a self,
            string: Arg<&'a str>,
            something: Arg<&'a &'a [u8]>,
            dyn_obj: Arg<&'a dyn Foo>,
            arc: Arg<Arc<dyn Foo>>,
        ) -> SharedFnConfig<'a, another_work_Call, another_work_ArgsMatcher<'a>, Vec<u8>, Self>
        {
            let another_work_args_matcher = another_work_ArgsMatcher {
                string,
                something,
                dyn_obj,
                arc,
            };
            let shared_fn_config = self.another_work_data.add_config(another_work_args_matcher);
            let another_work_config = SharedFnConfig::new(shared_fn_config, self);
            return another_work_config;
        }
    }
}

fn main() {
    let string = &String::from("amogus");
    let bytes = vec![1u8, 2, 3, 44];
    let something = &&bytes[..];
    let my_trait = MyTraitMock::new();
    my_trait
        .work(Arg::Is(|value| value == 32))
        .does(|| println!("work mock called"))
        .another_work(Arg::Eq(string), Arg::Eq(something), Arg::Any, Arg::Any)
        .returns(vec![4, 5, 6]);
    MyTrait::work(&my_trait, 22);
    // my_trait_mock.assert_work_args(|args| args.value_is(22));
    // my_trait_mock.assert_another_work_args(|args| args.value_is("amogus").something_is(&b"quo vadis"));

    println!("Done");
}
