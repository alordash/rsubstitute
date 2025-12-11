use crate::generated::{MyTraitMock, work_Call};
use crate::lib::argument_matching::Arg;
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

mod lib;

mod generated {
    use crate::lib::argument_matching::Arg;
    use crate::lib::{FnConfig, FnData};
    use crate::{Foo, MyTrait};
    use std::default::Default;
    use std::sync::Arc;

    #[allow(non_camel_case_types)]
    pub struct work_Call {
        pub value: i32,
    }

    #[allow(non_camel_case_types)]
    pub struct work_ArgsMatcher {
        pub value: Arg<i32>,
    }

    #[allow(non_camel_case_types)]
    pub struct another_work_Call {
        pub string: *const str,
        pub something: *const &'static [u8],
        pub dyn_obj: *const dyn Foo,
        pub arc: Arc<dyn Foo>,
    }

    #[allow(non_camel_case_types)]
    pub struct another_work_ArgsMatcher {
        pub string: Arg<*const str>,
        pub something: Arg<*const &'static [u8]>,
        pub dyn_obj: Arg<*const dyn Foo>,
        pub arc: Arg<Arc<dyn Foo>>,
    }

    #[allow(non_camel_case_types)]
    pub struct get_Call;

    pub struct MyTraitMock {
        work_data: FnData<work_Call, work_ArgsMatcher, ()>,
        another_work_data: FnData<another_work_Call, another_work_ArgsMatcher, Vec<u8>>,
        get_data: FnData<get_Call, (), i32>,
    }

    impl MyTrait for MyTraitMock {
        fn work(&self, value: i32) {
            self.work_data.register_call(work_Call { value });
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
                    string,
                    something: std::mem::transmute(something),
                    dyn_obj: std::mem::transmute(dyn_obj),
                    arc,
                }
            });
            return vec![1, 2, 3];
        }

        fn get(&self) -> i32 {
            self.get_data.register_call(get_Call);
            return self.get_data.get_return_value();
        }
    }

    impl MyTraitMock {
        pub fn new() -> Self {
            Self {
                work_data: Default::default(),
                another_work_data: Default::default(),
                get_data: Default::default(),
            }
        }

        pub fn work(&self, value: Arg<i32>) -> &mut FnConfig<work_ArgsMatcher, ()> {
            let work_args_matcher = work_ArgsMatcher { value };
            let work_config = FnConfig::new(work_args_matcher);
            return self.work_data.add_config(work_config);
        }
    }
}

fn main() {
    let my_trait = MyTraitMock::new();
    my_trait.work(Arg::Is(|value| value == 32)).returns(());
    // my_trait_mock.assert_work_args(|args| args.value_is(22));
    // my_trait_mock.assert_another_work_args(|args| args.value_is("amogus").something_is(&b"quo vadis"));
}
