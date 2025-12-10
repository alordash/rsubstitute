use crate::fn_config::FnConfig;
use std::default::Default;
use std::sync::Arc;

trait Foo {}

trait MyTrait {
    fn work(&self, value: i32);

    fn another_work(&self, string: &str, something: &&[u8], dyn_obj: &dyn Foo, arc: Arc<dyn Foo>);

    fn get(&self) -> i32;
}

mod fn_config {
    use std::cell::RefCell;

    pub struct FnConfig<TCall, TReturnValue> {
        calls: RefCell<Vec<TCall>>,
        return_value: RefCell<Option<TReturnValue>>,
    }

    impl<TCall, TReturnValue> Default for FnConfig<TCall, TReturnValue> {
        fn default() -> Self {
            Self {
                calls: RefCell::new(Vec::new()),
                return_value: RefCell::new(None),
            }
        }
    }

    impl<TCall, TReturnValue> FnConfig<TCall, TReturnValue> {
        pub fn add_call(&self, call: TCall) -> &Self {
            self.calls.borrow_mut().push(call);
            self
        }

        pub fn set_return_value(&self, return_value: TReturnValue) -> &Self {
            *self.return_value.borrow_mut() = Some(return_value);
            self
        }
        
        pub fn get_return_value(&self) -> TReturnValue {
            let Some(return_value) = self.return_value.borrow_mut().take() else {
                panic!("Return value must've been set!");
            };
            return return_value;
        }
    }
}

#[allow(non_camel_case_types)]
struct work_Call {
    pub value: i32,
}

#[allow(non_camel_case_types)]
struct another_work_Call {
    pub string: *const str,
    pub something: *const &'static [u8],
    pub dyn_obj: *const dyn Foo,
    pub arc: Arc<dyn Foo>,
}

#[allow(non_camel_case_types)]
struct get_Call;

struct MyTraitMock {
    work_config: FnConfig<work_Call, ()>,
    another_work_config: FnConfig<another_work_Call, ()>,
    get_config: FnConfig<get_Call, i32>,
}

impl MyTrait for MyTraitMock {
    fn work(&self, value: i32) {
        self.work_config.add_call(work_Call { value });
    }

    fn another_work(&self, string: &str, something: &&[u8], dyn_obj: &dyn Foo, arc: Arc<dyn Foo>) {
        self.another_work_config.add_call(unsafe {
            another_work_Call {
                string,
                something: std::mem::transmute(something),
                dyn_obj: std::mem::transmute(dyn_obj),
                arc,
            }
        });
    }

    fn get(&self) -> i32 {
        self.get_config.add_call(get_Call);
        return self.get_config.get_return_value();
    }
}

impl MyTraitMock {
    pub fn new() -> Self {
        Self {
            work_config: Default::default(),
            another_work_config: Default::default(),
            get_config: Default::default(),
        }
    }

    pub fn work(&self) -> &FnConfig<work_Call, ()> {
        &self.work_config
    }
}

fn main() {
    let my_trait = MyTraitMock::new();
    my_trait.work().add_call(work_Call { value: 2 });
    // my_trait_mock.assert_work_args(|args| args.value_is(22));
    // my_trait_mock.assert_another_work_args(|args| args.value_is("amogus").something_is(&b"quo vadis"));
}
