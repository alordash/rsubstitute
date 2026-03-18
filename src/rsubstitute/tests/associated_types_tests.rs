#![feature(associated_type_defaults)]

use rsubstitute::prelude::*;
use std::fmt::Debug;

#[mock(base)]
trait Trait {
    const CONST: usize = 43;

    type InputType<TAmogus: Clone>: Clone + Debug
        = i32
    where
        Self: Clone;

    type OutputType<TT>: Clone + Sized + Default
        = u8
    where
        Self: Sized,
        TT: Clone;

    fn get_const(&self) -> usize {
        Self::CONST
    }

    fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
    where
        Self: Clone + Sized,
        TT: ToString;
}

// mocked_base! {
//     #[derive(Clone)]
//     struct Struct;
//
//     impl Struct {
//         pub fn new() -> Self {
//             Self
//         }
//     }
//
//     impl Trait for Struct {
//         fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
//         where
//             Self: Clone + Sized,
//             TT: ToString,
//         {
//             Self::OutputType::<TT>::default()
//         }
//     }
// }

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    const TEST_CONST: usize = 111;
    type TestInputType = i32;
    type TestOutputType = &'static str;

    #[test]
    fn get_const_Trait_Ok() {
        // Arrange
        let mock = TraitMock::<TEST_CONST, TestInputType, TestOutputType>::new();
        let const_value = TEST_CONST * 2;
        mock.setup.get_const().returns(const_value);

        // Act
        let actual_const_value = mock.get_const();

        // Assert
        assert_eq!(const_value, actual_const_value);
        mock.received.get_const(Times::Once).no_other_calls();
    }

    #[test]
    fn get_const_TraitBase_Ok() {
        // Arrange
        let mock = TraitMock::<TEST_CONST, TestInputType, TestOutputType>::new();
        mock.setup.get_const().call_base();

        // Act
        let actual_const_value = mock.get_const();

        // Assert
        assert_eq!(TEST_CONST, actual_const_value);
        mock.received.get_const(Times::Once).no_other_calls();
    }

    #[test]
    fn get_my_type_Trait_Ok() {
        // Arrange
        let mock = TraitMock::<TEST_CONST, TestInputType, TestOutputType>::new();

        type FirstTT = u128;
        let first_input: TestInputType = 10;
        let first_output: TestOutputType = "quo vadis";
        type SecondTT = f64;
        let second_input: TestInputType = 20;
        let second_output: TestOutputType = "veridis quo";
        type UnknownTT = i16;

        mock.setup
            .get_my_type::<FirstTT>(first_input)
            .returns(first_output)
            .get_my_type::<SecondTT>(second_input)
            .returns(second_output);

        // Act
        let actual_first_output = mock.get_my_type::<FirstTT>(first_input);
        let actual_second_output = mock.get_my_type::<SecondTT>(second_input);

        // Assert
        assert_eq!(first_output, actual_first_output);
        assert_eq!(second_output, actual_second_output);

        mock.received
            .get_my_type::<FirstTT>(first_input, Times::Once)
            .get_my_type::<UnknownTT>(first_input, Times::Never)
            .get_my_type::<SecondTT>(second_input, Times::Once)
            .get_my_type::<UnknownTT>(second_input, Times::Never)
            .no_other_calls();
    }

    #[test]
    fn get_my_type_Struct_Ok() {
        // Arrange
        let mock = Struct::new();

        type FirstTT = u128;
        let first_input: i32 = 10;
        let first_output: u8 = 3;
        type SecondTT = f64;
        let second_input: i32 = 20;
        let second_output: u8 = 67;
        type UnknownTT = i16;

        mock.setup
            .as_Trait
            .get_my_type::<FirstTT>(first_input)
            .returns(first_output)
            .get_my_type::<SecondTT>(second_input)
            .returns(second_output);

        // Act
        let actual_first_output = mock.get_my_type::<FirstTT>(first_input);
        let actual_second_output = mock.get_my_type::<SecondTT>(second_input);

        // Assert
        assert_eq!(first_output, actual_first_output);
        assert_eq!(second_output, actual_second_output);

        mock.received
            .as_Trait
            .get_my_type::<FirstTT>(first_input, Times::Once)
            .get_my_type::<UnknownTT>(first_input, Times::Never)
            .get_my_type::<SecondTT>(second_input, Times::Once)
            .get_my_type::<UnknownTT>(second_input, Times::Never);
        mock.received.no_other_calls();
    }

    #[test]
    fn get_my_type_StructBase_Ok() {
        // Arrange
        let mock = Struct::new();

        type FirstTT = u128;
        let first_input: i32 = 10;
        type SecondTT = f64;
        let second_input: i32 = 20;
        type UnknownTT = i16;

        mock.setup
            .as_Trait
            .get_my_type::<FirstTT>(first_input)
            .call_base()
            .get_my_type::<SecondTT>(second_input)
            .call_base();

        // Act
        let actual_first_output = mock.get_my_type::<FirstTT>(first_input);
        let actual_second_output = mock.get_my_type::<SecondTT>(second_input);

        // Assert
        let expected_output = u8::default();
        assert_eq!(expected_output, actual_first_output);
        assert_eq!(expected_output, actual_second_output);

        mock.received
            .as_Trait
            .get_my_type::<FirstTT>(first_input, Times::Once)
            .get_my_type::<UnknownTT>(first_input, Times::Never)
            .get_my_type::<SecondTT>(second_input, Times::Once)
            .get_my_type::<UnknownTT>(second_input, Times::Never);
        mock.received.no_other_calls();
    }
}

#[cfg(not(test))]
#[derive(Clone)]
struct Struct;
#[cfg(not(test))]
impl Trait for Struct {
    fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
    where
        Self: Clone + Sized,
        TT: ToString,
    {
        Self::OutputType::<TT>::default()
    }
}
#[cfg(not(test))]
impl Struct {
    pub fn new() -> Self {
        Self
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
    #[derive(IArgsInfosProvider, IArgsTupleProvider, CloneForRSubstitute)]
    pub struct Trait_get_my_type_Call<'__rs, TT: Clone>
    where
        TT: ToString,
    {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_TT: PhantomData<TT>,
        input: <Struct<'__rs> as Trait>::InputType<i32>,
    }
    impl<'__rs, TT: Clone> IGenericsInfoProvider for Trait_get_my_type_Call<'__rs, TT>
    where
        TT: ToString,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("TT", core::any::type_name::<TT>())]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<TT>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(Debug, IArgsFormatter)]
    pub struct Trait_get_my_type_ArgsChecker<'__rs, TT: Clone>
    where
        TT: ToString,
    {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        _phantom_TT: PhantomData<TT>,
        input: Arg<'__rs, <Struct<'__rs> as Trait>::InputType<i32>>,
    }
    impl<'__rs, TT: Clone> IArgsChecker for Trait_get_my_type_ArgsChecker<'__rs, TT>
    where
        TT: ToString,
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_get_my_type_Call<'__rs, TT> = dyn_call.downcast_ref();
            vec![self.input.check(
                "input",
                &call.input,
                (&ArgPrinter(&&call.input)).debug_string(),
            )]
        }
    }
    impl<'__rs, TT: Clone> IGenericsInfoProvider for Trait_get_my_type_ArgsChecker<'__rs, TT>
    where
        TT: ToString,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![generic_type_info("TT", core::any::type_name::<TT>())]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<TT>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitSetup<'__rs> {
        data: Arc<StructData<'__rs>>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct TraitReceived<'__rs> {
        data: Arc<StructData<'__rs>>,
    }
    impl<'__rs> TraitSetup<'__rs> {
        pub fn get_my_type<'__rsa, TT: Clone>(
            &self,
            input: impl Into<Arg<'__rsa, <Struct<'__rsa> as Trait>::InputType<i32>>>,
        ) -> FnTuner<
            '_,
            Struct<'__rs>,
            Self,
            (&'__rs <Struct<'__rs> as Trait>::InputType<i32>,),
            <Struct<'__rsa> as Trait>::OutputType<TT>,
            true,
            true,
        >
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let Trait_get_my_type_args_checker: Trait_get_my_type_ArgsChecker<'_, TT> =
                Trait_get_my_type_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_TT: PhantomData,
                    input: transmute_lifetime!(input.into()),
                };
            let fn_tuner: FnTuner<
                '_,
                Struct<'__rs>,
                Self,
                (&'__rs <Struct<'__rs> as Trait>::InputType<i32>,),
                <Struct<'__rsa> as Trait>::OutputType<TT>,
                true,
                true,
            > = self
                .data
                .Trait_get_my_type
                .add_config(Trait_get_my_type_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'__rs> TraitReceived<'__rs> {
        pub fn get_my_type<'__rsa, TT: Clone>(
            &self,
            input: impl Into<Arg<'__rsa, <Struct<'__rsa> as Trait>::InputType<i32>>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rs <Struct<'__rs> as Trait>::InputType<i32>,)>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let Trait_get_my_type_args_checker: Trait_get_my_type_ArgsChecker<'_, TT> =
                Trait_get_my_type_ArgsChecker {
                    _phantom_lifetime: PhantomData,
                    _phantom___rs: PhantomData,
                    _phantom_TT: PhantomData,
                    input: transmute_lifetime!(input.into()),
                };
            self.data
                .Trait_get_my_type
                .verify_received(Trait_get_my_type_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData<'__rs> {
        _phantom_lifetime: PhantomData<&'__rs ()>,
        _phantom___rs: PhantomData<&'__rs ()>,
        pub Trait_get_my_type: FnData<'static, Struct<'__rs>, true, true>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructSetup<'__rs> {
        data: Arc<StructData<'__rs>>,
        pub as_Trait: TraitSetup<'__rs>,
    }
    #[doc(hidden)]
    #[derive(CloneForRSubstitute)]
    pub struct StructReceived<'__rs> {
        data: Arc<StructData<'__rs>>,
        pub as_Trait: TraitReceived<'__rs>,
    }
    #[derive(Clone)]
    #[doc(hidden)]
    pub struct Struct_InnerData;

    impl Struct_InnerData {
        pub fn new() -> Self {
            Self
        }
    }
    #[derive(Clone)]
    pub struct Struct<'__rs> {
        pub setup: StructSetup<'__rs>,
        pub received: StructReceived<'__rs>,
        pub data: Arc<StructData<'__rs>>,
        inner_data: Struct_InnerData,
    }
    impl<'__rs> Deref for Struct<'__rs> {
        type Target = Struct_InnerData;

        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl<'__rs> Trait for Struct<'__rs> {
        fn get_my_type<TT: Clone>(
            &self,
            input: <Struct<'__rs> as Trait>::InputType<i32>,
        ) -> <Struct<'__rs> as Trait>::OutputType<TT>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let call: Trait_get_my_type_Call<'_, TT> = Trait_get_my_type_Call {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                _phantom_TT: PhantomData,
                input: transmute_lifetime!(input),
            };
            return self.data.Trait_get_my_type.handle_base_returning(
                self,
                call,
                Self::base_Trait_get_my_type,
            );
        }
    }
    impl<'__rs> Struct<'__rs> {}
    impl<'__rs> Struct<'__rs> {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                _phantom_lifetime: PhantomData,
                _phantom___rs: PhantomData,
                Trait_get_my_type: FnData::new("Trait::get_my_type"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup {
                    data: data.clone(),
                    as_Trait: TraitSetup { data: data.clone() },
                },
                received: StructReceived {
                    data: data.clone(),
                    as_Trait: TraitReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
        fn base_Trait_get_my_type<'q, TT: Clone>(
            &self,
            call: Trait_get_my_type_Call<'q, TT>,
        ) -> <Struct<'q> as Trait>::OutputType<TT>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let Trait_get_my_type_Call::<'__rs, TT> { input: input, .. } = transmute_lifetime!(call);
            // <Struct<'__rs> as Trait>::OutputType::<TT>::default()
            todo!()
        }
    }
    impl<'__rs> StructSetup<'__rs> {}
    impl<'__rs> StructReceived<'__rs> {
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
