#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused)]

use rsubstitute_core::Times;
use rsubstitute_core::args_matching::Arg;
use std::cell::{LazyCell, RefCell};
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(not(test))]
fn global(number: i32) -> String {
    return format!("actual number: {number}");
}

use crate::generic_fn::do_flex;
#[cfg(test)]
use global::global;
use rsubstitute_proc_macro::mock;

#[cfg(test)]
mod global {
    use super::*;
    use rsubstitute::for_generated::*;

    fn base_global(number: i32) -> String {
        return format!("actual number: {number}");
    }

    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub struct global_Call<'a> {
        phantom_lifetime: PhantomData<&'a ()>,
        pub number: i32,
    }

    impl<'a> IArgInfosProvider for global_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            return vec![ArgInfo::new("number", self.number.clone())];
        }
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
    pub struct globalBaseCaller;

    impl<'a> IBaseCaller<global_Call<'a>, String> for globalBaseCaller {
        fn call_base(&self, call: global_Call) -> String {
            return base_global(call.number);
        }
    }

    #[allow(non_camel_case_types)]
    pub struct globalData<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        base_caller: Arc<RefCell<globalBaseCaller>>,
        global_data: FnData<global_Call<'a>, global_ArgsChecker<'a>, String, globalBaseCaller>,
    }

    #[allow(non_camel_case_types)]
    pub struct globalSetup<'a> {
        data: Arc<globalData<'a>>,
    }

    #[allow(non_camel_case_types)]
    pub struct globalReceived<'a> {
        data: Arc<globalData<'a>>,
    }

    #[allow(non_camel_case_types)]
    pub struct globalMock {
        pub setup: globalSetup<'static>,
        pub received: globalReceived<'static>,
        data: Arc<globalData<'static>>,
    }

    unsafe impl Send for globalMock {}
    unsafe impl Sync for globalMock {}

    impl<'a> globalSetup<'a> {
        pub fn setup(
            &'a self,
            number: Arg<'a, i32>,
        ) -> SharedFnConfig<
            'a,
            global_Call<'a>,
            global_ArgsChecker<'a>,
            String,
            Self,
            globalBaseCaller,
        > {
            let global_args_checker = global_ArgsChecker {
                phantom_lifetime: PhantomData,
                number,
            };
            let fn_config = self.data.global_data.add_config(global_args_checker);
            let shared_fn_config =
                SharedFnConfig::new(fn_config, self, Some(self.data.base_caller.clone()));
            return shared_fn_config;
        }
    }

    impl<'a> globalReceived<'a> {
        pub fn received(&'a self, number: Arg<'a, i32>, times: Times) -> &'a Self {
            let global_args_checker = global_ArgsChecker {
                phantom_lifetime: PhantomData,
                number,
            };
            self.data
                .global_data
                .verify_received(global_args_checker, times);
            return self;
        }
    }

    thread_local! {
        #[allow(non_upper_case_globals)]
        static global_MOCK: LazyLock<globalMock> = LazyLock::new(|| {
            let data = Arc::new(globalData {
                _phantom_lifetime: PhantomData,
                base_caller: Arc::new(RefCell::new(globalBaseCaller)),
                global_data: FnData::new("global", &SERVICES),
            });
            return globalMock {
                setup: globalSetup { data: data.clone() },
                received: globalReceived { data: data.clone() },
                data,
            };
        });
    }

    pub fn setup(
        number: Arg<'static, i32>,
    ) -> SharedFnConfig<
        'static,
        global_Call<'static>,
        global_ArgsChecker<'static>,
        String,
        globalSetup<'static>,
        globalBaseCaller,
    > {
        return global_MOCK.as_static().setup.setup(number);
    }

    pub fn received(number: Arg<'static, i32>, times: Times) -> &'static globalReceived<'static> {
        return global_MOCK.as_static().received.received(number, times);
    }

    pub fn global(number: i32) -> String {
        let call = global_Call {
            phantom_lifetime: PhantomData,
            number,
        };
        return global_MOCK
            .as_static()
            .data
            .global_data
            .handle_base_returning(call);
    }
}

#[cfg(test)]
mod tests {
    use crate::global;
    use crate::global::globalBaseCaller;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    #[test]
    pub fn global_test() {
        // Arrange
        global::setup(Arg::Eq(2))
            .call_base()
            .setup(Arg::Eq(143))
            .returns("MOCK: 143".to_string());

        // Act
        let result1 = global(2);
        let result2_1 = global(143);
        // TODO - check how NSubstitute behaves.
        // I think like this:
        // 1. If return only one argument - keep returning it
        // 2. If return many arguments - return only them until ran out of them, then throw
        // let result2_2 = global(143);

        // Assert
        global::received(Arg::Eq(2), Times::Once).received(Arg::Eq(143), Times::Exactly(1));
        // assert_eq!("MOCK: 2", result1);
        assert_eq!("actual number: 2", result1);
        assert_eq!("MOCK: 143", result2_1);
        // assert_eq!("MOCK: 143", result2_2);
    }

    #[test]
    pub fn global_test2() {
        // Arrange
        global::setup(Arg::Eq(11))
            .call_base()
            .setup(Arg::Eq(33))
            .returns("MOCK: 33".to_string());

        // Act
        let result1 = global(11);
        let result2_1 = global(33);
        // TODO - check how NSubstitute behaves.
        // I think like this:
        // 1. If return only one argument - keep returning it
        // 2. If return many arguments - return only them until ran out of them, then throw
        // let result2_2 = global(143);

        // Assert
        global::received(Arg::Eq(11), Times::Once).received(Arg::Eq(33), Times::Exactly(1));
        // assert_eq!("MOCK: 2", result1);
        assert_eq!("actual number: 11", result1);
        assert_eq!("MOCK: 33", result2_1);
        // assert_eq!("MOCK: 143", result2_2);
    }
}

fn main() {
    let global_result = global(22);
    println!("global_result: {global_result}");

    let flex_i32 = do_flex(2);
    dbg!(flex_i32);

    #[derive(Debug)]
    struct WithRef<'a> {
        number: &'a i32,
    }

    let flex_with_ref = do_flex(WithRef { number: &5 });
    dbg!(flex_with_ref);

    let local_number = 44;
    let flex_with_local_ref = do_flex(WithRef {
        number: &local_number,
    });
    dbg!(flex_with_local_ref);

    println!("Done");
}

mod generic_fn {
    use std::any::TypeId;
    use std::cell::{RefCell, UnsafeCell};
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use std::sync::{Arc, LazyLock};
    use transient::{Any, Downcast, Transient};

    #[derive(Transient)]
    struct Mock<T> {
        _phantom_T: PhantomData<T>,
    }

    impl<T> Default for Mock<T> {
        fn default() -> Self {
            Self {
                _phantom_T: PhantomData,
            }
        }
    }

    impl<T> Mock<T> {
        pub fn flex(&self, value: T) -> T {
            flex(value)
        }
    }

    #[derive(Default)]
    struct MocksMap {
        pub map: Arc<RefCell<HashMap<TypeId, *const ()>>>,
    }

    unsafe impl Send for MocksMap {}
    unsafe impl Sync for MocksMap {}

    thread_local! {
        static MOCKS_MAP: LazyLock<MocksMap> = LazyLock::new(Default::default);
    }

    fn flex<T>(value: T) -> T {
        value
    }

    fn get_mock<T>() -> *const Mock<T> {
        let arc_map = MOCKS_MAP.with(|x| x.map.clone());
        let mut map = arc_map.borrow_mut();

        let type_id = typeid::of::<T>();
        let raw_ptr = map
            .entry(type_id)
            .or_insert(Box::leak(Box::new(Mock::<T>::default())) as *const _ as *const ());
        let mock_ptr = *raw_ptr as *const Mock<T>;
        return mock_ptr;
    }

    pub fn do_flex<T>(value: T) -> T {
        let mock = get_mock();
        unsafe {
            return (*mock).flex(value);
        }
    }
}
