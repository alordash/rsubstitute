use rsubstitute::macros::*;

trait Trait {
    fn get(&self) -> i32;
}

pub const DEFAULT_STRUCT_GET_VALUE: i32 = 200;
pub const DEFAULT_TRAIT_GET_VALUE: i32 = 500;

// mocked_base! {
//     struct Struct;
//
//     impl Struct {
//         pub fn new() -> Self { Self }
//
//         fn get(&self) -> i32 {
//             DEFAULT_STRUCT_GET_VALUE
//         }
//
//         pub fn get_plus_one(&self) -> i32 {
//             let value = self.get() + Trait::get(self);
//             return value;
//         }
//     }
//
//     impl Trait for Struct {
//         fn get(&self) -> i32 {
//             DEFAULT_TRAIT_GET_VALUE
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;
    use rsubstitute::*;

    #[test]
    fn get_plus_one_Ok() {
        // Arrange
        let mock = StructMock::new();

        let value = 302;
        mock.setup.get_plus_one().returns(value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        assert_eq!(value, actual_value);

        mock.received.get_plus_one(Times::Once).no_other_calls();
    }

    #[test]
    fn get_plus_one_CallBase_Ok() {
        // Arrange
        let mock = StructMock::new();

        let struct_value = 302;
        let trait_value = 33;
        mock.setup
            .get()
            .returns(struct_value)
            .get_plus_one()
            .call_base()
            .Trait
            .get()
            .returns(trait_value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = struct_value + trait_value;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .Trait
            .get(Times::Once);
        mock.received.no_other_calls();
    }

    #[test]
    fn get_plus_one_StructCallBase_Ok() {
        // Arrange
        let mock = StructMock::new();

        let trait_value = 33;
        mock.setup
            .get()
            .call_base()
            .get_plus_one()
            .call_base()
            .Trait
            .get()
            .returns(trait_value);

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = DEFAULT_STRUCT_GET_VALUE + trait_value;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .Trait
            .get(Times::Once);
        mock.received.no_other_calls();
    }

    #[test]
    fn get_plus_one_StructAndTraitCallBase_Ok() {
        // Arrange
        let mock = StructMock::new();

        mock.setup
            .get()
            .call_base()
            .get_plus_one()
            .call_base()
            .Trait
            .get()
            .call_base();

        // Act
        let actual_value = mock.get_plus_one();

        // Assert
        let expected_value = DEFAULT_STRUCT_GET_VALUE + DEFAULT_TRAIT_GET_VALUE;
        assert_eq!(expected_value, actual_value);

        mock.received
            .get(Times::Once)
            .get_plus_one(Times::Once)
            .Trait
            .get(Times::Once);
        mock.received.no_other_calls();
    }
}

#[cfg(not(test))]
struct Struct;
#[cfg(not(test))]
impl Trait for Struct {
    fn get(&self) -> i32 {
        DEFAULT_TRAIT_GET_VALUE
    }
}
#[cfg(not(test))]
impl Struct {
    pub fn new() -> Self {
        Self
    }

    fn get(&self) -> i32 {
        DEFAULT_STRUCT_GET_VALUE
    }

    pub fn get_plus_one(&self) -> i32 {
        let value = self.get() + Trait::get(self);
        return value;
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
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct Trait_get_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct Trait_get_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for Trait_get_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_get_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitSetup<'rs> {
        data: Arc<StructMockData<'rs>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitReceived<'rs> {
        data: Arc<StructMockData<'rs>>,
    }
    impl<'rs> TraitSetup<'rs> {
        pub fn get(&self) -> FnTuner<'_, Self, (), i32, true> {
            let Trait_get_args_checker: Trait_get_ArgsChecker<'rs> = Trait_get_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, true> = self
                .data
                .Trait_get_data
                .add_config(Trait_get_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs> TraitReceived<'rs> {
        pub fn get(&self, times: Times) -> FnVerifier<Self, ()> {
            let Trait_get_args_checker: Trait_get_ArgsChecker<'rs> = Trait_get_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data
                .Trait_get_data
                .verify_received(Trait_get_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct get_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct get_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for get_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(
        IArgsInfosProvider, IArgsTupleProvider, IGenericsHashKeyProvider, CloneForRSubstitute,
    )]
    pub struct get_plus_one_Call<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter, IGenericsHashKeyProvider)]
    pub struct get_plus_one_ArgsChecker<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
    }
    impl<'rs> IArgsChecker for get_plus_one_ArgsChecker<'rs> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &get_plus_one_Call<'rs> = dyn_call.downcast_ref();
            vec![]
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructMockData<'rs> {
        _phantom_lifetime: PhantomData<&'rs ()>,
        get_data: FnData<'rs, StructMock<'rs>, true>,
        get_plus_one_data: FnData<'rs, StructMock<'rs>, true>,
        Trait_get_data: FnData<'rs, StructMock<'rs>, true>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructMockSetup<'rs> {
        data: Arc<StructMockData<'rs>>,
        pub Trait: TraitSetup<'rs>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructMockReceived<'rs> {
        data: Arc<StructMockData<'rs>>,
        pub Trait: TraitReceived<'rs>,
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
    impl<'rs> Trait for StructMock<'rs> {
        fn get(&self) -> i32 {
            let call: Trait_get_Call<'_> = unsafe {
                Trait_get_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self
                .data
                .Trait_get_data
                // TODO - create separate base fn for implemented Traits methods
                .handle_base_returning(&self, call, Self::base_Trait_get);
        }
    }
    impl<'rs> StructMock<'rs> {
        fn get(&self) -> i32 {
            let call: get_Call<'_> = unsafe {
                get_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self
                .data
                .get_data
                .handle_base_returning(&self, call, Self::base_get);
        }

        pub fn get_plus_one(&self) -> i32 {
            let call: get_plus_one_Call<'_> = unsafe {
                get_plus_one_Call {
                    _phantom_lifetime: PhantomData,
                }
            };
            return self.data.get_plus_one_data.handle_base_returning(
                &self,
                call,
                Self::base_get_plus_one,
            );
        }
    }
    impl<'rs> StructMock<'rs> {
        pub fn new() -> Self {
            let data = Arc::new(StructMockData {
                _phantom_lifetime: PhantomData,
                get_data: FnData::new("get"),
                get_plus_one_data: FnData::new("get_plus_one"),
                Trait_get_data: FnData::new("Trait::get"),
            });
            let inner_data = Struct_InnerData::new();
            return StructMock {
                setup: StructMockSetup {
                    data: data.clone(),
                    Trait: TraitSetup { data: data.clone() },
                },
                received: StructMockReceived {
                    data: data.clone(),
                    Trait: TraitReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
        fn base_get(&self, call: get_Call<'rs>) -> i32 {
            let get_Call { .. } = call;
            DEFAULT_STRUCT_GET_VALUE
        }

        fn base_get_plus_one(&self, call: get_plus_one_Call<'rs>) -> i32 {
            let get_plus_one_Call { .. } = call;
            let value = self.get() + Trait::get(self);
            return value;
        }
        fn base_Trait_get(&self, call: Trait_get_Call<'rs>) -> i32 {
            let Trait_get_Call { .. } = call;
            DEFAULT_TRAIT_GET_VALUE
        }
    }
    impl<'rs> StructMockSetup<'rs> {
        pub fn get(&self) -> FnTuner<'_, Self, (), i32, true> {
            let get_args_checker: get_ArgsChecker<'rs> = get_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Self, (), i32, true> =
                self.data.get_data.add_config(get_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
        pub fn get_plus_one(&self) -> FnTuner<'_, Self, (), i32, true> {
            let get_plus_one_args_checker: get_plus_one_ArgsChecker<'rs> =
                get_plus_one_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                };
            let fn_tuner: FnTuner<'_, Self, (), i32, true> = self
                .data
                .get_plus_one_data
                .add_config(get_plus_one_args_checker, self);
            return unsafe { core::mem::transmute(fn_tuner) };
        }
    }
    impl<'rs> StructMockReceived<'rs> {
        pub fn get(&self, times: Times) -> FnVerifier<Self, ()> {
            let get_args_checker: get_ArgsChecker<'rs> = get_ArgsChecker {
                _phantom_lifetime: PhantomData,
            };
            self.data.get_data.verify_received(get_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn get_plus_one(&self, times: Times) -> FnVerifier<Self, ()> {
            let get_plus_one_args_checker: get_plus_one_ArgsChecker<'rs> =
                get_plus_one_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                };
            self.data
                .get_plus_one_data
                .verify_received(get_plus_one_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
