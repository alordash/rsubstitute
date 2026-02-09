#[allow(mismatched_lifetime_syntaxes)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
mod __rsubstitute_generated_Struct {
    use super::*;
    use rsubstitute::for_generated::*;
    pub struct MyTrait_work_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        value: i32,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for MyTrait_work_Call<'a> {
        #[inline]
        fn clone(&self) -> MyTrait_work_Call<'a> {
            MyTrait_work_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<'a> IArgInfosProvider for MyTrait_work_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            <[_]>::into_vec(::alloc::boxed::box_new([ArgInfo::new(
                "value",
                self.value.clone(),
            )]))
        }
    }
    pub struct MyTrait_work_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        value: Arg<i32>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for MyTrait_work_ArgsChecker<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "MyTrait_work_ArgsChecker",
                "_phantom_lifetime",
                &self._phantom_lifetime,
                "value",
                &&self.value,
            )
        }
    }
    impl<'a> IArgsFormatter for MyTrait_work_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", self.value)) })
        }
    }
    impl<'a> IArgsChecker<MyTrait_work_Call<'a>> for MyTrait_work_ArgsChecker<'a> {
        fn check(&self, call: MyTrait_work_Call<'a>) -> Vec<ArgCheckResult> {
            <[_]>::into_vec(::alloc::boxed::box_new([self
                .value
                .check("value", call.value)]))
        }
    }
    impl<'a> IBaseCaller<MyTrait_work_Call<'a>, String> for MyTrait<'a> {
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
            MyTrait<'a>,
            MyTrait_work_Call<'a>,
            MyTrait_work_ArgsChecker<'a>,
            String,
            Self,
        > {
            let MyTrait_work_args_checker = MyTrait_work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                value: value.into(),
            };
            let fn_config = self
                .data
                .MyTrait_work_data
                .add_config(MyTrait_work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'a> MyTraitReceived<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn work(&'a self, value: impl Into<Arg<i32>>, times: Times) -> &'a Self {
            let MyTrait_work_args_checker = MyTrait_work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                value: value.into(),
            };
            self.data
                .MyTrait_work_data
                .verify_received(MyTrait_work_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'a self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub struct Debug_fmt_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        f: &'a mut Formatter<'a>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for Debug_fmt_Call<'a> {
        #[inline]
        fn clone(&self) -> Debug_fmt_Call<'a> {
            Debug_fmt_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
                f: ::core::clone::Clone::clone(&self.f),
            }
        }
    }
    impl<'a> IArgInfosProvider for Debug_fmt_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            <[_]>::into_vec(::alloc::boxed::box_new([ArgInfo::new("f", self.f.clone())]))
        }
    }
    pub struct Debug_fmt_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        f: Arg<&'a mut Formatter<'a>>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for Debug_fmt_ArgsChecker<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Debug_fmt_ArgsChecker",
                "_phantom_lifetime",
                &self._phantom_lifetime,
                "f",
                &&self.f,
            )
        }
    }
    impl<'a> IArgsFormatter for Debug_fmt_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", self.f)) })
        }
    }
    impl<'a> IArgsChecker<Debug_fmt_Call<'a>> for Debug_fmt_ArgsChecker<'a> {
        fn check(&self, call: Debug_fmt_Call<'a>) -> Vec<ArgCheckResult> {
            <[_]>::into_vec(::alloc::boxed::box_new([self.f.check_ref("f", call.f)]))
        }
    }
    impl<'a> IBaseCaller<Debug_fmt_Call<'a>, std::fmt::Result> for Debug<'a> {
        fn call_base(&self, call: Debug_fmt_Call<'a>) -> std::fmt::Result {
            let Debug_fmt_Call { f, .. } = call;
            return f.write_fmt(format_args!("Struct = {{ number = {0} }}", self.number));
        }
    }
    pub struct DebugSetup<'a> {
        data: Arc<StructMockData<'a>>,
    }
    pub struct DebugReceived<'a> {
        data: Arc<StructMockData<'a>>,
    }
    impl<'a> DebugSetup<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn fmt(
            &'a self,
            f: impl Into<Arg<&'a mut Formatter<'a>>>,
        ) -> SharedFnConfig<
            'a,
            Debug<'a>,
            Debug_fmt_Call<'a>,
            Debug_fmt_ArgsChecker<'a>,
            std::fmt::Result,
            Self,
        > {
            let Debug_fmt_args_checker = Debug_fmt_ArgsChecker {
                _phantom_lifetime: PhantomData,
                f: f.into(),
            };
            let fn_config = self.data.Debug_fmt_data.add_config(Debug_fmt_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'a> DebugReceived<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn fmt(&'a self, f: impl Into<Arg<&'a mut Formatter<'a>>>, times: Times) -> &'a Self {
            let Debug_fmt_args_checker = Debug_fmt_ArgsChecker {
                _phantom_lifetime: PhantomData,
                f: f.into(),
            };
            self.data
                .Debug_fmt_data
                .verify_received(Debug_fmt_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'a self) {
            self.data.verify_received_nothing_else();
        }
    }
    pub struct first_struct_impl_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for first_struct_impl_Call<'a> {
        #[inline]
        fn clone(&self) -> first_struct_impl_Call<'a> {
            first_struct_impl_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
            }
        }
    }
    impl<'a> IArgInfosProvider for first_struct_impl_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct first_struct_impl_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for first_struct_impl_ArgsChecker<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "first_struct_impl_ArgsChecker",
                "_phantom_lifetime",
                &&self._phantom_lifetime,
            )
        }
    }
    impl<'a> IArgsFormatter for first_struct_impl_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("")) })
        }
    }
    impl<'a> IArgsChecker<first_struct_impl_Call<'a>> for first_struct_impl_ArgsChecker<'a> {
        fn check(&self, call: first_struct_impl_Call<'a>) -> Vec<ArgCheckResult> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<'a> IBaseCaller<first_struct_impl_Call<'a>, ()> for StructMock<'a> {
        fn call_base(&self, call: first_struct_impl_Call<'a>) {
            let first_struct_impl_Call { .. } = call;
            {
                ::std::io::_print(format_args!("first_struct_impl\n"));
            };
        }
    }
    pub struct get_number_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for get_number_Call<'a> {
        #[inline]
        fn clone(&self) -> get_number_Call<'a> {
            get_number_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
            }
        }
    }
    impl<'a> IArgInfosProvider for get_number_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct get_number_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for get_number_ArgsChecker<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "get_number_ArgsChecker",
                "_phantom_lifetime",
                &&self._phantom_lifetime,
            )
        }
    }
    impl<'a> IArgsFormatter for get_number_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("")) })
        }
    }
    impl<'a> IArgsChecker<get_number_Call<'a>> for get_number_ArgsChecker<'a> {
        fn check(&self, call: get_number_Call<'a>) -> Vec<ArgCheckResult> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<'a> IBaseCaller<get_number_Call<'a>, i32> for StructMock<'a> {
        fn call_base(&self, call: get_number_Call<'a>) -> i32 {
            let get_number_Call { .. } = call;
            self.number
        }
    }
    pub struct format_Call<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::clone::Clone for format_Call<'a> {
        #[inline]
        fn clone(&self) -> format_Call<'a> {
            format_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
            }
        }
    }
    impl<'a> IArgInfosProvider for format_Call<'a> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            ::alloc::vec::Vec::new()
        }
    }
    pub struct format_ArgsChecker<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
    }
    #[automatically_derived]
    impl<'a> ::core::fmt::Debug for format_ArgsChecker<'a> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "format_ArgsChecker",
                "_phantom_lifetime",
                &&self._phantom_lifetime,
            )
        }
    }
    impl<'a> IArgsFormatter for format_ArgsChecker<'a> {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("")) })
        }
    }
    impl<'a> IArgsChecker<format_Call<'a>> for format_ArgsChecker<'a> {
        fn check(&self, call: format_Call<'a>) -> Vec<ArgCheckResult> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<'a> IBaseCaller<format_Call<'a>, String> for StructMock<'a> {
        fn call_base(&self, call: format_Call<'a>) -> String {
            let format_Call { .. } = call;
            let number = self.get_number();
            let work_result = self.work(number);
            let result = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!(
                    "Struct, number = {0}, work_result = {1}",
                    number, work_result,
                ))
            });
            return result;
        }
    }
    pub struct StructMockData<'a> {
        _phantom_lifetime: PhantomData<&'a ()>,
        first_struct_impl_data: FnData<
            StructMock<'a>,
            first_struct_impl_Call<'a>,
            first_struct_impl_ArgsChecker<'a>,
            (),
        >,
        get_number_data:
            FnData<StructMock<'a>, get_number_Call<'a>, get_number_ArgsChecker<'a>, i32>,
        format_data: FnData<StructMock<'a>, format_Call<'a>, format_ArgsChecker<'a>, String>,
        MyTrait_work_data:
            FnData<StructMock<'a>, MyTrait_work_Call<'a>, MyTrait_work_ArgsChecker<'a>, String>,
        Debug_fmt_data:
            FnData<StructMock<'a>, Debug_fmt_Call<'a>, Debug_fmt_ArgsChecker<'a>, std::fmt::Result>,
    }
    impl<'a> IMockData for StructMockData<'a> {
        fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
            return <[_]>::into_vec(::alloc::boxed::box_new([
                self.first_struct_impl_data
                    .get_unexpected_calls_error_msgs(),
                self.get_number_data.get_unexpected_calls_error_msgs(),
                self.format_data.get_unexpected_calls_error_msgs(),
                self.MyTrait_work_data.get_unexpected_calls_error_msgs(),
                self.Debug_fmt_data.get_unexpected_calls_error_msgs(),
            ]));
        }
    }
    pub struct StructMockSetup<'a> {
        data: Arc<StructMockData<'a>>,
        pub MyTraitSetup: MyTraitSetup<'a>,
        pub DebugSetup: DebugSetup<'a>,
    }
    pub struct StructMockReceived<'a> {
        data: Arc<StructMockData<'a>>,
        pub MyTraitReceived: MyTraitReceived<'a>,
        pub DebugReceived: DebugReceived<'a>,
    }
    struct Struct_InnerData {
        number: i32,
    }
    impl Struct_InnerData {
        pub fn new(number: i32) -> Self {
            Self { number }
        }
    }
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
    impl<'a> Struct for StructMock<'a> {
        fn first_struct_impl<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) {
            let call = unsafe {
                first_struct_impl_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            self.data.first_struct_impl_data.handle_base(&self, call);
        }
        fn get_number<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> i32 {
            let call = unsafe {
                get_number_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.get_number_data.handle_base_returning(&self, call);
        }
        fn format<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) -> String {
            let call = unsafe {
                format_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.format_data.handle_base_returning(&self, call);
        }
    }
    impl<'a> StructMock<'a> {
        #[allow(dead_code)]
        pub fn new() -> Self {
            let data = Arc::new(StructMockData {
                _phantom_lifetime: PhantomData,
                first_struct_impl_data: FnData::new("first_struct_impl", &SERVICES),
                get_number_data: FnData::new("get_number", &SERVICES),
                format_data: FnData::new("format", &SERVICES),
                MyTrait_work_data: FnData::new("MyTrait_work", &SERVICES),
                Debug_fmt_data: FnData::new("Debug_fmt", &SERVICES),
            });
            return StructMock {
                setup: StructMockSetup { data: data.clone() },
                received: StructMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'a> StructMockSetup<'a> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn first_struct_impl(
            &'a self,
        ) -> SharedFnConfig<
            'a,
            StructMock<'a>,
            first_struct_impl_Call<'a>,
            first_struct_impl_ArgsChecker<'a>,
            (),
            Self,
        > {
            let first_struct_impl_args_checker = first_struct_impl_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self
                .data
                .first_struct_impl_data
                .add_config(first_struct_impl_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
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
        pub fn first_struct_impl(&'a self, times: Times) -> &'a Self {
            let first_struct_impl_args_checker = first_struct_impl_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .first_struct_impl_data
                .verify_received(first_struct_impl_args_checker, times);
            return self;
        }
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
