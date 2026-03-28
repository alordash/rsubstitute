use std::marker::PhantomData;
use rsubstitute_proc_macro::mocked_base;

trait Trait {
    fn work(&self);
}

// mocked_base! {
//     struct Struct<T>(PhantomData<T>);
//
//     impl<T> Struct<T> {
//         pub fn new() -> Self {Self::new()}
//     }
//
//     impl Trait for Struct<i32> {
//         fn work(&self) {
//             unreachable!()
//         }
//     }
//
//     impl Trait for Struct<bool> {
//         fn work(&self) {
//             unreachable!()
//         }
//     }
// }

fn main() {
    println!("Done");
}
#[cfg(not(test))]
struct Struct<T>(PhantomData<T>);
#[cfg(not(test))]
impl Trait for Struct<i32> {
    fn work(&self) {
        unreachable!()
    }
}
#[cfg(not(test))]
impl Trait for Struct<bool> {
    fn work(&self) {
        unreachable!()
    }
}
#[cfg(not(test))]
impl<T> Struct<T> {
    pub fn new() -> Self {
        Self::new()
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
    pub struct Trait_Struct_i32_work_Call;
    impl IArgsInfosProvider for Trait_Struct_i32_work_Call {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl IArgsTupleProvider for Trait_Struct_i32_work_Call {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl IGenericsInfoProvider for Trait_Struct_i32_work_Call {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl Clone for Trait_Struct_i32_work_Call {
        fn clone(&self) -> Self {
            Self
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_Struct_i32_work_ArgsChecker;
    impl IArgsChecker for Trait_Struct_i32_work_ArgsChecker {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_Struct_i32_work_Call = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl IArgsFormatter for Trait_Struct_i32_work_ArgsChecker {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl IGenericsInfoProvider for Trait_Struct_i32_work_ArgsChecker {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    pub struct Trait_Struct_i32Data {
        pub Trait_Struct_i32_work: FnData<'static, Struct<i32>, true, true>
    }
    #[doc(hidden)]
    pub struct Trait_Struct_i32Setup {
        data: Arc<Trait_Struct_i32Data>,
    }
    impl Clone for Trait_Struct_i32Setup {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct Trait_Struct_i32Received {
        data: Arc<Trait_Struct_i32Data>,
    }
    impl Clone for Trait_Struct_i32Received {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    impl Trait_Struct_i32Setup {
        pub fn work<'__rsa>(&self) -> FnTuner<'_, Struct<i32>, Self, (), (), &Struct<i32>, true, true> {
            let Trait_Struct_i32_work_args_checker: Trait_Struct_i32_work_ArgsChecker = Trait_Struct_i32_work_ArgsChecker;
            let fn_tuner: FnTuner<'_, Struct<i32>, Self, (), (), &Struct<i32>, true, true> = self
                .data
                .Trait_work
                .add_config(Trait_work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<T> TraitReceived<T> {
        pub fn work<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let Trait_work_args_checker: Trait_work_ArgsChecker<T> = Trait_work_ArgsChecker {
                _phantom_GenericParam_T: PhantomData,
            };
            self.data
                .Trait_work
                .verify_received(Trait_work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    pub struct Trait_work_Call<T> {
        _phantom_GenericParam_T: PhantomData<T>,
    }
    impl<T> IArgsInfosProvider for Trait_work_Call<T> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![]
        }
    }
    impl<T> IArgsTupleProvider for Trait_work_Call<T> {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new(())) as *mut _ as *mut ()
        }
    }
    impl<T> IGenericsInfoProvider for Trait_work_Call<T> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl<T> Clone for Trait_work_Call<T> {
        fn clone(&self) -> Self {
            Self {
                _phantom_GenericParam_T: (&self._phantom_GenericParam_T).clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Trait_work_ArgsChecker<T> {
        _phantom_GenericParam_T: PhantomData<T>,
    }
    impl<T> IArgsChecker for Trait_work_ArgsChecker<T> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Trait_work_Call<T> = dyn_call.downcast_ref();
            vec![]
        }
    }
    impl<T> IArgsFormatter for Trait_work_ArgsChecker<T> {
        fn fmt_args(&self) -> String {
            format!("",)
        }
    }
    impl<T> IGenericsInfoProvider for Trait_work_ArgsChecker<T> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {}
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct TraitSetup<T> {
        data: Arc<StructData<T>>,
    }
    impl<T> Clone for TraitSetup<T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct TraitReceived<T> {
        data: Arc<StructData<T>>,
    }
    impl<T> Clone for TraitReceived<T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    impl<T> TraitSetup<T> {
        pub fn work<'__rsa>(&self) -> FnTuner<'_, Struct<T>, Self, (), (), &Struct<T>, true, true> {
            let Trait_work_args_checker: Trait_work_ArgsChecker<T> = Trait_work_ArgsChecker {
                _phantom_GenericParam_T: PhantomData,
            };
            let fn_tuner: FnTuner<'_, Struct<T>, Self, (), (), &Struct<T>, true, true> = self
                .data
                .Trait_work
                .add_config(Trait_work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<T> TraitReceived<T> {
        pub fn work<'__rsa>(&self, times: Times) -> FnVerifier<Self, ()> {
            let Trait_work_args_checker: Trait_work_ArgsChecker<T> = Trait_work_ArgsChecker {
                _phantom_GenericParam_T: PhantomData,
            };
            self.data
                .Trait_work
                .verify_received(Trait_work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData<T> {
        _phantom_GenericParam_T: PhantomData<T>,
        pub Trait_work: FnData<'static, Struct<T>, true, true>,
        pub Trait_work: FnData<'static, Struct<T>, true, true>,
    }
    #[doc(hidden)]
    pub struct StructSetup<T> {
        data: Arc<StructData<T>>,
        pub as_Trait: TraitSetup<T>,
        pub as_Trait: TraitSetup<T>,
    }
    impl<T> Clone for StructSetup<T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
                as_Trait: (&self.as_Trait).clone(),
                as_Trait: (&self.as_Trait).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct StructReceived<T> {
        data: Arc<StructData<T>>,
        pub as_Trait: TraitReceived<T>,
        pub as_Trait: TraitReceived<T>,
    }
    impl<T> Clone for StructReceived<T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
                as_Trait: (&self.as_Trait).clone(),
                as_Trait: (&self.as_Trait).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct Struct_InnerData<T>(PhantomData<T>);

    impl<T> Struct_InnerData<T> {
        pub fn new() -> Self {
            Self::new()
        }
    }
    pub struct Struct<T> {
        pub setup: StructSetup<T>,
        pub received: StructReceived<T>,
        pub data: Arc<StructData<T>>,
        inner_data: Struct_InnerData<T>,
    }
    impl<T> AsRef<Struct<T>> for Struct<T> {
        fn as_ref(&self) -> &Struct<T> {
            self
        }
    }
    impl<T> Deref for Struct<T> {
        type Target = Struct_InnerData<T>;
        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl Trait for Struct<i32> {
        fn work(&self) {
            let call: Trait_work_Call<T> = Trait_work_Call {
                _phantom_GenericParam_T: PhantomData,
            };
            self.data
                .clone()
                .Trait_work
                .handle_base(self, call, Self::base_Trait_work);
        }
    }

    impl Trait for Struct<bool> {
        fn work(&self) {
            let call: Trait_work_Call<T> = Trait_work_Call {
                _phantom_GenericParam_T: PhantomData,
            };
            self.data
                .clone()
                .Trait_work
                .handle_base(self, call, Self::base_Trait_work);
        }
    }
    impl<T> Struct<T> {}
    impl<T> Struct<T> {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                _phantom_GenericParam_T: PhantomData,
                Trait_work: FnData::new("Trait::work"),
                Trait_work: FnData::new("Trait::work"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup {
                    data: data.clone(),
                    as_Trait: TraitSetup { data: data.clone() },
                    as_Trait: TraitSetup { data: data.clone() },
                },
                received: StructReceived {
                    data: data.clone(),
                    as_Trait: TraitReceived { data: data.clone() },
                    as_Trait: TraitReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
        fn base_Trait_work(self: &Struct<T>, call: Trait_work_Call<T>) {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let Trait_work_Call::<T> { .. } = call;
            unreachable!()
        }
        fn base_Trait_work(self: &Struct<T>, call: Trait_work_Call<T>) {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let Trait_work_Call::<T> { .. } = call;
            unreachable!()
        }
    }
    impl<T> StructSetup<T> {}
    impl<T> StructReceived<T> {
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
