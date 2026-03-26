use std::marker::PhantomData;

trait Same<'a, T> {
    fn work(&self, t: &'a T);
}

trait Different<'a, T> {
    fn work(&self, t: &'a T);
}

// mocked_base! {
//     struct Struct<'a, T>(PhantomData<&'a T>);
//
//     impl<'a, T> Struct<'a, T> {
//         pub fn new() -> Self { Self(PhantomData) }
//     }
//
//     // TODO - write in doc that this is not supported, that generic params idents in impl blocks should be same as in item definition
//     // impl<'b, S> Struct<'b, S> {
//     //     pub fn new() -> Self {
//     //         Self(PhantomData)
//     //     }
//     //
//     //     pub fn work(&self, s: &'b S) {}
//     // }
//
//     // impl<'a, T> Same<'a, T> for Struct<'a, T> {
//     //     fn work(&self, t: &'a T) {
//     //         unreachable!()
//     //     }
//     // }
//
//     impl<'a, 'd, T, Td> Different<'d, Td> for Struct<'a, T> {
//         fn work(&self, t: &'d Td) {
//             unreachable!()
//         }
//     }
// }

mod tests {
    #[test]
    fn compile() {}
}

#[cfg(not(test))]
struct Struct<'a, T>(PhantomData<&'a T>);
#[cfg(not(test))]
impl<'a, 'd, T, Td> Different<'d, Td> for Struct<'a, T> {
    fn work(&self, t: &'d Td) {
        unreachable!()
    }
}
#[cfg(not(test))]
impl<'a, T> Struct<'a, T> {
    pub fn new() -> Self {
        Self(PhantomData)
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
    pub struct Different_work_Call<'a, 'a, 'd, T, T, Td> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_T: PhantomData<T>,
        _phantom_t: PhantomData<&'d Td>,
        t: *const Td,
    }
    impl<'a, 'a, 'd, T, T, Td> IArgsInfosProvider for Different_work_Call<'a, 'a, 'd, T, T, Td> {
        fn get_arg_infos(&self) -> Vec<ArgInfo> {
            vec![ArgInfo::new(
                "t",
                &self.t,
                (&ArgPrinter::<&Td>(transmute_lifetime!(&self.t))).debug_string(),
            )]
        }
    }
    impl<'a, 'a, 'd, T, T, Td> IArgsTupleProvider for Different_work_Call<'a, 'a, 'd, T, T, Td> {
        fn get_ptr_to_boxed_tuple_of_refs(&self) -> *mut () {
            Box::leak(Box::new((&self.t,))) as *mut _ as *mut ()
        }
    }
    impl<'a, 'a, 'd, T, T, Td> IGenericsInfoProvider for Different_work_Call<'a, 'a, 'd, T, T, Td> {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![
                generic_type_info("T", core::any::type_name::<T>()),
                generic_type_info("Td", core::any::type_name::<Td>()),
            ]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<T>(), tid::<Td>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    impl<'a, 'a, 'd, T, T, Td> Clone for Different_work_Call<'a, 'a, 'd, T, T, Td> {
        fn clone(&self) -> Self {
            Self {
                _phantom_GenericParam_a: (&self._phantom_GenericParam_a).clone(),
                _phantom_GenericParam_T: (&self._phantom_GenericParam_T).clone(),
                _phantom_t: (&self._phantom_t).clone(),
                t: (&self.t).clone(),
            }
        }
    }
    #[doc(hidden)]
    #[derive(Debug)]
    pub struct Different_work_ArgsChecker<'a, 'a, 'd, T, T, Td> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_T: PhantomData<T>,
        _phantom_t: PhantomData<&'d Td>,
        t: Arg<*const Td>,
    }
    impl<'a, 'a, 'd, T, T, Td> IArgsChecker for Different_work_ArgsChecker<'a, 'a, 'd, T, T, Td> {
        #[allow(unused)]
        fn check(&self, dyn_call: &DynCall) -> Vec<ArgCheckResult> {
            let call: &Different_work_Call<'a, 'a, 'd, T, T, Td> = dyn_call.downcast_ref();
            vec![transmute_lifetime!(&self.t, &Arg::<&'d Td>).check_ref(
                "t",
                transmute_lifetime!(&call.t),
                (&ArgPrinter::<&Td>(transmute_lifetime!(&call.t))).debug_string(),
            )]
        }
    }
    impl<'a, 'a, 'd, T, T, Td> IArgsFormatter for Different_work_ArgsChecker<'a, 'a, 'd, T, T, Td> {
        fn fmt_args(&self) -> String {
            format!(
                "{}",
                (&ArgPrinter(&transmute_lifetime!(&self.t, &Arg::<&'d Td>))).debug_string()
            )
        }
    }
    impl<'a, 'a, 'd, T, T, Td> IGenericsInfoProvider
        for Different_work_ArgsChecker<'a, 'a, 'd, T, T, Td>
    {
        fn get_generic_parameter_infos(&self) -> Vec<GenericParameterInfo> {
            vec![
                generic_type_info("T", core::any::type_name::<T>()),
                generic_type_info("Td", core::any::type_name::<Td>()),
            ]
        }
        fn hash_generics_type_ids(&self, hasher: &mut GenericsHasher) {
            [tid::<T>(), tid::<Td>()].hash(hasher)
        }
        fn hash_const_values(&self, hasher: &mut GenericsHasher) {}
    }
    #[doc(hidden)]
    pub struct DifferentSetup<'a, T> {
        data: Arc<StructData<'a, T>>,
    }
    impl<'a, T> Clone for DifferentSetup<'a, T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct DifferentReceived<'a, T> {
        data: Arc<StructData<'a, T>>,
    }
    impl<'a, T> Clone for DifferentReceived<'a, T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
            }
        }
    }
    impl<'a, T> DifferentSetup<'a, T> {
        pub fn work<'__rsa>(
            &self,
            t: impl Into<Arg<&'d Td>>,
        ) -> FnTuner<'_, Struct<'a, T>, Self, (&'__rsa &'d Td,), (), &Self, true, true>
        where
            '__rsa: 'a,
            'a: '__rsa,
        {
            let Different_work_args_checker: Different_work_ArgsChecker<'a, 'a, 'd, T, T, Td> =
                Different_work_ArgsChecker {
                    _phantom_GenericParam_a: PhantomData,
                    _phantom_GenericParam_T: PhantomData,
                    _phantom_t: PhantomData,
                    t: transmute_lifetime!(t.into()),
                };
            let fn_tuner: FnTuner<
                '_,
                Struct<'a, T>,
                Self,
                (&'__rsa &'d Td,),
                (),
                &Self,
                true,
                true,
            > = self
                .data
                .Different_work
                .add_config(Different_work_args_checker, self);
            return transmute_lifetime!(fn_tuner);
        }
    }
    impl<'a, T> DifferentReceived<'a, T> {
        pub fn work<'__rsa>(
            &self,
            t: impl Into<Arg<&'d Td>>,
            times: Times,
        ) -> FnVerifier<Self, (&'__rsa &'d Td,)>
        where
            '__rsa: 'a,
            'a: '__rsa,
        {
            let Different_work_args_checker: Different_work_ArgsChecker<'a, 'a, 'd, T, T, Td> =
                Different_work_ArgsChecker {
                    _phantom_GenericParam_a: PhantomData,
                    _phantom_GenericParam_T: PhantomData,
                    _phantom_t: PhantomData,
                    t: transmute_lifetime!(t.into()),
                };
            self.data
                .Different_work
                .verify_received(Different_work_args_checker, times);
            return FnVerifier::new(self.clone());
        }
    }
    #[doc(hidden)]
    #[derive(IMockData)]
    pub struct StructData<'a, T> {
        _phantom_GenericParam_a: PhantomData<&'a ()>,
        _phantom_GenericParam_T: PhantomData<T>,
        pub Different_work: FnData<'static, Struct<'a, T>, true, true>,
    }
    #[doc(hidden)]
    pub struct StructSetup<'a, T> {
        data: Arc<StructData<'a, T>>,
        pub as_Different: DifferentSetup<'a, T>,
    }
    impl<'a, T> Clone for StructSetup<'a, T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
                as_Different: (&self.as_Different).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct StructReceived<'a, T> {
        data: Arc<StructData<'a, T>>,
        pub as_Different: DifferentReceived<'a, T>,
    }
    impl<'a, T> Clone for StructReceived<'a, T> {
        fn clone(&self) -> Self {
            Self {
                data: (&self.data).clone(),
                as_Different: (&self.as_Different).clone(),
            }
        }
    }
    #[doc(hidden)]
    pub struct Struct_InnerData<'a, T>(PhantomData<&'a T>);

    impl<'a, T> Struct_InnerData<'a, T> {
        pub fn new() -> Self {
            Self(PhantomData)
        }
    }
    pub struct Struct<'a, T> {
        pub setup: StructSetup<'a, T>,
        pub received: StructReceived<'a, T>,
        pub data: Arc<StructData<'a, T>>,
        inner_data: Struct_InnerData<'a, T>,
    }
    impl<'a, T> AsRef<Struct<'a, T>> for Struct<'a, T> {
        fn as_ref(&self) -> &Struct<'a, T> {
            self
        }
    }
    impl<'a, T> Deref for Struct<'a, T> {
        type Target = Struct_InnerData<'a, T>;
        fn deref(&self) -> &Self::Target {
            &self.inner_data
        }
    }
    impl<'a, T> Different<'d, Td> for Struct<'a, T> {
        fn work(&self, t: &'d Td) {
            let call: Different_work_Call<'_, '_, '_, T, T, Td> = Different_work_Call {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_T: PhantomData,
                _phantom_t: PhantomData,
                t: transmute_lifetime!(t),
            };
            self.data
                .clone()
                .Different_work
                .handle_base(self, call, Self::base_Different_work);
        }
    }
    impl<'a, T> Struct<'a, T> {}
    impl<'a, T> Struct<'a, T> {
        pub fn new() -> Self {
            let data = Arc::new(StructData {
                _phantom_GenericParam_a: PhantomData,
                _phantom_GenericParam_T: PhantomData,
                Different_work: FnData::new("Different::work"),
            });
            let inner_data = Struct_InnerData::new();
            return Struct {
                setup: StructSetup {
                    data: data.clone(),
                    as_Different: DifferentSetup { data: data.clone() },
                },
                received: StructReceived {
                    data: data.clone(),
                    as_Different: DifferentReceived { data: data.clone() },
                },
                data,
                inner_data,
            };
        }
        fn base_Different_work(&self, call: Different_work_Call<'a, 'a, 'd, T, T, Td>) {
            #[allow(non_shorthand_field_patterns)]
            #[allow(unused_variables)]
            let Different_work_Call::<'_, '_, '_, T, T, Td> { t: t, .. } = call;
            let t: &'d Td = transmute_lifetime!(t);
            unreachable!()
        }
    }
    impl<'a, T> StructSetup<'a, T> {}
    impl<'a, T> StructReceived<'a, T> {
        pub fn no_other_calls(&self) {
            self.data.verify_received_nothing_else();
        }
    }
}
