use crate::generated::{MyTraitMock, work_Call};
use std::sync::Arc;

trait Foo {}

trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(&self, string: &str, something: &&[u8], dyn_obj: &dyn Foo, arc: Arc<dyn Foo>);

    fn get(&self) -> i32;
}

mod lib;

mod generated {
    use crate::lib::{FnData, IValuePredicate};
    use crate::{Foo, MyTrait};
    use std::default::Default;
    use std::sync::Arc;

    #[allow(non_camel_case_types)]
    pub struct work_Call {
        pub value: i32,
    }

    #[allow(non_camel_case_types)]
    pub struct another_work_Call {
        pub string: *const str,
        pub something: *const &'static [u8],
        pub dyn_obj: *const dyn Foo,
        pub arc: Arc<dyn Foo>,
    }

    #[allow(non_camel_case_types)]
    pub struct get_Call;

    pub struct MyTraitMock {
        work_data: FnData<work_Call, ()>,
        another_work_data: FnData<another_work_Call, ()>,
        get_data: FnData<get_Call, i32>,
    }

    impl MyTrait for MyTraitMock {
        fn work(&self, value: i32) {
            self.work_data.add_call(work_Call { value });
        }

        fn another_work(
            &self,
            string: &str,
            something: &&[u8],
            dyn_obj: &dyn Foo,
            arc: Arc<dyn Foo>,
        ) {
            self.another_work_data.add_call(unsafe {
                another_work_Call {
                    string,
                    something: std::mem::transmute(something),
                    dyn_obj: std::mem::transmute(dyn_obj),
                    arc,
                }
            });
        }

        fn get(&self) -> i32 {
            self.get_data.add_call(get_Call);
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

        pub fn work(&self, value_predicate: impl IValuePredicate<i32>) -> &FnData<work_Call, ()> {
            &self.work_data
        }
    }
}

fn main() {
    let my_trait = MyTraitMock::new();
    my_trait.work(|value| value == 32).add_call(work_Call { value: 2 });
    // my_trait_mock.assert_work_args(|args| args.value_is(22));
    // my_trait_mock.assert_another_work_args(|args| args.value_is("amogus").something_is(&b"quo vadis"));
}
