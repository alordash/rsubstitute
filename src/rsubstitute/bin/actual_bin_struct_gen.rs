pub use __rsubstitute_generated_Struct::*;
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Struct {
    use super::*;
    use rsubstitute::for_generated::*;
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct work_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        value: i32,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::clone::Clone
        for work_Call<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn clone(&self) -> work_Call<'__rsubstitute_arg_field_lifetime> {
            work_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for work_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            <[_]>::into_vec(::alloc::boxed::box_new([ArgInfo::new(
                "value",
                self.value.clone(),
            )]))
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct work_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        value: Arg<i32>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::fmt::Debug
        for work_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "work_ArgsChecker",
                "_phantom_lifetime",
                &self._phantom_lifetime,
                "value",
                &&self.value,
            )
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsFormatter
        for work_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", self.value)) })
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<work_Call<'__rsubstitute_arg_field_lifetime>>
        for work_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: work_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            <[_]>::into_vec(::alloc::boxed::box_new([self
                .value
                .check("value", call.value)]))
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<work_Call<'__rsubstitute_arg_field_lifetime>, String>
        for StructMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: work_Call<'__rsubstitute_arg_field_lifetime>) -> String {
            let work_Call { value, .. } = call;
            return "working...".to_owned();
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct fmt_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        f: &'__rsubstitute_arg_field_lifetime mut Formatter<'__rsubstitute_arg_field_lifetime>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::clone::Clone
        for fmt_Call<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn clone(&self) -> fmt_Call<'__rsubstitute_arg_field_lifetime> {
            fmt_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
                f: ::core::clone::Clone::clone(&self.f),
            }
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for fmt_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            <[_]>::into_vec(::alloc::boxed::box_new([ArgInfo::new("f", self.f.clone())]))
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct fmt_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        f: Arg<&'__rsubstitute_arg_field_lifetime mut Formatter<'__rsubstitute_arg_field_lifetime>>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::fmt::Debug
        for fmt_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "fmt_ArgsChecker",
                "_phantom_lifetime",
                &self._phantom_lifetime,
                "f",
                &&self.f,
            )
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsFormatter
        for fmt_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("{0:?}", self.f)) })
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<fmt_Call<'__rsubstitute_arg_field_lifetime>>
        for fmt_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: fmt_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            <[_]>::into_vec(::alloc::boxed::box_new([self.f.check_ref("f", call.f)]))
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<fmt_Call<'__rsubstitute_arg_field_lifetime>, std::fmt::Result>
        for StructMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: fmt_Call<'__rsubstitute_arg_field_lifetime>) -> std::fmt::Result {
            let fmt_Call { f, .. } = call;
            return f.write_fmt(format_args!("Struct = {{ number = {0} }}", self.number));
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct first_struct_impl_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::clone::Clone
        for first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn clone(&self) -> first_struct_impl_Call<'__rsubstitute_arg_field_lifetime> {
            first_struct_impl_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
            }
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            ::alloc::vec::Vec::new()
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct first_struct_impl_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::fmt::Debug
        for first_struct_impl_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
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
    impl<'__rsubstitute_arg_field_lifetime> IArgsFormatter
        for first_struct_impl_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("")) })
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>>
        for first_struct_impl_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self,
            call: first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>,
        ) -> Vec<ArgCheckResult> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>, ()>
        for StructMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>) {
            let first_struct_impl_Call { .. } = call;
            {
                ::std::io::_print(format_args!("first_struct_impl\n"));
            };
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct new_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        number: i32,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::clone::Clone
        for new_Call<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn clone(&self) -> new_Call<'__rsubstitute_arg_field_lifetime> {
            new_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
                number: ::core::clone::Clone::clone(&self.number),
            }
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for new_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            <[_]>::into_vec(::alloc::boxed::box_new([ArgInfo::new(
                "number",
                self.number.clone(),
            )]))
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct new_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        number: Arg<i32>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::fmt::Debug
        for new_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "new_ArgsChecker",
                "_phantom_lifetime",
                &self._phantom_lifetime,
                "number",
                &&self.number,
            )
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgsFormatter
        for new_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0:?}", self.number))
            })
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<new_Call<'__rsubstitute_arg_field_lifetime>>
        for new_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(&self, call: new_Call<'__rsubstitute_arg_field_lifetime>) -> Vec<ArgCheckResult> {
            <[_]>::into_vec(::alloc::boxed::box_new([self
                .number
                .check("number", call.number)]))
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<new_Call<'__rsubstitute_arg_field_lifetime>, Self>
        for StructMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: new_Call<'__rsubstitute_arg_field_lifetime>) -> Self {
            let new_Call { number, .. } = call;
            Self { number }
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct get_number_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::clone::Clone
        for get_number_Call<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn clone(&self) -> get_number_Call<'__rsubstitute_arg_field_lifetime> {
            get_number_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
            }
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for get_number_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            ::alloc::vec::Vec::new()
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct get_number_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::fmt::Debug
        for get_number_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
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
    impl<'__rsubstitute_arg_field_lifetime> IArgsFormatter
        for get_number_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("")) })
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<get_number_Call<'__rsubstitute_arg_field_lifetime>>
        for get_number_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self,
            call: get_number_Call<'__rsubstitute_arg_field_lifetime>,
        ) -> Vec<ArgCheckResult> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<get_number_Call<'__rsubstitute_arg_field_lifetime>, i32>
        for StructMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: get_number_Call<'__rsubstitute_arg_field_lifetime>) -> i32 {
            let get_number_Call { .. } = call;
            self.number
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct format_Call<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::clone::Clone
        for format_Call<'__rsubstitute_arg_field_lifetime>
    {
        #[inline]
        fn clone(&self) -> format_Call<'__rsubstitute_arg_field_lifetime> {
            format_Call {
                _phantom_lifetime: ::core::clone::Clone::clone(&self._phantom_lifetime),
            }
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> IArgInfosProvider
        for format_Call<'__rsubstitute_arg_field_lifetime>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            ::alloc::vec::Vec::new()
        }
    }
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    pub struct format_ArgsChecker<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    impl<'__rsubstitute_arg_field_lifetime> ::core::fmt::Debug
        for format_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
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
    impl<'__rsubstitute_arg_field_lifetime> IArgsFormatter
        for format_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn fmt_args(&self) -> String {
            ::alloc::__export::must_use({ ::alloc::fmt::format(format_args!("")) })
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IArgsChecker<format_Call<'__rsubstitute_arg_field_lifetime>>
        for format_ArgsChecker<'__rsubstitute_arg_field_lifetime>
    {
        fn check(
            &self,
            call: format_Call<'__rsubstitute_arg_field_lifetime>,
        ) -> Vec<ArgCheckResult> {
            ::alloc::vec::Vec::new()
        }
    }
    impl<'__rsubstitute_arg_field_lifetime>
        IBaseCaller<format_Call<'__rsubstitute_arg_field_lifetime>, String>
        for StructMock<'__rsubstitute_arg_field_lifetime>
    {
        fn call_base(&self, call: format_Call<'__rsubstitute_arg_field_lifetime>) -> String {
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
    pub struct StructMockData<'__rsubstitute_arg_field_lifetime> {
        _phantom_lifetime: PhantomData<&'__rsubstitute_arg_field_lifetime ()>,
        work_data: FnData<
            StructMock<'__rsubstitute_arg_field_lifetime>,
            work_Call<'__rsubstitute_arg_field_lifetime>,
            work_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            String,
        >,
        fmt_data: FnData<
            StructMock<'__rsubstitute_arg_field_lifetime>,
            fmt_Call<'__rsubstitute_arg_field_lifetime>,
            fmt_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            std::fmt::Result,
        >,
        first_struct_impl_data: FnData<
            StructMock<'__rsubstitute_arg_field_lifetime>,
            first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>,
            first_struct_impl_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            (),
        >,
        new_data: FnData<
            StructMock<'__rsubstitute_arg_field_lifetime>,
            new_Call<'__rsubstitute_arg_field_lifetime>,
            new_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            Self,
        >,
        get_number_data: FnData<
            StructMock<'__rsubstitute_arg_field_lifetime>,
            get_number_Call<'__rsubstitute_arg_field_lifetime>,
            get_number_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            i32,
        >,
        format_data: FnData<
            StructMock<'__rsubstitute_arg_field_lifetime>,
            format_Call<'__rsubstitute_arg_field_lifetime>,
            format_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            String,
        >,
    }
    impl<'__rsubstitute_arg_field_lifetime> IMockData
        for StructMockData<'__rsubstitute_arg_field_lifetime>
    {
        fn get_received_nothing_else_error_msgs(&self) -> Vec<Vec<String>> {
            return <[_]>::into_vec(::alloc::boxed::box_new([
                self.work_data.get_unexpected_calls_error_msgs(),
                self.fmt_data.get_unexpected_calls_error_msgs(),
                self.first_struct_impl_data
                    .get_unexpected_calls_error_msgs(),
                self.new_data.get_unexpected_calls_error_msgs(),
                self.get_number_data.get_unexpected_calls_error_msgs(),
                self.format_data.get_unexpected_calls_error_msgs(),
            ]));
        }
    }
    pub struct StructMockSetup<'__rsubstitute_arg_field_lifetime> {
        data: Arc<StructMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    pub struct StructMockReceived<'__rsubstitute_arg_field_lifetime> {
        data: Arc<StructMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    #[allow(non_camel_case_types)]
    pub struct StructMock<'__rsubstitute_arg_field_lifetime> {
        pub setup: StructMockSetup<'__rsubstitute_arg_field_lifetime>,
        pub received: StructMockReceived<'__rsubstitute_arg_field_lifetime>,
        data: Arc<StructMockData<'__rsubstitute_arg_field_lifetime>>,
    }
    // TODO - split fns across various Trait impls and struct impl
    // TODO - impl just `impl StructMock`, not `impl Struct for StructMock`
    impl<'__rsubstitute_arg_field_lifetime> Struct for StructMock<'__rsubstitute_arg_field_lifetime> {
        fn work<'__rsubstitute_arg_anonymous>(
            &'__rsubstitute_arg_anonymous self,
            value: i32,
        ) -> String {
            let call = unsafe {
                work_Call {
                    _phantom_lifetime: PhantomData,
                    value: std::mem::transmute(value),
                }
            };
            return self.data.work_data.handle_base_returning(&self, call);
        }
        fn fmt<'__rsubstitute_arg_anonymous>(
            &'__rsubstitute_arg_anonymous self,
            f: &'__rsubstitute_arg_anonymous mut Formatter<'_>,
        ) -> std::fmt::Result {
            let call = unsafe {
                fmt_Call {
                    _phantom_lifetime: PhantomData,
                    f: std::mem::transmute(f),
                }
            };
            return self.data.fmt_data.handle_base_returning(&self, call);
        }
        fn first_struct_impl<'__rsubstitute_arg_anonymous>(&'__rsubstitute_arg_anonymous self) {
            let call = unsafe {
                first_struct_impl_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            self.data.first_struct_impl_data.handle_base(&self, call);
        }
        fn new<'__rsubstitute_arg_anonymous>(number: i32) -> Self {
            let call = unsafe {
                new_Call {
                    _phantom_lifetime: PhantomData,
                    number: std::mem::transmute(number),
                }
            };
            return self.data.new_data.handle_base_returning(&self, call);
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
    impl<'__rsubstitute_arg_field_lifetime> StructMock<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        pub fn new() -> Self {
            let data = Arc::new(StructMockData {
                _phantom_lifetime: PhantomData,
                work_data: FnData::new("work", &SERVICES),
                fmt_data: FnData::new("fmt", &SERVICES),
                first_struct_impl_data: FnData::new("first_struct_impl", &SERVICES),
                new_data: FnData::new("new", &SERVICES),
                get_number_data: FnData::new("get_number", &SERVICES),
                format_data: FnData::new("format", &SERVICES),
            });
            return StructMock {
                setup: StructMockSetup { data: data.clone() },
                received: StructMockReceived { data: data.clone() },
                data,
            };
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> StructMockSetup<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn work(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<i32>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            StructMock<'__rsubstitute_arg_field_lifetime>,
            work_Call<'__rsubstitute_arg_field_lifetime>,
            work_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            String,
            Self,
        > {
            let work_args_checker = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                value: value.into(),
            };
            let fn_config = self.data.work_data.add_config(work_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn fmt(
            &'__rsubstitute_arg_field_lifetime self,
            f: impl Into<
                Arg<
                    &'__rsubstitute_arg_field_lifetime mut Formatter<
                        '__rsubstitute_arg_field_lifetime,
                    >,
                >,
            >,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            StructMock<'__rsubstitute_arg_field_lifetime>,
            fmt_Call<'__rsubstitute_arg_field_lifetime>,
            fmt_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            std::fmt::Result,
            Self,
        > {
            let fmt_args_checker = fmt_ArgsChecker {
                _phantom_lifetime: PhantomData,
                f: f.into(),
            };
            let fn_config = self.data.fmt_data.add_config(fmt_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn first_struct_impl(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            StructMock<'__rsubstitute_arg_field_lifetime>,
            first_struct_impl_Call<'__rsubstitute_arg_field_lifetime>,
            first_struct_impl_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
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
        pub fn new(
            &'__rsubstitute_arg_field_lifetime self,
            number: impl Into<Arg<i32>>,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            StructMock<'__rsubstitute_arg_field_lifetime>,
            new_Call<'__rsubstitute_arg_field_lifetime>,
            new_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            Self,
            Self,
        > {
            let new_args_checker = new_ArgsChecker {
                _phantom_lifetime: PhantomData,
                number: number.into(),
            };
            let fn_config = self.data.new_data.add_config(new_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn get_number(
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            StructMock<'__rsubstitute_arg_field_lifetime>,
            get_number_Call<'__rsubstitute_arg_field_lifetime>,
            get_number_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
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
            &'__rsubstitute_arg_field_lifetime self,
        ) -> SharedFnConfig<
            '__rsubstitute_arg_field_lifetime,
            StructMock<'__rsubstitute_arg_field_lifetime>,
            format_Call<'__rsubstitute_arg_field_lifetime>,
            format_ArgsChecker<'__rsubstitute_arg_field_lifetime>,
            String,
            Self,
        > {
            let format_args_checker = format_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_config = self.data.format_data.add_config(format_args_checker);
            let shared_fn_config = SharedFnConfig::new(fn_config, self);
            return shared_fn_config;
        }
    }
    impl<'__rsubstitute_arg_field_lifetime> StructMockReceived<'__rsubstitute_arg_field_lifetime> {
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn work(
            &'__rsubstitute_arg_field_lifetime self,
            value: impl Into<Arg<i32>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let work_args_checker = work_ArgsChecker {
                _phantom_lifetime: PhantomData,
                value: value.into(),
            };
            self.data
                .work_data
                .verify_received(work_args_checker, times);
            return self;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn fmt(
            &'__rsubstitute_arg_field_lifetime self,
            f: impl Into<
                Arg<
                    &'__rsubstitute_arg_field_lifetime mut Formatter<
                        '__rsubstitute_arg_field_lifetime,
                    >,
                >,
            >,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let fmt_args_checker = fmt_ArgsChecker {
                _phantom_lifetime: PhantomData,
                f: f.into(),
            };
            self.data.fmt_data.verify_received(fmt_args_checker, times);
            return self;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn first_struct_impl(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
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
        pub fn new(
            &'__rsubstitute_arg_field_lifetime self,
            number: impl Into<Arg<i32>>,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let new_args_checker = new_ArgsChecker {
                _phantom_lifetime: PhantomData,
                number: number.into(),
            };
            self.data.new_data.verify_received(new_args_checker, times);
            return self;
        }
        #[allow(dead_code)]
        #[allow(mismatched_lifetime_syntaxes)]
        pub fn get_number(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
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
        pub fn format(
            &'__rsubstitute_arg_field_lifetime self,
            times: Times,
        ) -> &'__rsubstitute_arg_field_lifetime Self {
            let format_args_checker = format_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .format_data
                .verify_received(format_args_checker, times);
            return self;
        }
        pub fn no_other_calls(&'__rsubstitute_arg_field_lifetime self) {
            self.data.verify_received_nothing_else();
        }
    }
}
