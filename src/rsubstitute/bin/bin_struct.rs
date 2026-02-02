trait MyTrait {
    fn work(&self, value: i32) -> String;
}

struct Struct {
    number: i32,
}

impl MyTrait for Struct {
    fn work(&self, value: i32) -> String {
        return "working...".to_owned();
    }
}

impl Struct {
    pub fn new(number: i32) -> Self {
        Self { number }
    }

    pub fn get_number(&self) -> i32 {
        self.number
    }

    pub fn format(&self) -> String {
        let number = self.get_number();
        let work_result = self.work(number);
        let result = format!("Struct, number = {number}, work_result = {work_result}");
        return result;
    }
}

#[cfg(test)]
mod tests {
    use crate::StructMock;
    use rsubstitute_core::Times;
    use rsubstitute_core::args_matching::Arg;

    fn struct_test() {
        // Arrange
        let mock_number = 10;
        let mock = StructMock::new(mock_number);

        let get_number_returned_value = 22;
        mock.setup
            .get_number()
            .returns(get_number_returned_value)
            .format()
            .call_base();

        let my_trait_work_returned_value_for_format = "for format!".to_owned();
        let my_trait_work_accepted_value_for_call_base = 333;
        let my_trait_work_returned_value_for_mock = "Mocked value!".to_owned();
        let my_trait_work_accepted_value_for_mock = 4;
        mock.setup
            .MyTrait
            .work(get_number_returned_value)
            .returns(my_trait_work_returned_value_for_format.clone())
            .work(my_trait_work_accepted_value_for_call_base)
            .call_base()
            .work(my_trait_work_accepted_value_for_mock)
            .returns(my_trait_work_returned_value_for_mock);

        // Act
        let actual_get_number_returned_value = mock.get_number();
        let actual_format_value = mock.format();

        let actual_my_trait_work_call_base_value =
            mock.work(my_trait_work_accepted_value_for_call_base);
        let actual_my_trait_work_returned_value_for_mock =
            mock.work(my_trait_work_accepted_value_for_mock);

        // Assert
        assert_eq!(get_number_returned_value, actual_get_number_returned_value);
        let expected_format_value = format!(
            "Struct, number = {get_number_returned_value}, work_result = {my_trait_work_returned_value_for_format}"
        );
        assert_eq!(expected_format_value, actual_format_value);

        let expected_my_trait_work_call_base_value = "working...".to_owned();
        assert_eq!(
            expected_my_trait_work_call_base_value,
            actual_my_trait_work_call_base_value
        );
        assert_eq!(
            my_trait_work_returned_value_for_mock,
            actual_my_trait_work_returned_value_for_mock
        );

        mock.received
            .get_number(Times::Exactly(2))
            .format(Times::Once)
            .no_other_calls();
        mock.received
            .MyTrait
            .work(my_trait_work_accepted_value_for_call_base, Times::Once)
            .work(my_trait_work_accepted_value_for_mock, Times::Once)
            .work(get_number_returned_value, Times::Once)
            .no_other_calls();
    }
}

pub use __rsubstitute_generated_Struct::*;
mod __rsubstitute_generated_Struct {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::for_generated::*;

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

    #[derive(IMockData)]
    pub struct StructMockData<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        MyTrait_work_data:
            FnData<StructMock<'a>, MyTrait_work_Call<'a>, MyTrait_work_ArgsChecker<'a>, ()>,
        get_number_data:
            FnData<StructMock<'a>, get_number_Call<'a>, get_number_ArgsChecker<'a>, i32>,
        format_data: FnData<StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String>,
    }

    pub struct MyTraitSetup<'a> {
        data: Arc<StructMockData<'a>>,
    }

    pub struct StructMockSetup<'a> {
        data: Arc<StructMockData<'a>>,
    }

    pub struct StructMockReceived<'a> {
        data: Arc<StructMockData<'a>>,
    }

    struct Struct_InnerData {
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

    // impl<'a> Struct for StructMock<'a> {
    //     fn get_number<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> i32 {
    //         let call = unsafe {
    //             get_number_Call {
    //                 _phantom_lifetime: PhantomData,
    //             }
    //         };
    //         return self.data.get_number_data.handle_returning(call);
    //     }
    //     fn format<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> String {
    //         let call = unsafe {
    //             format_Call {
    //                 _phantom_lifetime: PhantomData,
    //             }
    //         };
    //         return self.data.format_data.handle_returning(call);
    //     }
    // }

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
                setup: StructMockSetup { data: data.clone() },
                received: StructMockReceived { data: data.clone() },
                data,
                inner_data,
            };
        }
    }

    impl<'a> StructMockSetup<'a> {
        #[allow(dead_code)]
        #[allow(elided_named_lifetimes)]
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
        #[allow(elided_named_lifetimes)]
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
        #[allow(elided_named_lifetimes)]
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
        #[allow(elided_named_lifetimes)]
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

fn main() {
    println!("done")
}
