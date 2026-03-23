#![feature(associated_type_defaults)]

use rsubstitute::prelude::*;
use std::fmt::Debug;

// TODO - test how TraitA : TraitB behaves
// TODO - write that mock(base) has no effect on static fns
#[mock]
fn f() {}

// #[mock(base)]
// trait Trait {
//     const CONST: usize = 43;
//
//     type InputType<TAmogus: Clone>: Clone + Debug
//         = i32
//     where
//         Self: Clone;
//
//     type OutputType<TT>: Clone + Sized + Default
//         = u8
//     where
//         Self: Sized,
//         TT: Clone;
//
//     fn get_const(&self) -> usize {
//         Self::CONST
//     }
//
//     fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
//     where
//         Self: Clone + Sized,
//         TT: ToString;
// }

// TODO - write in docs about limitation: `Self` should not be used ambigiously, e.g.
// correct: <Self as Trait>::OutputType
//   wrong: Self::OutputType
mocked_base! {
    #[derive(Clone)]
    struct Struct;

    impl Struct {
        pub fn new() -> Self {
            Self
        }
    }

    impl Trait for Struct {
        fn get_my_type<TT: Clone>(&self, input: Self::InputType<i32>) -> Self::OutputType<TT>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            <Self as Trait>::OutputType::<TT>::default()
        }
    }
}

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
#[cfg(test)]
pub use __rsubstitute_generated_Trait::*;
#[cfg(test)]
#[allow(unused_parens)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(mismatched_lifetime_syntaxes)]
mod __rsubstitute_generated_Trait {
    use super::*;
    use rsubstitute::for_generated::*;
    #[doc(hidden)]
    pub struct Trait_get_const_Call<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > {
        _phantom_GenericParam_Trait_InputType: PhantomData<Trait_InputType>,
        _phantom_GenericParam_Trait_OutputType: PhantomData<Trait_OutputType>,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > IArgsInfosProvider for Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > IArgsTupleProvider for Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > IGenericsInfoProvider
        for Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > Clone for Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn clone(&self) -> Self {
            Self {
                _phantom_GenericParam_Trait_InputType: (&self
                    ._phantom_GenericParam_Trait_InputType)
                    .clone(),
                _phantom_GenericParam_Trait_OutputType: (&self
                    ._phantom_GenericParam_Trait_OutputType)
                    .clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_get_const_ArgsChecker<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > {
        _phantom_GenericParam_Trait_InputType: PhantomData<Trait_InputType>,
        _phantom_GenericParam_Trait_OutputType: PhantomData<Trait_OutputType>,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > IArgsChecker for Trait_get_const_ArgsChecker<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType> =
                dyn_call.downcast_ref();
            vec![]
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > IArgsFormatter
        for Trait_get_const_ArgsChecker<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > IGenericsInfoProvider
        for Trait_get_const_ArgsChecker<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct Trait_get_my_type_Call<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    >
    where
        TT: ToString,
    {
        _phantom_GenericParam_Trait_InputType: PhantomData<Trait_InputType>,
        _phantom_GenericParam_Trait_OutputType: PhantomData<Trait_OutputType>,
        _phantom_input: PhantomData<
            <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,
        >,
        _phantom_GenericParam_TT: PhantomData<TT>,
        input: <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > IArgsInfosProvider
        for Trait_get_my_type_Call<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "input",
                &self.input,
                (&ArgPrinter(&self.input)).debug_string(),
            )]
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > IArgsTupleProvider
        for Trait_get_my_type_Call<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.input,))) as *mut _ as *mut ()
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > IGenericsInfoProvider
        for Trait_get_my_type_Call<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > Clone for Trait_get_my_type_Call<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        fn clone(&self) -> Self {
            Self {
                _phantom_GenericParam_Trait_InputType: (&self
                    ._phantom_GenericParam_Trait_InputType)
                    .clone(),
                _phantom_GenericParam_Trait_OutputType: (&self
                    ._phantom_GenericParam_Trait_OutputType)
                    .clone(),
                _phantom_input: (&self._phantom_input).clone(),
                _phantom_GenericParam_TT: (&self._phantom_GenericParam_TT).clone(),
                input: (&self.input).clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_get_my_type_ArgsChecker<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    >
    where
        TT: ToString,
    {
        _phantom_GenericParam_Trait_InputType: PhantomData<Trait_InputType>,
        _phantom_GenericParam_Trait_OutputType: PhantomData<Trait_OutputType>,
        _phantom_input: PhantomData<
            <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,
        >,
        _phantom_GenericParam_TT: PhantomData<TT>,
        input: Arg<
            <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,
        >,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > IArgsChecker
        for Trait_get_my_type_ArgsChecker<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_get_my_type_Call<Trait_CONST, Trait_InputType, Trait_OutputType, TT> =
                dyn_call.downcast_ref();
            vec![self.input.check(
                "input",
                transmute_lifetime!(&call.input),
                (&ArgPrinter(&call.input)).debug_string(),
            )]
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > IArgsFormatter
        for Trait_get_my_type_ArgsChecker<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        fn fmt_args(&self) -> String {
            format!("{}", (&ArgPrinter(&self.input)).debug_string())
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
        TT: Clone,
    > IGenericsInfoProvider
        for Trait_get_my_type_ArgsChecker<Trait_CONST, Trait_InputType, Trait_OutputType, TT>
    where
        TT: ToString,
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct TraitMockData<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > {
        _phantom_GenericParam_Trait_InputType: PhantomData<Trait_InputType>,
        _phantom_GenericParam_Trait_OutputType: PhantomData<Trait_OutputType>,
        pub Trait_get_const:
            FnData<'static, TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>, true, false>,
        pub Trait_get_my_type: FnData<
            'static,
            TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>,
            false,
            false,
        >,
    }
    #[doc(hidden)]
    pub struct TraitMockSetup<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > {
        data: Arc<TraitMockData<Trait_CONST, Trait_InputType, Trait_OutputType>>,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > Clone for TraitMockSetup<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct TraitMockReceived<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > {
        data: Arc<TraitMockData<Trait_CONST, Trait_InputType, Trait_OutputType>>,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > Clone for TraitMockReceived<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    pub struct TraitMock<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > {
        pub setup: TraitMockSetup<Trait_CONST, Trait_InputType, Trait_OutputType>,
        pub received: TraitMockReceived<Trait_CONST, Trait_InputType, Trait_OutputType>,
        pub data: Arc<TraitMockData<Trait_CONST, Trait_InputType, Trait_OutputType>>,
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > AsRef<TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>>
        for TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn as_ref(&self) -> &TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> {
            self
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > Clone for TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        fn clone(&self) -> Self {
            Self {
                setup: (&self.setup).clone(),
                received: (&self.received).clone(),
                data: (&self.data).clone(),
            }
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > Trait for TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        const CONST: usize = Trait_CONST;
        type InputType<TAmogus: Clone>
            = Trait_InputType
        where
            Self: Clone;

        type OutputType<TT>
            = Trait_OutputType
        where
            Self: Sized,
            TT: Clone;

        fn get_const(&self) -> usize {
            let call: Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType> =
                Trait_get_const_Call {
                    _phantom_GenericParam_Trait_InputType: PhantomData,
                    _phantom_GenericParam_Trait_OutputType: PhantomData,
                };
            return self.data.clone().Trait_get_const.handle_base_returning(
                self,
                call,
                Self::base_get_const,
            );
        }

        fn get_my_type<TT: Clone>(
            &self,
            input: <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<
                i32,
            >,
        ) -> <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::OutputType<TT>
        where
            Self: Clone + Sized,
            TT: ToString,
        {
            let call: Trait_get_my_type_Call<Trait_CONST, Trait_InputType, Trait_OutputType, TT> =
                Trait_get_my_type_Call {
                    _phantom_GenericParam_Trait_InputType: PhantomData,
                    _phantom_GenericParam_Trait_OutputType: PhantomData,
                    _phantom_input: PhantomData,
                    _phantom_GenericParam_TT: PhantomData,
                    input: transmute_lifetime!(input),
                };
            return self
                .data
                .clone()
                .Trait_get_my_type
                .handle_returning(self, call);
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        pub fn new() -> Self {
            let data = Arc::new(TraitMockData {
                _phantom_GenericParam_Trait_InputType: PhantomData,
                _phantom_GenericParam_Trait_OutputType: PhantomData,
                Trait_get_const: FnData::new("Trait::get_const"),
                Trait_get_my_type: FnData::new("Trait::get_my_type"),
            });
            return TraitMock {
                setup: TraitMockSetup { data: data.clone() },
                received: TraitMockReceived { data: data.clone() },
                data,
            };
        }
        fn base_get_const(
            &self,
            call: Trait_get_const_Call<Trait_CONST, Trait_InputType, Trait_OutputType>,
        ) -> usize {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let Trait_get_const_Call::<Trait_CONST, Trait_InputType, Trait_OutputType> { .. } =
                call;
            <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::CONST
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > TraitMockSetup<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        pub fn get_const<'__rsa>(
            &self,
        ) -> FnTuner<
            '_,
            TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>,
            Self,
            (),
            usize,
            &Self,
            true,
            false,
        > {
            let Trait_get_const_args_checker: Trait_get_const_ArgsChecker<
                Trait_CONST,
                Trait_InputType,
                Trait_OutputType,
            > = Trait_get_const_ArgsChecker {
                _phantom_GenericParam_Trait_InputType: PhantomData,
                _phantom_GenericParam_Trait_OutputType: PhantomData,
            };
            let fn_tuner: FnTuner<
                '_,
                TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>,
                Self,
                (),
                usize,
                &Self,
                true,
                false,
            > = self
                .data
                .Trait_get_const
                .add_config(Trait_get_const_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
        pub fn get_my_type<'__rsa, TT: Clone>(&self, input: impl Into<Arg<<TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>>>) -> FnTuner<'_, TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>, Self, (&'__rsa <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,), <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::OutputType<TT>
            , &Self, false, false>
        where
            Self: Clone + Sized,
            TT: ToString
        {
            let Trait_get_my_type_args_checker: Trait_get_my_type_ArgsChecker<
                Trait_CONST,
                Trait_InputType,
                Trait_OutputType,
                TT,
            > = Trait_get_my_type_ArgsChecker {
                _phantom_GenericParam_Trait_InputType: PhantomData,
                _phantom_GenericParam_Trait_OutputType: PhantomData,
                _phantom_input: PhantomData,
                _phantom_GenericParam_TT: PhantomData,
                input: transmute_lifetime!(input.into()),
            };
            let fn_tuner: FnTuner<'_, TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType>, Self, (&'__rsa <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,), <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::OutputType<TT>
                , &Self, false, false> = self.data.Trait_get_my_type.add_config(Trait_get_my_type_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<
        const Trait_CONST: usize,
        Trait_InputType: Clone + Debug,
        Trait_OutputType: Clone + Sized + Default,
    > TraitMockReceived<Trait_CONST, Trait_InputType, Trait_OutputType>
    {
        pub fn get_const<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let Trait_get_const_args_checker: Trait_get_const_ArgsChecker<
                Trait_CONST,
                Trait_InputType,
                Trait_OutputType,
            > = Trait_get_const_ArgsChecker {
                _phantom_GenericParam_Trait_InputType: PhantomData,
                _phantom_GenericParam_Trait_OutputType: PhantomData,
            };
            self.data
                .Trait_get_const
                .verify_received(Trait_get_const_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn get_my_type<'__rsa, TT: Clone>(&self, input: impl Into<Arg<<TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>>>, times: Times) -> FnVerifier<Self, (&'__rsa <TraitMock<Trait_CONST, Trait_InputType, Trait_OutputType> as Trait>::InputType<i32>,)>
        where
            Self: Clone + Sized,
            TT: ToString
        {
            let Trait_get_my_type_args_checker: Trait_get_my_type_ArgsChecker<
                Trait_CONST,
                Trait_InputType,
                Trait_OutputType,
                TT,
            > = Trait_get_my_type_ArgsChecker {
                _phantom_GenericParam_Trait_InputType: PhantomData,
                _phantom_GenericParam_Trait_OutputType: PhantomData,
                _phantom_input: PhantomData,
                _phantom_GenericParam_TT: PhantomData,
                input: transmute_lifetime!(input.into()),
            };
            self.data
                .Trait_get_my_type
                .verify_received(Trait_get_my_type_args_checker, times);
            return FnVerifier::new(self.clone());
        }
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
