fn main() {}

trait MyTrait {
    fn work(&self, value: i32) -> String;
}

pub use __rsubstitute_generated_Struct::*;
use rsubstitute_proc_macro::mocked;
use std::fmt::{Debug, Formatter};

mod __rsubstitute_generated_Struct {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    use super::*;
    use rsubstitute::for_generated::*;
    use std::ops::Deref;

    #[derive(Clone)]
    pub struct MyTrait_work_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        pub value: i32,
    }
    impl<'a> IArgInfosProvider for MyTrait_work_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new("value", self.value)]
        }
    }

    #[derive(Debug, IArgsFormatter)]
    pub struct MyTrait_work_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        pub value: Arg<i32>,
    }

    impl<'a> IArgsChecker<MyTrait_work_Call<'a>> for MyTrait_work_ArgsChecker<'a> {
        fn check(&self, call: MyTrait_work_Call<'a>) -> Vec<ArgCheckResult> {
            vec![self.value.check("value", call.value)]
        }
    }

    impl<'a> IBaseCaller<MyTrait_work_Call<'a>, String> for StructMock<'a> {
        fn call_base(&self, call: MyTrait_work_Call<'a>) -> String {
            let MyTrait_work_Call { value, .. } = call;
            return "working...".to_owned();
        }
    }

    pub struct MyTraitSetup<'a> {
        data: Arc<StructMockData<'a>>,
    }

    pub struct MyTraitReceived<'a> {
        data: Arc<StructMockData<'a>>,
    }

    impl<'a> MyTraitSetup<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn work(
            &'a self,
            value: impl Into<Arg<i32>>,
        ) -> SharedFnConfig<
            'a,
            StructMock<'a>,
            MyTrait_work_Call<'a>,
            MyTrait_work_ArgsChecker<'a>,
            String,
            Self,
        > {
            let MyTrait_work_ArgsChecker = MyTrait_work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                value: value.into(),
            };
            let fn_config = self
                .data
                .MyTrait_work_data
                .add_config(MyTrait_work_ArgsChecker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }

    impl<'a> MyTraitReceived<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn work(&'a self, value: impl Into<Arg<i32>>, times: Times) -> &'a Self {
            let MyTrait_work_ArgsChecker = MyTrait_work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                value: value.into(),
            };
            self.data
                .MyTrait_work_data
                .verify_received(MyTrait_work_ArgsChecker, times);
            return self;
        }

        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else()
        }
    }

    #[derive(Clone)]
    pub struct get_number_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    impl<'a> IArgInfosProvider for get_number_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }

    #[derive(Debug, IArgsFormatter)]
    pub struct get_number_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }

    impl<'a> IArgsChecker<get_number_Call<'a>> for get_number_ArgsChecker<'a> {
        fn check(&self, call: get_number_Call<'a>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }

    impl<'a> IBaseCaller<get_number_Call<'a>, i32> for StructMock<'a> {
        fn call_base(&self, call: get_number_Call<'a>) -> i32 {
            let get_number_Call { .. } = call;
            return self.number;
        }
    }

    #[derive(Clone)]
    pub struct format_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    impl<'a> IArgInfosProvider for format_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }

    #[derive(Debug, IArgsFormatter)]
    pub struct format_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }

    impl<'a> IArgsChecker<format_Call<'a>> for format_ArgsChecker<'a> {
        fn check(&self, call: format_Call<'a>) -> Vec<ArgCheckResult> {
            vec![]
        }
    }

    impl<'a> IBaseCaller<format_Call<'a>, String> for StructMock<'a> {
        fn call_base(&self, call: format_Call<'a>) -> String {
            let format_Call { .. } = call;
            let number = self.get_number();
            let work_result = self.work(number);
            let result = format!("Struct, number = {number}, work_result = {work_result}");
            return result;
        }
    }

    #[derive(IMockData)]
    pub struct StructMockData<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        MyTrait_work_data:
            FnData<StructMock<'a>, MyTrait_work_Call<'a>, MyTrait_work_ArgsChecker<'a>, String>,
        get_number_data:
            FnData<StructMock<'a>, get_number_Call<'a>, get_number_ArgsChecker<'a>, i32>,
        format_data: FnData<StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String>,
    }

    pub struct StructMockSetup<'a> {
        data: Arc<StructMockData<'a>>,
        pub MyTrait: MyTraitSetup<'a>,
    }

    pub struct StructMockReceived<'a> {
        data: Arc<StructMockData<'a>>,
        pub MyTrait: MyTraitReceived<'a>,
    }

    pub struct Struct_InnerData {
        number: i32,
    }

    impl Struct_InnerData {
        fn new(number: i32) -> Self {
            Self { number }
        }
    }

    #[allow(non_camel_case_types)]
    pub struct StructMock<'a> {
        pub setup: StructMockSetup<'a>,
        pub received: StructMockReceived<'a>,
        data: Arc<StructMockData<'a>>,
        inner_data: Struct_InnerData,
    }

    impl<'a> Deref for StructMock<'a> {
        type Target = Struct_InnerData;

        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }

    impl<'a> MyTrait for StructMock<'a> {
        fn work(&self, value: i32) -> String {
            let call = unsafe {
                MyTrait_work_Call {
                    _phantom_lifetime: PhantomData,
                    value,
                }
            };
            return self
                .data
                .MyTrait_work_data
                .handle_base_returning(self, call);
        }
    }

    impl<'a> StructMock<'a> {
        pub fn get_number<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> i32 {
            let call = unsafe {
                get_number_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.get_number_data.handle_base_returning(self, call);
        }

        pub fn format<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> String {
            let call = unsafe {
                format_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.format_data.handle_base_returning(self, call);
        }
    }

    impl<'a> StructMock<'a> {
        #[allow(dead_code)]
        pub fn new(number: i32) -> Self {
            let data = Arc::new(StructMockData {
                _phantom_lifetime: PhantomData,
                MyTrait_work_data: FnData::new("MyTrait_work", &SERVICES),
                get_number_data: FnData::new("get_number", &SERVICES),
                format_data: FnData::new("format", &SERVICES),
            });
            let inner_data = Struct_InnerData::new(number);
            return StructMock {
                setup: StructMockSetup {
                    data: data.clone(),
                    MyTrait: MyTraitSetup { data: data.clone() },
                },
                received: StructMockReceived {
                    data: data.clone(),
                    MyTrait: MyTraitReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
    }

    impl<'a> StructMockSetup<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn get_number(
            &'a self,
        ) -> SharedFnConfig<
            'a,
            StructMock<'a>,
            get_number_Call<'a>,
            get_number_ArgsChecker<'a>,
            i32,
            Self,
        > {
            let get_number_args_checker = get_number_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self
                .data
                .get_number_data
                .add_config(get_number_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn format(
            &'a self,
        ) -> SharedFnConfig<'a, StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String, Self>
        {
            let format_args_checker = format_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.format_data.add_config(format_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }

    impl<'a> StructMockReceived<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn get_number(&'a self, times: Times) -> &'a Self {
            let get_number_args_checker = get_number_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .get_number_data
                .verify_received(get_number_args_checker, times);
            return self;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn format(&'a self, times: Times) -> &'a Self {
            let format_args_checker = format_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .format_data
                .verify_received(format_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'a self) {
            self.data.verify_received_nothing_else();
        }
    }
}
