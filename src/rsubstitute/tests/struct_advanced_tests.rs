use rsubstitute::macros::*;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct Foo {
    pub number: Vec<i32>,
}

// mocked! {
//     struct Struct;
//
//     impl Struct {
//         pub fn new() -> Self {
//             Self
//         }
//
//         fn fooo(&mut self, Foo { mut number }: Foo, mut qq: &mut &mut &&& &mut i32) {
//             println!("number: {number:?}")
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    #[test]
    fn flex() {}

    #[test]
    fn compile() {}
}

#[cfg(not(test))]
struct Struct;
#[cfg(not(test))]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    fn fooo(&mut self, Foo { mut number }: Foo, mut qq: &mut &mut &&&&mut i32) {
        println!("number: {number:?}")
    }
}
#[cfg(test)]
pub use __rsubstitute_generated_Struct::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Struct {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    #[derive(IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider)]
    pub struct fooo_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        arg_1: Foo,
        qq: *mut &'rs mut &'rs &'rs &'rs &'rs mut i32,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct fooo_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        arg_1: Arg<'rs, Foo>,
        qq: Arg<'rs, &'rs mut &'rs mut &'rs &'rs &'rs &'rs mut i32>,
    }
    impl<'rs> IArgsChecker for fooo_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &fooo_Call<'rs> = dyn_call.downcast_ref();
            vec![
                self.arg_1.check(
                    "arg_1",
                    &call.arg_1,
                    (&ArgPrinter(&&call.arg_1)).debug_string(),
                ),
                self.qq
                    .check_mut("qq", &call.qq, (&ArgPrinter(&&call.qq)).debug_string()),
            ]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructMockData<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        fooo_data: FnData<'rs, StructMock<'rs>, false, true>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructMockSetup<'rs> {
        data: Arc<StructMockData<'rs>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructMockReceived<'rs> {
        data: Arc<StructMockData<'rs>>,
    }
    #[doc(hidden)]
    pub struct Struct_InnerData;

    impl Struct_InnerData {
        pub fn new() -> Self {
            Self
        }
    }
    pub struct StructMock<'rs> {
        pub setup: StructMockSetup<'rs>,
        pub received: StructMockReceived<'rs>,
        data: Arc<StructMockData<'rs>>,
        inner_data: Struct_InnerData,
    }
    impl<'rs> Deref for StructMock<'rs> {
        type Target = Struct_InnerData;

        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl<'rs> StructMock<'rs> {
        fn fooo(&mut self, arg_1: Foo, qq: &mut &mut &&&&mut i32) {
            let call: fooo_Call<'_> = unsafe {
                fooo_Call {
                    _phantom_lifetime: PhantomData,
                    arg_1: core::mem::transmute(arg_1),
                    qq: core::mem::transmute(qq),
                }
            };
            self.data.fooo_data.handle(&self, call);
        }
    }
    impl<'rs> StructMock<'rs> {
        pub fn new() -> Self {
            let data = Arc::new(StructMockData {
                _phantom_lifetime: PhantomData,
                fooo_data: FnData::new("fooo"),
            });
            let inner_data = Struct_InnerData::new();
            return StructMock {
                setup: StructMockSetup { data: data.clone() },
                received: StructMockReceived { data: data.clone() },
                data,
                inner_data,
            };
        }
    }
    impl<'rs> StructMockSetup<'rs> {
        pub fn fooo(
            &self,
            arg_1: impl Into<Arg<'rs, Foo>>,
            qq: impl Into<Arg<'rs, &'rs mut &'rs mut &'rs &'rs &'rs &'rs mut i32>>,
        ) -> FnTuner<
            '_,
            StructMock<'rs>,
            Self,
            (&'rs Foo, &'rs &'rs mut &'rs mut &'rs &'rs &'rs &'rs mut i32),
            (),
            false,
            true,
        > {
            let fooo_args_checker: fooo_ArgsChecker<'rs> = fooo_ArgsChecker {
                _phantom_lifetime: PhantomData,
                arg_1: arg_1.into(),
                qq: qq.into(),
            };
            let fn_tuner: FnTuner<
                '_,
                StructMock<'rs>,
                Self,
                (&'rs Foo, &'rs &'rs mut &'rs mut &'rs &'rs &'rs &'rs mut i32),
                (),
                false,
                true,
            > = self.data.fooo_data.add_config(fooo_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs> StructMockReceived<'rs> {
        pub fn fooo(
            &self,
            arg_1: impl Into<Arg<'rs, Foo>>,
            qq: impl Into<Arg<'rs, &'rs mut &'rs mut &'rs &'rs &'rs &'rs mut i32>>,
            times: Times,
        ) -> FnVerifier<Self, (&'rs Foo, &'rs &'rs mut &'rs mut &'rs &'rs &'rs &'rs mut i32)>
        {
            let fooo_args_checker: fooo_ArgsChecker<'rs> = fooo_ArgsChecker {
                _phantom_lifetime: PhantomData,
                arg_1: arg_1.into(),
                qq: qq.into(),
            };
            self.data
                .fooo_data
                .verify_received(fooo_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
