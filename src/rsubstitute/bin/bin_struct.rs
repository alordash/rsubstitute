trait Trait {
    fn work(&self);
}

struct Struct {
    number: i32,
}

impl Trait for Struct {
    fn work(&self) {
        println!("working...");
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
        let result = format!("Struct, number = {number}");
        return result;
    }
}

pub use __rsubstitute_generated_Struct::*;
mod __rsubstitute_generated_Struct {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::for_generated::*;

    #[derive(Clone)]
    pub struct Trait_work_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    impl<'a> IArgInfosProvider for Trait_work_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }

    #[derive(Debug, IArgsFormatter)]
    pub struct Trait_work_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }

    impl<'a> IArgsChecker<Trait_work_Call<'a>> for Trait_work_ArgsChecker<'a> {
        fn check(&self, call: Trait_work_Call<'a>) -> Vec<ArgCheckResult> {
            vec![]
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
        Trait_work_data:
            FnData<StructMock<'a>, Trait_work_Call<'a>, Trait_work_ArgsChecker<'a>, ()>,
        get_number_data:
            FnData<StructMock<'a>, get_number_Call<'a>, get_number_ArgsChecker<'a>, i32>,
        format_data: FnData<StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String>,
    }

    pub struct StructMockSetup<'a> {
        data: Arc<StructMockData<'a>>,
    }

    pub struct StructMockReceived<'a> {
        data: Arc<StructMockData<'a>>,
    }

    struct StructInnerData {
        number: i32,
    }

    impl StructInnerData {
        fn new(number: i32) -> Self {
            Self { number }
        }
    }

    #[allow(non_camel_case_types)]
    pub struct StructMock<'a> {
        pub setup: StructMockSetup<'a>,
        pub received: StructMockReceived<'a>,
        data: Arc<StructMockData<'a>>,
        inner_data: StructInnerData,
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
                Trait_work_data: FnData::new("Trait_work", &SERVICES),
                get_number_data: FnData::new("get_number", &SERVICES),
                format_data: FnData::new("format", &SERVICES),
            });
            let inner_data = StructInnerData::new(number);
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
